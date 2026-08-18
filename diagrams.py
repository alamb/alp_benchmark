#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.10"
# dependencies = [
#     "matplotlib>=3.8",
# ]
# ///
"""Generate benchmark diagrams from one or more report markdown files.

Usage:
    ./diagrams.py reports/kosta.md [reports/alamb.md ...]

Each report is one run of the same benchmark on a different machine. Parses
the "Benchmark environment" and "Parquet compression results" tables produced
by benchmark.sh and renders, per report (into diagrams/<stem>/):

  per dataset (all encodings):   compression_speed.png, decompression_speed.png, compression_ratio.png
  per dataset (ZSTD vs ALP):     compression_speed_zstd_vs_alp.png, ...

and, combined across ALL reports (into diagrams/): avg_compression_speed.png,
avg_decompression_speed.png, avg_compression_ratio.png, and — for reports
with a "Random access" table — avg_random_access.png. Each combined chart has
one bar group per machine, three bars (PLAIN, PLAIN + ZSTD, ALP) per group,
to show whether the shape holds across machines.

Compressed size is plotted as compression ratio = uncompressed size /
compressed size = 64 / (bits/value), so higher is better on every chart.
Average charts get a broken y-axis when the tallest bar dwarfs the runner-up
(>3x); the per-dataset ratio charts cap the y-axis because near-constant
datasets (gov26/31/40) reach ratio ~300.
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
RATIO_CAP = 10  # per-dataset compression ratio cap; gov26/31/40 reach ~300
SPEED_Y = "GB/s (higher is better)"
RATIO_Y = "compression ratio (higher is better)"
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
    (compression GB/s, decompression GB/s, compression ratio)."""
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
        table[dataset][choice] = (comp, decomp, 64.0 / bits)

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
    if "\n" not in subtitle and len(subtitle) > 8 * fig.get_figwidth() and " · " in subtitle:
        head, _, tail = subtitle.rpartition(" · ")
        subtitle = f"{head}\n{tail}"
    fig.text(0.02, 0.97, title, ha="left", va="top", fontsize=13, fontweight="bold")
    fig.text(0.02, 0.912, subtitle, ha="left", va="top", fontsize=9.5, color=MUTED, linespacing=1.4)


def label_bar(ax, x, v, lo, hi, fmt=lambda v: f"{v:,.1f}", fontsize=11):
    if lo <= v <= hi:
        ax.annotate(fmt(v), (x, v), xytext=(0, 4), textcoords="offset points",
                    ha="center", va="bottom", fontsize=fontsize, fontweight="bold", color=INK)


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


def grouped_bars(ax, n_groups, vals):
    n = len(CHOICES)
    width = 0.8 / n
    for j, choice in enumerate(CHOICES):
        xs = [i + (j - (n - 1) / 2) * width for i in range(n_groups)]
        ax.bar(xs, [vals[i][j] for i in range(n_groups)],
               width=width * 0.9, color=COLORS[choice], zorder=3)


def bar_positions(n_groups):
    n = len(CHOICES)
    width = 0.8 / n
    return [(i + (j - (n - 1) / 2) * width, i, j) for i in range(n_groups) for j in range(n)]


def choice_legend(ax):
    ax.legend(handles=[Patch(color=COLORS[c], label=c) for c in CHOICES],
              loc="lower right", bbox_to_anchor=(1, 1.0), ncol=len(CHOICES),
              frameon=False, fontsize=9, borderaxespad=0)


def pick_outlier(vals):
    """Return the choice index whose bars dwarf (>3x) every other bar on
    every machine, or None if no choice does."""
    for j in range(len(CHOICES)):
        others = [row[k] for row in vals for k in range(len(CHOICES)) if k != j]
        if min(row[j] for row in vals) > 3 * max(others):
            return j
    return None


def grouped_avg_chart(path, labels, vals, y_label, title, subtitle, fmt=lambda v: f"{v:,.1f}", y_fmt=None):
    """One bar group per machine, one bar per Parquet choice."""
    extra = subtitle.count("\n")
    fig, ax = plt.subplots(figsize=(max(5.4, 2.4 * len(labels) + 1.6), 3.9 + 0.22 * extra), dpi=200)
    grouped_bars(ax, len(labels), vals)
    style_axis(ax)
    if y_fmt is not None:
        ax.yaxis.set_major_formatter(y_fmt)
    top = max(max(row) for row in vals) * 1.18
    ax.set_ylim(0, top)
    ax.set_xlim(-0.6, len(labels) - 0.4)
    for x, i, j in bar_positions(len(labels)):
        label_bar(ax, x, vals[i][j], 0, top, fmt, fontsize=9)
    ax.set_xticks(range(len(labels)), labels, fontsize=11)
    ax.set_ylabel(y_label, fontsize=10)
    choice_legend(ax)
    add_titles(fig, title, subtitle)
    fig.tight_layout(rect=(0, 0, 1, 0.91 - 0.045 * extra))
    fig.savefig(path, facecolor="white")
    plt.close(fig)


