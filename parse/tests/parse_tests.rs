use parse::{parse_template_str, Results, Step, StepKind};

/** DX **/
// this test will fail to build if `clone` or `default formatter` is not available
#[test]
fn confirm_clone_and_debug() {
    let template_str: &str = "p { color: blue; }";
    let steps = parse_template_str(template_str, StepKind::BaseScope);

    let cloned = steps.clone();
    let _debugged = format!("{:?}", cloned);
}

#[test]
fn parse_selector_no_spaces() {
    let template_str: &str = "p{color:blue;}";

    let steps = parse_template_str(template_str, StepKind::BaseScope);
    let expected: Results = Vec::from([]);

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
    let expected: Results = Vec::from([]);

    assert_eq!(steps, expected);
}

#[test]
fn parse_selector_with_media_selector_no_spaces() {
    let template_str: &str = "@media print{p{color: blue;}}";

    let steps = parse_template_str(template_str, StepKind::BaseScope);
    let expected: Results = Vec::from([]);

    assert_eq!(steps, expected);
}

#[test]
fn parse_selector_with_media_selector_with_spaces() {
    let template_str: &str = "
		@media print {
			p {
				color: blue;
			}
		}
	";

    let steps = parse_template_str(template_str, StepKind::BaseScope);
    let expected: Results = Vec::from([]);

    assert_eq!(steps, expected);
}
