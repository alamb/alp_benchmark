#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.10"
# dependencies = [
#     "matplotlib>=3.8",
# ]
# ///
"""Generate benchmark diagrams from a report markdown file.

Usage:
    ./diagrams.py reports/kosta.md      # writes PNGs to diagrams/kosta/

Parses the "Benchmark environment" and "Parquet compression results" tables
produced by benchmark.sh and renders:

  per dataset (all encodings):   compression_speed.png, decompression_speed.png, density.png
  per dataset (ZSTD vs ALP):     compression_speed_zstd_vs_alp.png, ...
  averages over all datasets:    avg_compression_speed.png, avg_decompression_speed.png, avg_density.png

If the report has a "Random access" table, also renders random_access.png
(rows decoded per second, one bar per encoding).

Compressed size is plotted as density = 1024 / (bits/value), so higher is
better on every chart. Average charts get a broken y-axis when the tallest
bar dwarfs the runner-up (>3x); the per-dataset density charts cap the y-axis
because near-constant datasets (gov26/31/40) reach density ~5000.
"""
import re
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib import gridspec
from matplotlib.patches import Patch
from matplotlib.ticker import EngFormatter

CHOICES = ["PLAIN", "PLAIN + ZSTD", "ALP"]
COLORS = {"PLAIN": "#2a78d6", "PLAIN + ZSTD": "#eb6834", "ALP": "#1baf7a"}
INK, MUTED, GRID, BASE = "#0b0b0b", "#898781", "#e1e0d9", "#c3c2b7"
AVG_KEY = "ALL AVG."
DENSITY_CAP = 150  # per-dataset density cap; gov26/31/40 reach ~5000
SPEED_Y = "GB/s (higher is better)"
DENSITY_Y = "values per 1024 bits (higher is better)"
ROWS_Y = "rows per second (higher is better)"

plt.rcParams.update({
    "font.family": "sans-serif",
    "font.sans-serif": ["Helvetica Neue", "Arial", "DejaVu Sans"],
    "figure.facecolor": "white",
    "axes.facecolor": "white",
    "text.color": INK,
    "axes.edgecolor": BASE,
    "axes.labelcolor": "#52514e",
    "xtick.color": "#52514e",
    "ytick.color": MUTED,
})


def parse_report(path: Path):
    """Return (cpu, datasets, table) where table[dataset][choice] =
    (compression GB/s, decompression GB/s, density values/1024 bits)."""
    text = path.read_text()

    cpu = ""
    m = re.search(r"^\|\s*CPU\s*\|\s*(.+?)\s*\|\s*$", text, re.M)
    if m:
        cpu = m.group(1).strip("`")

    table: dict[str, dict[str, tuple]] = {}
    datasets: list[str] = []
    section = text.split("## Parquet compression results", 1)[-1]
    for line in section.splitlines():
        cells = [c.strip().strip("*").strip("`").strip() for c in line.strip().strip("|").split("|")]
        if len(cells) != 5 or cells[1] not in CHOICES:
            continue
        dataset, choice = cells[0], cells[1]
        comp, decomp, bits = (float(c) for c in cells[2:])
        if dataset not in table:
            table[dataset] = {}
            if dataset != AVG_KEY:
                datasets.append(dataset)
        table[dataset][choice] = (comp, decomp, 1024.0 / bits)

    missing = [d for d in [*datasets, AVG_KEY] if set(table.get(d, {})) != set(CHOICES)]
    if not datasets or missing:
        sys.exit(f"error: could not parse compression results from {path} (missing: {missing})")
    return cpu, datasets, table


def parse_random_access(path: Path):
    """Return (n_rows, dataset, {choice: rows per second}) from the
    "Random access" table, or None if the report has no such table."""
    text = path.read_text()
    if "## Random access" not in text:
        return None
    section = text.split("## Random access", 1)[-1]

    m = re.search(r"\|\s*(\d+)\s+random rows\s*\(µs\)\s*\|", section)
    n_rows = int(m.group(1)) if m else 100
    m = re.search(r"rows from\s*`([^`]+)`", section)
    dataset = m.group(1) if m else ""

    rows_per_sec = {}
    for line in section.splitlines():
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cells) == 2 and cells[0] in CHOICES:
            rows_per_sec[cells[0]] = n_rows / (float(cells[1]) * 1e-6)
    if set(rows_per_sec) != set(CHOICES):
        print(f"warning: skipping random access chart, incomplete table in {path}", file=sys.stderr)
        return None
    return n_rows, dataset, rows_per_sec


