"""Validate the local canonical document set without contacting upstream."""

import re
import sys
from collections import Counter
from pathlib import Path
from urllib.parse import unquote, urlsplit


DECISION_ID = re.compile(r"^D-\d{2}$")
REQUIREMENT_ID = re.compile(r"^[A-Z]+-\d{2}$")
LINK = re.compile(r"\[[^\]\n]+\]\(([^)\s]+)\)")
NORMATIVE_FILES = {
    "ARCHITECTURE.md", "COMPATIBILITY.md", "CONFIGURATION.md", "DEFAULTS.md",
    "REQUIREMENTS.md", "ROADMAP.md", "SPECIFICATION.md", "UI-SPECIFICATION.md",
}

# Historical material in DECISIONS and implementation notes is outside this scan.
RETIRED_WORDING = {
    "mandatory MIPS port": re.compile(r"\bRIPS(?:\s*\(|:\s*|\s+is\s+)Rust-\u043f\u043e\u0440\u0442\s+MIPS", re.I),
    "mandatory library in a requirement": re.compile(
        r"^\|\s*(?:CORE-03\s*\|[^|\n]*,\s*Tokio,|CORE-04\s*\|\s*rustls\b|"
        r"DPI-01\s*\|[^|\n]*Lua\s+\u0447\u0435\u0440\u0435\u0437\s+mlua\b)",
        re.I | re.M,
    ),
}


def table_ids(text, pattern):
    index = text.split("\n## ", 1)[0]
    return Counter(
        fields[1].strip()
        for line in index.splitlines()
        if line.startswith("|") and pattern.fullmatch((fields := line.split("|"))[1].strip())
    )


def anchors(text):
    seen = Counter()
    result = set()
    for heading in re.findall(r"^#{1,6} (.+?) *#*$", text, re.M):
        slug = re.sub(r"[^\w\- ]", "", heading.lower()).replace(" ", "-")
        suffix = seen[slug]
        result.add(f"{slug}-{suffix}" if suffix else slug)
        seen[slug] += 1
    return result


def validate(root):
    docs = root / "docs"
    files = {path: path.read_text(encoding="utf-8") for path in docs.glob("*.md")}
    instructions = root / "AGENTS.md"
    files[instructions] = instructions.read_text(encoding="utf-8")
    requirements = table_ids(files[docs / "REQUIREMENTS.md"], REQUIREMENT_ID)
    decisions = table_ids(files[docs / "DECISIONS.md"], DECISION_ID)
    problems = []
    for identifier, count in (requirements + decisions).items():
        if count != 1:
            problems.append(f"duplicate registry identifier: {identifier}")
    if not requirements or not decisions:
        problems.append("canonical registries are empty")
    if "D-26" not in decisions or not any(slug.startswith("d-26-") for slug in anchors(files[docs / "DECISIONS.md"])):
        problems.append("D-26 needs a current entry and a linked replacement description")

    for path, content in files.items():
        relative = path.relative_to(root)
        text = re.sub(r"(?ms)^\x60{3}[^\n]*\n.*?^\x60{3}[ \t]*$", "", content)
        for match in LINK.finditer(text):
            url = urlsplit(match[1])
            if url.scheme or url.netloc:
                continue
            target = (path.parent / unquote(url.path)).resolve() if url.path else path
            if not target.is_relative_to(root) or not target.exists():
                problems.append(f"{relative}: unresolved local link {match[1]}")
            elif url.fragment and target.suffix == ".md":
                target_text = files.get(target)
                if target_text is None:
                    target_text = target.read_text(encoding="utf-8")
                if unquote(url.fragment) not in anchors(target_text):
                    problems.append(f"{relative}: unknown heading in {match[1]}")
        for identifier in re.findall(r"(?m)^\|\s*([A-Z]+-\d{2})\s*\|", text):
            registry = decisions if DECISION_ID.fullmatch(identifier) else requirements
            if identifier not in registry:
                problems.append(f"{relative}: unknown identifier {identifier}")
        if path.name in NORMATIVE_FILES:
            for reason, pattern in RETIRED_WORDING.items():
                if pattern.search(text):
                    problems.append(f"{relative}: retired instruction ({reason})")
    return problems


if __name__ == "__main__":
    errors = validate(Path(__file__).resolve().parents[1])
    if errors:
        print("\n".join(errors))
        sys.exit(1)
    print("Canonical document links, IDs and D-26 wording checked.")