def grouped_broken_chart(path, labels, vals, outlier, y_label, title, subtitle, fmt=lambda v: f"{v:,.1f}", y_fmt=None):
    """Two panels sharing x: top shows only the outlier choice's range, bottom the rest."""
    out_vals = [row[outlier] for row in vals]
    rest = [v for row in vals for j, v in enumerate(row) if j != outlier]
    lo_t, hi_t = min(out_vals) * 0.85, max(out_vals) * 1.12
    hi_b = max(rest) * 1.35
    extra = subtitle.count("\n")

    fig = plt.figure(figsize=(max(5.4, 2.4 * len(labels) + 1.6), 4.3 + 0.22 * extra), dpi=200)
    gs = gridspec.GridSpec(2, 1, height_ratios=[1, 2.6], hspace=0.08)
    ax_t = fig.add_subplot(gs[0])
    ax_b = fig.add_subplot(gs[1], sharex=ax_t)

    for ax in (ax_t, ax_b):
        grouped_bars(ax, len(labels), vals)
        style_axis(ax)
        if y_fmt is not None:
            ax.yaxis.set_major_formatter(y_fmt)
    ax_t.set_ylim(lo_t, hi_t)
    ax_b.set_ylim(0, hi_b)
    ax_b.set_xlim(-0.6, len(labels) - 0.4)

    ax_t.spines["bottom"].set_visible(False)
    ax_b.spines["top"].set_visible(False)
    ax_t.tick_params(labelbottom=False)

    # diagonal break marks on the y-spine
    kw = dict(marker=[(-1, -0.5), (1, 0.5)], markersize=10, linestyle="none",
              color=BASE, mec=BASE, mew=1.2, clip_on=False)
    ax_t.plot([0], [0], transform=ax_t.transAxes, **kw)
    ax_b.plot([0], [1], transform=ax_b.transAxes, **kw)

    for x, i, j in bar_positions(len(labels)):
        label_bar(ax_b, x, vals[i][j], 0, hi_b, fmt, fontsize=9)
        label_bar(ax_t, x, vals[i][j], lo_t, hi_t, fmt, fontsize=9)

    ax_b.set_xticks(range(len(labels)), labels, fontsize=11)
    choice_legend(ax_t)
    fig.supylabel(y_label, fontsize=10, x=0.02)
    add_titles(fig, title, subtitle)
    gs.tight_layout(fig, rect=(0.03, 0, 1, 0.91 - 0.045 * extra))
    fig.savefig(path, facecolor="white")
    plt.close(fig)


def main():
    if len(sys.argv) < 2:
        sys.exit(f"usage: {sys.argv[0]} reports/<name>.md [reports/<name2>.md ...]")
    reports = [Path(p) for p in sys.argv[1:]]
    bad = [str(p) for p in reports if not p.is_file()]
    if bad:
        sys.exit(f"error: not a file: {', '.join(bad)}")

    machines = [(report, *parse_report(report)) for report in reports]

    metrics = [
        (0, "compression_speed", "Compression speed by dataset", "Average compression speed", SPEED_Y, None),
        (1, "decompression_speed", "Decompression speed by dataset", "Average decompression speed", SPEED_Y, None),
        (2, "compression_ratio", "Compression ratio by dataset", "Average compression ratio", RATIO_Y, RATIO_CAP),
    ]

    outputs = []
    for report, cpu, datasets, table in machines:
        out = Path("diagrams") / report.stem
        out.mkdir(parents=True, exist_ok=True)
        subtitle = f"{len(datasets)} datasets · {cpu}" if cpu else f"{len(datasets)} datasets"

        for metric, stem, ds_title, _, y_label, cap in metrics:
            cap_note = f" — y-axis capped at {cap}" if cap else ""
            per_dataset_chart(out / f"{stem}.png", datasets, table, metric, CHOICES,
                              ds_title + cap_note, subtitle, y_label, cap)
            per_dataset_chart(out / f"{stem}_zstd_vs_alp.png", datasets, table, metric,
                              CHOICES[1:], ds_title + " (ZSTD vs ALP)" + cap_note, subtitle, y_label, cap)

        outputs.append(out)

    # combined average charts: one bar group per machine, written to diagrams/
    labels = [report.stem for report, *_ in machines]
    sizes = [len(datasets) for _, _, datasets, _ in machines]
    if len(set(sizes)) == 1:
        avg_subtitle = f"Arithmetic mean over {sizes[0]} datasets per machine"
    else:
        avg_subtitle = "Arithmetic mean per machine"

    for metric, stem, _, avg_title, y_label, _ in metrics:
        vals = [[table[AVG_KEY][c][metric] for c in CHOICES] for _, _, _, table in machines]
        avg_path = Path("diagrams") / f"avg_{stem}.png"
        outlier = pick_outlier(vals)
        if outlier is None:
            grouped_avg_chart(avg_path, labels, vals, y_label, avg_title, avg_subtitle)
        else:
            grouped_broken_chart(avg_path, labels, vals, outlier, y_label, avg_title, avg_subtitle)
        outputs.append(avg_path)

    ras = [(report.stem, cpu, parse_random_access(report)) for report, cpu, _, _ in machines]
    ras = [(stem, cpu, ra) for stem, cpu, ra in ras if ra is not None]
    if ras:
        ra_labels = [stem for stem, _, _ in ras]
        vals = [[rows_per_sec[c] for c in CHOICES] for _, _, (_, _, rows_per_sec) in ras]
        heads = {(n_rows, dataset) for _, _, (n_rows, dataset, _) in ras}
        if len(heads) == 1:
            n_rows, dataset = heads.pop()
            head = f"Decoding {n_rows} random rows" + (f" from {dataset}" if dataset else "")
        else:
            head = "Decoding random rows"
        ra_subtitle = head
        ra_path = Path("diagrams") / "avg_random_access.png"
        kwargs = dict(fmt=human, y_fmt=EngFormatter(places=0, sep=""))
        outlier = pick_outlier(vals)
        if outlier is None:
            grouped_avg_chart(ra_path, ra_labels, vals, ROWS_Y, "Random access speed", ra_subtitle, **kwargs)
        else:
            grouped_broken_chart(ra_path, ra_labels, vals, outlier, ROWS_Y, "Random access speed", ra_subtitle, **kwargs)
        outputs.append(ra_path)

    for out in outputs:
        if out.is_dir():
            for p in sorted(out.iterdir()):
                print(p)
        else:
            print(out)


if __name__ == "__main__":
    main()