def human(v):
    if v >= 1e6:
        return f"{v / 1e6:,.1f}M"
    if v >= 1e3:
        return f"{v / 1e3:,.1f}K"
    return f"{v:,.1f}"


def style_axis(ax):
    ax.spines["top"].set_visible(False)
    ax.spines["right"].set_visible(False)
    ax.yaxis.grid(True, color=GRID, linewidth=0.8)
    ax.set_axisbelow(True)
    ax.tick_params(axis="x", length=0)
    ax.tick_params(axis="y", length=0)


def add_titles(fig, title, subtitle):
    # wrap long subtitles (e.g. long CPU names) at a separator on narrow figures
    if len(subtitle) > 8 * fig.get_figwidth() and " · " in subtitle:
        head, _, tail = subtitle.rpartition(" · ")
        subtitle = f"{head}\n{tail}"
    fig.text(0.02, 0.97, title, ha="left", va="top", fontsize=13, fontweight="bold")
    fig.text(0.02, 0.925, subtitle, ha="left", va="top", fontsize=9.5, color=MUTED, linespacing=1.4)


def label_bar(ax, x, v, lo, hi, fmt=lambda v: f"{v:,.1f}"):
    if lo <= v <= hi:
        ax.annotate(fmt(v), (x, v), xytext=(0, 4), textcoords="offset points",
                    ha="center", va="bottom", fontsize=11, fontweight="bold", color=INK)


def per_dataset_chart(path, datasets, table, metric, choices, title, subtitle, y_label, cap=None):
    fig, ax = plt.subplots(figsize=(14, 5), dpi=200)
    n = len(choices)
    width = 0.8 / n
    for j, choice in enumerate(choices):
        xs = [i + (j - (n - 1) / 2) * width for i in range(len(datasets))]
        ax.bar(xs, [table[d][choice][metric] for d in datasets],
               width=width * 0.92, color=COLORS[choice], zorder=3)
    style_axis(ax)
    if cap is not None:
        ax.set_ylim(0, cap)
    ax.set_xlim(-0.6, len(datasets) - 0.4)
    ax.set_xticks(range(len(datasets)), datasets, rotation=45, ha="right", fontsize=8)
    ax.set_ylabel(y_label, fontsize=10)
    ax.legend(handles=[Patch(color=COLORS[c], label=c) for c in choices],
              loc="lower right", bbox_to_anchor=(1, 1.0), ncol=len(choices),
              frameon=False, fontsize=9, borderaxespad=0)
    add_titles(fig, title, subtitle)
    fig.tight_layout(rect=(0, 0, 1, 0.90))
    fig.savefig(path, facecolor="white")
    plt.close(fig)


def avg_plain_chart(path, vals, y_label, title, subtitle, fmt=lambda v: f"{v:,.1f}", y_fmt=None):
    fig, ax = plt.subplots(figsize=(5.4, 4.2), dpi=200)
    ax.bar(range(len(vals)), vals, width=0.55, color=[COLORS[c] for c in CHOICES], zorder=3)
    style_axis(ax)
    top = max(vals) * 1.18
    ax.set_ylim(0, top)
    if y_fmt is not None:
        ax.yaxis.set_major_formatter(y_fmt)
    for x, v in enumerate(vals):
        label_bar(ax, x, v, 0, top, fmt)
    ax.set_xticks(range(len(vals)), CHOICES, fontsize=11)
    ax.set_ylabel(y_label, fontsize=10)
    add_titles(fig, title, subtitle)
    fig.tight_layout(rect=(0, 0, 1, 0.86))
    fig.savefig(path, facecolor="white")
    plt.close(fig)


