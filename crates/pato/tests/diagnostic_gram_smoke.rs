use gram_codec::parse_gram;
use relateby_pato::diagnostic_gram;
use relateby_pato::diagnostics::{
    Diagnostic, DiagnosticCode, Edit, FileDiagnostics, Location, Remediation, RemediationOption,
    RemediationSteps,
};
use std::path::PathBuf;

#[test]
fn serializes_each_remediation_grade_to_parseable_gram() {
    let file = PathBuf::from("sample.gram");
    let report = FileDiagnostics::new(
        "sample.gram",
        vec![
            Diagnostic::new(
                DiagnosticCode::P004,
                "Relationship label 'knows' should be uppercase",
                Location::new(4, 10),
                Remediation::Auto {
                    summary: "Recase to KNOWS".to_string(),
                    steps: RemediationSteps::Structured(vec![Edit::Replace {
                        file: file.clone(),
                        line: 4,
                        column: 10,
                        replace: "knows".to_string(),
                        with: "KNOWS".to_string(),
                    }]),
                },
            ),
            Diagnostic::new(
                DiagnosticCode::P002,
                "Identity 'alice' is defined twice",
                Location::new(2, 1),
                Remediation::Guided {
                    summary: "Rename one of the duplicate identities".to_string(),
                    steps: RemediationSteps::Inline(vec![
                        "Rename either occurrence so the identities are unique.".to_string(),
                    ]),
                },
            ),
            Diagnostic::new(
                DiagnosticCode::P005,
                "'persn' is referenced but not defined in this file",
                Location::new(6, 3),
                Remediation::Ambiguous {
                    summary: "Choose whether the reference should be renamed or defined"
                        .to_string(),
                    decision: "Is 'persn' a misspelling, or should it be defined here?".to_string(),
                    options: vec![
                        RemediationOption {
                            description: "Rename reference to 'person' (closest match)".to_string(),
                            edit: Edit::Replace {
                                file: file.clone(),
                                line: 6,
                                column: 3,
                                replace: "persn".to_string(),
                                with: "person".to_string(),
                            },
                        },
                        RemediationOption {
                            description: "Add a 'persn' definition to this file".to_string(),
                            edit: Edit::Append {
                                file: file.clone(),
                                content: "(persn:Entity)".to_string(),
                            },
                        },
                    ],
                },
            ),
            Diagnostic::new(
                DiagnosticCode::P007,
                "No schema was found for this file",
                Location::new(1, 1),
                Remediation::None,
            ),
        ],
    );

    let gram = diagnostic_gram::to_gram(&[report]).expect("diagnostic gram should serialize");
    parse_gram(&gram).expect("diagnostic gram should itself parse");
}
