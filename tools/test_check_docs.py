from pathlib import Path
from tempfile import TemporaryDirectory
from unittest import TestCase

from check_docs import validate


class CanonicalDocumentChecks(TestCase):
    def setUp(self):
        directory = TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        self.root = Path(directory.name)
        self.docs = self.root / "docs"
        self.docs.mkdir()
        self.write(self.root / "AGENTS.md", "[Policy](docs/DECISIONS.md#d-26--selection)")
        self.write(
            self.docs / "DECISIONS.md",
            "| D-26 | Current | Archived choice |\n\n## D-26 \u2014 Selection\nHistorical mention: RIPS: Rust-\u043f\u043e\u0440\u0442 MIPS.",
        )
        self.write(self.docs / "REQUIREMENTS.md", "| CORE-03 | Transport guarantees |")

    def write(self, path, text):
        path.write_text(text, encoding="utf-8")

    def test_valid_links_and_archived_wording(self):
        self.assertEqual(validate(self.root), [])

    def test_noncanonical_root_is_resolved_before_comparing_paths(self):
        self.assertEqual(validate(self.docs / '..'), [])

    def test_unresolved_heading_and_unknown_requirement(self):
        self.write(self.docs / "ARCHITECTURE.md", "[Missing](DECISIONS.md#absent)\n| CORE-99 | typo |")
        errors = "\n".join(validate(self.root))
        self.assertIn("unknown heading", errors)
        self.assertIn("unknown identifier CORE-99", errors)

    def test_duplicate_requirement_and_retired_normative_wording(self):
        self.write(self.docs / "REQUIREMENTS.md", "| CORE-03 | Transport |\n| CORE-03 | Transport |")
        self.write(self.docs / "ARCHITECTURE.md", "RIPS: Rust-\u043f\u043e\u0440\u0442 MIPS.")
        errors = "\n".join(validate(self.root))
        self.assertIn("duplicate registry identifier: CORE-03", errors)
        self.assertIn("retired instruction", errors)