def avg_broken_chart(path, vals, y_label, title, subtitle, fmt=lambda v: f"{v:,.1f}", y_fmt=None):
    """Two panels sharing x: top shows only the outlier's range, bottom the rest."""
    second = sorted(vals)[-2]
    outlier = max(vals)

    fig = plt.figure(figsize=(5.4, 4.6), dpi=200)
    gs = gridspec.GridSpec(2, 1, height_ratios=[1, 2.6], hspace=0.08)
    ax_t = fig.add_subplot(gs[0])
    ax_b = fig.add_subplot(gs[1], sharex=ax_t)

    for ax in (ax_t, ax_b):
        ax.bar(range(len(vals)), vals, width=0.55, color=[COLORS[c] for c in CHOICES], zorder=3)
        style_axis(ax)
        if y_fmt is not None:
            ax.yaxis.set_major_formatter(y_fmt)
    ax_t.set_ylim(outlier * 0.93, outlier * 1.12)
    ax_b.set_ylim(0, second * 1.35)

    ax_t.spines["bottom"].set_visible(False)
    ax_b.spines["top"].set_visible(False)
    ax_t.tick_params(labelbottom=False)

    # diagonal break marks on the y-spine
    kw = dict(marker=[(-1, -0.5), (1, 0.5)], markersize=10, linestyle="none",
              color=BASE, mec=BASE, mew=1.2, clip_on=False)
    ax_t.plot([0], [0], transform=ax_t.transAxes, **kw)
    ax_b.plot([0], [1], transform=ax_b.transAxes, **kw)

    for x, v in enumerate(vals):
        label_bar(ax_b, x, v, 0, second * 1.35, fmt)
        label_bar(ax_t, x, v, outlier * 0.93, outlier * 1.12, fmt)

    ax_b.set_xticks(range(len(vals)), CHOICES, fontsize=11)
    fig.supylabel(y_label, fontsize=10, x=0.02)
    add_titles(fig, title, subtitle)
    gs.tight_layout(fig, rect=(0.03, 0, 1, 0.86))
    fig.savefig(path, facecolor="white")
    plt.close(fig)


def main():
    if len(sys.argv) != 2:
        sys.exit(f"usage: {sys.argv[0]} reports/<name>.md")
    report = Path(sys.argv[1])
    if not report.is_file():
        sys.exit(f"error: {report} is not a file")
    out = Path("diagrams") / report.stem
    out.mkdir(parents=True, exist_ok=True)

    cpu, datasets, table = parse_report(report)
    subtitle = f"{len(datasets)} datasets · {cpu}" if cpu else f"{len(datasets)} datasets"
    avg_subtitle = f"Arithmetic mean over {len(datasets)} datasets · {cpu}".rstrip(" ·")

    metrics = [
        (0, "compression_speed", "Compression speed by dataset", "Average compression speed", SPEED_Y, None),
        (1, "decompression_speed", "Decompression speed by dataset", "Average decompression speed", SPEED_Y, None),
        (2, "density", "Compression density by dataset", "Average compression density", DENSITY_Y, DENSITY_CAP),
    ]
    for metric, stem, ds_title, avg_title, y_label, cap in metrics:
        cap_note = f" — y-axis capped at {cap}" if cap else ""
        per_dataset_chart(out / f"{stem}.png", datasets, table, metric, CHOICES,
                          ds_title + cap_note, subtitle, y_label, cap)
        per_dataset_chart(out / f"{stem}_zstd_vs_alp.png", datasets, table, metric,
                          CHOICES[1:], ds_title + " (ZSTD vs ALP)" + cap_note, subtitle, y_label, cap)

        vals = [table[AVG_KEY][c][metric] for c in CHOICES]
        avg_path = out / f"avg_{stem}.png"
        if max(vals) > 3 * sorted(vals)[-2]:
            avg_broken_chart(avg_path, vals, y_label, avg_title, avg_subtitle)
        else:
            avg_plain_chart(avg_path, vals, y_label, avg_title, avg_subtitle)

    random_access = parse_random_access(report)
    if random_access is not None:
        n_rows, dataset, rows_per_sec = random_access
        vals = [rows_per_sec[c] for c in CHOICES]
        ra_subtitle = f"Decoding {n_rows} random rows" + (f" from {dataset}" if dataset else "")
        ra_subtitle += f" · {cpu}" if cpu else ""
        kwargs = dict(fmt=human, y_fmt=EngFormatter(places=0, sep=""))
        ra_path = out / "random_access.png"
        if max(vals) > 3 * sorted(vals)[-2]:
            avg_broken_chart(ra_path, vals, ROWS_Y, "Random access speed", ra_subtitle, **kwargs)
        else:
            avg_plain_chart(ra_path, vals, ROWS_Y, "Random access speed", ra_subtitle, **kwargs)

    for p in sorted(out.iterdir()):
        print(p)


if __name__ == "__main__":
    main()
