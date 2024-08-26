use parse::{parse_template_str, Results, Step, StepKind};

/** DX **/
// this test will fail to build if `clone` or `default formatter` is not available
#[test]
fn confirm_clone_and_debug() {
    let template_str: &str = "p { color: blue; }";
    let steps = parse_template_str(template_str, StepKind::Initial);

    let cloned = steps.clone();
    let _debugged = format!("{:?}", cloned);
}

#[test]
fn parse_selector_no_spaces() {
    let template_str: &str = "p{color:blue;}";

    let steps = parse_template_str(template_str, StepKind::Initial);
    let expected: Results = Vec::from([
        Step {
            kind: StepKind::Initial,
            origin: 0,
            target: 0,
        },
        Step {
            kind: StepKind::Selectors,
            origin: 0,
            target: 1,
        },
        Step {
            kind: StepKind::Declaration,
            origin: 1,
            target: 2,
        },
        Step {
            kind: StepKind::Property,
            origin: 2,
            target: 7,
        },
        Step {
            kind: StepKind::PropertyAssignment,
            origin: 7,
            target: 8,
        },
        Step {
            kind: StepKind::Value,
            origin: 8,
            target: 12,
        },
        Step {
            kind: StepKind::ValueClose,
            origin: 12,
            target: 13,
        },
        Step {
            kind: StepKind::DeclarationClose,
            origin: 13,
            target: 14,
        },
    ]);

    assert_eq!(steps, expected);
}

#[test]
fn parse_selector_with_spaces() {
    let template_str: &str = "
		p {
			color: blue;
		}
	";

    let steps = parse_template_str(template_str, StepKind::BaseScope);
    let expected: Results = Vec::from([
        Step {
            kind: StepKind::BaseScope,
            origin: 0,
            target: 3,
        },
        Step {
            kind: StepKind::Selectors,
            origin: 3,
            target: 5,
        },
        Step {
            kind: StepKind::Declaration,
            origin: 5,
            target: 10,
        },
        Step {
            kind: StepKind::Property,
            origin: 10,
            target: 15,
        },
        Step {
            kind: StepKind::PropertyAssignment,
            origin: 15,
            target: 17,
        },
        Step {
            kind: StepKind::Value,
            origin: 17,
            target: 21,
        },
        Step {
            kind: StepKind::ValueClose,
            origin: 21,
            target: 25,
        },
        Step {
            kind: StepKind::DeclarationClose,
            origin: 25,
            target: 26,
        },
        Step {
            kind: StepKind::BaseScope,
            origin: 26,
            target: 28,
        },
    ]);

    assert_eq!(steps, expected);
}

#[test]
fn parse_selector_with_media_selector_no_spaces() {
    let template_str: &str = "@media print{p{color: blue;}}";

    let steps = parse_template_str(template_str, StepKind::Initial);
    let expected: Results = Vec::from([
        Step {
            kind: StepKind::Initial,
            origin: 0,
            target: 0,
        },
        Step {
            kind: StepKind::AtRule,
            origin: 0,
            target: 12,
        },
        Step {
            kind: StepKind::AtRuleScope,
            origin: 12,
            target: 13,
        },
        Step {
            kind: StepKind::Selectors,
            origin: 13,
            target: 14,
        },
        Step {
            kind: StepKind::Declaration,
            origin: 14,
            target: 15,
        },
        Step {
            kind: StepKind::Property,
            origin: 15,
            target: 20,
        },
        Step {
            kind: StepKind::PropertyAssignment,
            origin: 20,
            target: 22,
        },
        Step {
            kind: StepKind::Value,
            origin: 22,
            target: 26,
        },
        Step {
            kind: StepKind::ValueClose,
            origin: 26,
            target: 27,
        },
        Step {
            kind: StepKind::DeclarationClose,
            origin: 27,
            target: 28,
        },
        Step {
            kind: StepKind::Selectors,
            origin: 28,
            target: 29,
        },
    ]);

    assert_eq!(steps, expected);
}

#[test]
fn parse_selector_with_injections_no_spaces() {
    let template_str: &str = "{}p{color:blue;}{ }";

    let steps = parse_template_str(template_str, StepKind::Initial);
    let expected: Results = Vec::from([
        Step {
            kind: StepKind::Initial,
            origin: 0,
            target: 0,
        },
        Step {
            kind: StepKind::Injection,
            origin: 0,
            target: 1,
        },
        Step {
            kind: StepKind::InjectionConfirmed,
            origin: 1,
            target: 2,
        },
        Step {
            kind: StepKind::Selectors,
            origin: 2,
            target: 3,
        },
        Step {
            kind: StepKind::Declaration,
            origin: 3,
            target: 4,
        },
        Step {
            kind: StepKind::Property,
            origin: 4,
            target: 9,
        },
        Step {
            kind: StepKind::PropertyAssignment,
            origin: 9,
            target: 10,
        },
        Step {
            kind: StepKind::Value,
            origin: 10,
            target: 14,
        },
        Step {
            kind: StepKind::ValueClose,
            origin: 14,
            target: 15,
        },
        Step {
            kind: StepKind::DeclarationClose,
            origin: 15,
            target: 16,
        },
        Step {
            kind: StepKind::Injection,
            origin: 16,
            target: 17,
        },
        Step {
            kind: StepKind::InjectionSpace,
            origin: 17,
            target: 18,
        },
        Step {
            kind: StepKind::InjectionConfirmed,
            origin: 18,
            target: 19,
        },
    ]);

    assert_eq!(steps, expected);
}

#[test]
fn parse_at_rule_with_quotes_no_spaces() {
    let template_str: &str = "@import('hai.css');";

    let steps = parse_template_str(template_str, StepKind::Initial);
    let expected: Results = Vec::from([
        Step {
            kind: StepKind::Initial,
            origin: 0,
            target: 0,
        },
        Step {
            kind: StepKind::AtRule,
            origin: 0,
            target: 8,
        },
        Step {
            kind: StepKind::AtRuleQuote,
            origin: 8,
            target: 16,
        },
        Step {
            kind: StepKind::AtRuleQuoteClose,
            origin: 16,
            target: 17,
        },
        Step {
            kind: StepKind::AtRule,
            origin: 17,
            target: 18,
        },
        Step {
            kind: StepKind::AtRuleClose,
            origin: 18,
            target: 19,
        },
    ]);

    assert_eq!(steps, expected);
}

#[test]
fn parse_at_rule_with_double_quotes_no_spaces() {
    let template_str: &str = "@import(\"hai.css\");";

    let steps = parse_template_str(template_str, StepKind::Initial);
    let expected: Results = Vec::from([
        Step {
            kind: StepKind::Initial,
            origin: 0,
            target: 0,
        },
        Step {
            kind: StepKind::AtRule,
            origin: 0,
            target: 8,
        },
        Step {
            kind: StepKind::AtRuleDoubleQuote,
            origin: 8,
            target: 16,
        },
        Step {
            kind: StepKind::AtRuleDoubleQuoteClose,
            origin: 16,
            target: 17,
        },
        Step {
            kind: StepKind::AtRule,
            origin: 17,
            target: 18,
        },
        Step {
            kind: StepKind::AtRuleClose,
            origin: 18,
            target: 19,
        },
    ]);

    assert_eq!(steps, expected);
}
