import json
import os
from pathlib import Path
from urllib.request import Request, urlopen
from urllib.parse import quote

sources = json.loads(Path("upstream/sources.json").read_text(encoding="utf-8"))
interval = os.environ.get("INTERVAL", "all")
lines = ["# Upstream review report", "", "No code or resources are imported automatically.", "",
         "| Source | Baseline | Head | Status |", "|---|---|---|---|"]
failed = False
for source in sources:
    if interval != "all" and int(interval) != source["interval_hours"]:
        continue
    repo = source["repository"]
    baseline = source["baseline"]
    url = f"https://api.github.com/repos/{repo}/commits/{quote(source['ref'], safe='')}"
    try:
        req = Request(url, headers={"Authorization": "Bearer " + os.environ["GH_TOKEN"],
                                    "Accept": "application/vnd.github+json", "User-Agent": "Aeris-upstream-review"})
        with urlopen(req, timeout=30) as response:
            head = json.load(response)["sha"]
        status = "unchanged" if head == baseline else f"[review changes](https://github.com/{repo}/compare/{baseline}...{head})"
    except Exception as error:
        failed = True
        head = "unavailable"
        status = type(error).__name__
    lines.append(f"| {repo} | {baseline} | {head} | {status} |")
report = "\n".join(lines) + "\n"
Path("upstream-report.md").write_text(report, encoding="utf-8")
if os.environ.get("GITHUB_STEP_SUMMARY"):
    with open(os.environ["GITHUB_STEP_SUMMARY"], "a", encoding="utf-8") as summary:
        summary.write(report)
raise SystemExit(1 if failed else 0)
