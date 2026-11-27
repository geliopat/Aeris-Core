import json
import os
import re
from pathlib import Path
from urllib.parse import quote
from urllib.request import Request, urlopen


def fetch_head(repository, ref, token):
    if not token:
        raise ValueError("GH_TOKEN is required")
    url = f"https://api.github.com/repos/{repository}/git/ref/heads/{quote(ref, safe='/')}"
    request = Request(
        url,
        headers={
            "Authorization": "Bearer " + token,
            "Accept": "application/vnd.github+json",
            "User-Agent": "Aeris-upstream-review",
        },
    )
    with urlopen(request, timeout=30) as response:
        return json.load(response)["object"]["sha"]


def fetch_recent_subjects(repository, head, token):
    if not token:
        raise ValueError("GH_TOKEN is required")
    url = f"https://api.github.com/repos/{repository}/commits?sha={quote(head, safe='')}&per_page=10"
    request = Request(
        url,
        headers={
            "Authorization": "Bearer " + token,
            "Accept": "application/vnd.github+json",
            "User-Agent": "Aeris-upstream-review",
        },
    )
    with urlopen(request, timeout=30) as response:
        commits = json.load(response)
    return [
        commit["commit"]["message"].split("\n", 1)[0][:160]
        for commit in commits[:10]
    ]


SUBJECT_CATEGORIES = {
    "security": r"\b(security|vulnerab\w*|cve|exploit\w*|auth\w*)\b",
    "compatibility": r"\b(compat\w*|interop\w*|regression\w*)\b",
    "protocol": r"\b(protocol\w*|xhttp|xudp|vless|dns|tls|quic)\b",
    "Windows": r"\b(windows|win32|wintun|windivert|wfp)\b",
    "performance": r"\b(perf\w*|optim\w*|latency|throughput)\b",
    "dependencies": r"\b(dependenc\w*|deps|crate\w*|bump|upgrade)\b",
    "diagnostics": r"\b(diagnostic\w*|logging|log|metrics|tracing)\b",
    "features": r"\b(feature\w*|support|implement\w*|add)\b",
    "architecture": r"\b(architect\w*|refactor\w*|design|lifecycle)\b",
    "bug fixes": r"\b(fix\w*|bug\w*)\b",
}


def classify_subjects(subjects):
    text = " ".join(re.sub(r"[\x00-\x1f\x7f]", " ", subject[:160]) for subject in subjects[:10])
    return [name for name, pattern in SUBJECT_CATEGORIES.items() if re.search(pattern, text, re.IGNORECASE)]


def make_report(sources, interval, get_head, get_subjects=None):
    lines = [
        "# Upstream review report",
        "",
        "No code or resources are imported automatically.",
        "Observed heads are point-in-time metadata, not researched or approved revisions.",
        "",
        "| Source | Branch | Reviewed baseline | Researched SHA | Approved implementation SHA | Observed head | Status | Applicability to assess | Recent subject indicators |",
        "|---|---|---|---|---|---|---|---|---|",
    ]
    failed = False
    for source in sources:
        if interval != "all" and int(interval) != source["interval_hours"]:
            continue
        repo = source["repository"]
        branch = source["ref"]
        baseline = source.get("baseline")
        previous = baseline or source.get("researched_sha") or source.get("observed_sha")
        indicators = "not checked"
        try:
            head = get_head(repo, branch)
            if baseline is None:
                status = "Candidate; research and license review pending"
            elif head == baseline:
                status = "unchanged"
            else:
                status = f"[review changes](https://github.com/{repo}/compare/{baseline}...{head})"
            if previous and head != previous and baseline is None:
                status += f"; [compare prior observation](https://github.com/{repo}/compare/{previous}...{head})"
        except Exception as error:
            failed = True
            head = "unavailable"
            status = type(error).__name__
        if head != "unavailable" and head != previous and get_subjects is not None:
            try:
                subjects = get_subjects(repo, head)
                categories = classify_subjects(subjects)
                indicators = "Potential: " + ", ".join(categories) if categories else "No keyword matches"
            except Exception as error:
                failed = True
                indicators = f"metadata unavailable ({type(error).__name__})"
        scope = source.get("applicability", source["purpose"])
        researched = source.get("researched_sha") or "pending"
        approved = source.get("approved_implementation_sha") or "not recorded"
        lines.append(
            f"| {repo} | {branch} | {baseline or 'not approved'} | {researched} | {approved} | "
            f"{head} | {status} | {scope} | {indicators} |"
        )
    lines.extend([
        "",
        "Indicators are potential keyword matches in at most 10 recent commit subjects; they",
        "are not an exhaustive diff or a confirmed classification of changes.",
        "Applicability, source, security and license review remain pending.",
    ])
    return "\n".join(lines) + "\n", failed


def main():
    sources = json.loads(Path("upstream/sources.json").read_text(encoding="utf-8"))
    interval = os.environ.get("INTERVAL", "all")
    token = os.environ.get("GH_TOKEN", "")
    report, failed = make_report(
        sources,
        interval,
        lambda repo, ref: fetch_head(repo, ref, token),
        lambda repo, head: fetch_recent_subjects(repo, head, token),
    )
    Path("upstream-report.md").write_text(report, encoding="utf-8")
    if os.environ.get("GITHUB_STEP_SUMMARY"):
        with open(os.environ["GITHUB_STEP_SUMMARY"], "a", encoding="utf-8") as summary:
            summary.write(report)
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())
