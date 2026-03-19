use gram_codec::parse_gram;
use relateby_pato::diagnostic_gram;
use relateby_pato::diagnostics::{
    rule_info, Diagnostic, DiagnosticCode, Edit, FileDiagnostics, Location, Remediation,
    RemediationOption,
};
use serde_json::Value as JsonValue;
use std::path::PathBuf;

#[test]
fn serializes_each_remediation_grade_to_parseable_gram() {
    let file = PathBuf::from("sample.gram");
    let p005_remediation = rule_info(DiagnosticCode::P005)
        .remediation
        .expect("P005 remediation template should exist");
    let report = FileDiagnostics::new(
        "sample.gram",
        vec![
            Diagnostic::new(
                DiagnosticCode::P004,
                Location::new(4, 10),
                Remediation::Auto {
                    id: DiagnosticCode::P004
                        .remediation_id()
                        .expect("P004 remediation template should exist"),
                    edits: vec![Edit::Replace {
                        file: file.clone(),
                        line: 4,
                        column: 10,
                        replace: "knows".to_string(),
                        with: "KNOWS".to_string(),
                    }],
                },
            )
            .with_fact("label_kind", "relationship")
            .with_fact("observed", "knows")
            .with_fact("expected", "KNOWS"),
            Diagnostic::new(
                DiagnosticCode::P002,
                Location::new(2, 1),
                Remediation::Guided {
                    id: DiagnosticCode::P002
                        .remediation_id()
                        .expect("P002 remediation template should exist"),
                    edits: Vec::new(),
                },
            )
            .with_fact("identity", "alice")
            .with_fact("first_line", 1_u32)
            .with_fact("first_column", 1_u32),
            Diagnostic::new(
                DiagnosticCode::P005,
                Location::new(6, 3),
                Remediation::Ambiguous {
                    id: p005_remediation.id,
                    options: vec![
                        RemediationOption {
                            id: p005_remediation.option_templates[0].id,
                            edit: Edit::Replace {
                                file: file.clone(),
                                line: 6,
                                column: 3,
                                replace: "persn".to_string(),
                                with: "person".to_string(),
                            },
                        },
                        RemediationOption {
                            id: p005_remediation.option_templates[1].id,
                            edit: Edit::Append {
                                file: file.clone(),
                                content: "(persn:Entity)".to_string(),
                            },
                        },
                    ],
                },
            )
            .with_fact("unresolved_identity", "persn")
            .with_fact("suggested_identity", "person"),
            Diagnostic::new(DiagnosticCode::P007, Location::new(1, 1), Remediation::None),
        ],
    );

    let gram = diagnostic_gram::to_gram(&[report]).expect("diagnostic gram should serialize");
    assert!(gram.contains(":Problem"));
    assert!(gram.contains("recase-label"));
    assert!(gram.contains("// "));
    parse_gram(&gram).expect("diagnostic gram should itself parse");
}

#[test]
fn comments_are_optional_and_json_omits_derived_prose() {
    let report = FileDiagnostics::new(
        "sample.gram",
        vec![Diagnostic::new(
            DiagnosticCode::P002,
            Location::new(2, 1),
            Remediation::Guided {
                id: DiagnosticCode::P002
                    .remediation_id()
                    .expect("P002 remediation template should exist"),
                edits: Vec::new(),
            },
        )
        .with_fact("identity", "alice")
        .with_fact("first_line", 1_u32)
        .with_fact("first_column", 1_u32)],
    );

    let with_comments =
        diagnostic_gram::to_gram(&[report.clone()]).expect("gram with comments should serialize");
    let without_comments = diagnostic_gram::to_gram_without_comments(&[report.clone()])
        .expect("gram without comments should serialize");
    assert!(with_comments.contains("// "));
    assert!(!without_comments.contains("// "));
    parse_gram(&with_comments).expect("commented gram should parse");
    parse_gram(&without_comments).expect("comment-free gram should parse");

    let json = diagnostic_gram::to_json(&[report]).expect("json should serialize");
    let encoded = json.to_string();
    let JsonValue::Object(root) = json else {
        panic!("expected diagnostics json object");
    };
    assert!(root.get("problems").is_some());
    assert!(!encoded.contains("\"message\""));
    assert!(!encoded.contains("\"decision\""));
    assert_eq!(
        root["problems"][0]["remediation"],
        JsonValue::String("rename-duplicate-identity".to_string())
    );
}
