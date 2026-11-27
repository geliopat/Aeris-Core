import copy
import io
import json
import unittest
from unittest.mock import patch

import check_upstream


class UpstreamReportTests(unittest.TestCase):
    def setUp(self):
        self.sources = [
            {
                "repository": "owner/established",
                "ref": "main",
                "baseline": "a" * 40,
                "interval_hours": 6,
                "purpose": "existing reference",
            },
            {
                "repository": "owner/candidate",
                "ref": "FlClash",
                "baseline": None,
                "interval_hours": 24,
                "purpose": "new reference",
                "applicability": "protocol and Windows review",
                "observed_sha": None,
                "researched_sha": None,
                "approved_implementation_sha": None,
            },
        ]

    def test_existing_baseline_comparison_and_interval(self):
        calls = []

        def get_head(repo, ref):
            calls.append((repo, ref))
            return "b" * 40

        report, failed = check_upstream.make_report(self.sources, "6", get_head)
        self.assertFalse(failed)
        self.assertEqual(calls, [("owner/established", "main")])
        self.assertIn(f"compare/{'a' * 40}...{'b' * 40}", report)
        self.assertNotIn("owner/candidate", report)

    def test_candidate_observation_does_not_approve_or_mutate_metadata(self):
        original = copy.deepcopy(self.sources)
        report, failed = check_upstream.make_report(self.sources, "24", lambda repo, ref: "c" * 40)
        self.assertFalse(failed)
        self.assertIn("owner/candidate | FlClash | not approved", report)
        self.assertIn("Candidate; research and license review pending", report)
        self.assertIn("| pending | not recorded |", report)
        self.assertIn("protocol and Windows review", report)
        self.assertNotIn("compare/None", report)
        self.assertEqual(self.sources, original)

    def test_failures_are_reported_without_claiming_success(self):
        def fail(_repo, _ref):
            raise TimeoutError("secret detail should not appear")

        report, failed = check_upstream.make_report(self.sources, "all", fail)
        self.assertTrue(failed)
        self.assertEqual(report.count("unavailable | TimeoutError"), 2)
        self.assertNotIn("secret detail", report)

    def test_unchanged_baseline(self):
        report, failed = check_upstream.make_report(self.sources, "6", lambda repo, ref: "a" * 40)
        self.assertFalse(failed)
        self.assertIn("unchanged", report)
        self.assertNotIn("review changes", report)

    def test_fetch_head_escapes_branch_and_sends_token_only_in_header(self):
        payload = io.BytesIO(json.dumps({"object": {"sha": "d" * 40}}).encode())
        with patch.object(check_upstream, "urlopen", return_value=payload) as fetch:
            head = check_upstream.fetch_head("owner/repo", "feature/name#part", "test-token")
        self.assertEqual(head, "d" * 40)
        request = fetch.call_args.args[0]
        self.assertTrue(request.full_url.endswith("/git/ref/heads/feature/name%23part"))
        self.assertNotIn("test-token", request.full_url)
        self.assertEqual(request.get_header("Authorization"), "Bearer test-token")

    def test_missing_token_is_reported_without_network(self):
        with patch.object(check_upstream, "urlopen") as fetch:
            report, failed = check_upstream.make_report(
                self.sources, "6", lambda repo, ref: check_upstream.fetch_head(repo, ref, "")
            )
        self.assertTrue(failed)
        self.assertIn("unavailable | ValueError", report)
        fetch.assert_not_called()

    def test_changed_and_new_heads_use_one_bounded_metadata_lookup(self):
        calls = []

        def recent(repo, head):
            calls.append((repo, head))
            return ["Fix Windows TLS compatibility", "Security diagnostics"]

        report, failed = check_upstream.make_report(
            self.sources, "all", lambda repo, ref: "c" * 40, recent
        )
        self.assertFalse(failed)
        self.assertEqual(len(calls), 2)
        self.assertIn("Potential: security, compatibility, protocol, Windows, diagnostics, bug fixes", report)
        self.assertIn("not an exhaustive diff", report)
        self.assertIn("source, security and license review remain pending", report)

    def test_prior_observation_comparison_and_no_lookup_when_unchanged(self):
        self.sources[1]["observed_sha"] = "c" * 40
        calls = []
        report, failed = check_upstream.make_report(
            self.sources, "24", lambda repo, ref: "c" * 40,
            lambda repo, head: calls.append((repo, head)),
        )
        self.assertFalse(failed)
        self.assertEqual(calls, [])
        self.assertNotIn("compare prior observation", report)
        report, failed = check_upstream.make_report(
            self.sources, "24", lambda repo, ref: "d" * 40, lambda repo, head: []
        )
        self.assertFalse(failed)
        self.assertIn(f"compare/{'c' * 40}...{'d' * 40}", report)
        self.assertIn("| not approved |", report)

    def test_metadata_failure_is_visible_without_exposing_error(self):
        def fail(_repo, _head):
            raise TimeoutError("private details")

        report, failed = check_upstream.make_report(
            self.sources, "24", lambda repo, ref: "e" * 40, fail
        )
        self.assertTrue(failed)
        self.assertIn("metadata unavailable (TimeoutError)", report)
        self.assertNotIn("private details", report)

    def test_recent_subject_fetch_is_limited_and_does_not_return_bodies(self):
        commits = [{"commit": {"message": f"subject {number}\nbody secret"}} for number in range(12)]
        payload = io.BytesIO(json.dumps(commits).encode())
        with patch.object(check_upstream, "urlopen", return_value=payload) as fetch:
            subjects = check_upstream.fetch_recent_subjects("owner/repo", "f" * 40, "test-token")
        self.assertEqual(len(subjects), 10)
        self.assertEqual(subjects[0], "subject 0")
        self.assertIn("/repos/owner/repo/commits?sha=", fetch.call_args.args[0].full_url)
        self.assertTrue(fetch.call_args.args[0].full_url.endswith("per_page=10"))
        self.assertNotIn("body secret", str(subjects))


if __name__ == "__main__":
    unittest.main()
