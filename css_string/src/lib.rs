use parse::{get_text_from_step, parse_template_str, Step, StepKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Results {
    pub strs: Vec<String>,
    pub injs: Vec<StepKind>,
}

// pub enum StepKind {
//     Initial,
//     BaseScope,
//     Property,
//     PropertyClose,
//     PropertyAssignment,
//     Value,
//     ValueClose,
//     Declaration,
//     DeclarationClose,
//     Selectors,
//     AtRuleScope,
//     AtRuleScopeClose,
//     AtRule,
//     Injection,
//     InjectionSpace,
//     InjectionConfirmed,
// }

impl Results {
    pub fn new() -> Results {
        Results {
            strs: Vec::from(["".to_string()]),
            injs: Vec::new(),
        }
    }
}

// this needs to be a function
// this is what is cached by a parent scope or context
pub fn compose(template_str: &str) -> Results {
    // check for already built results
    let mut results = Results::new();

    for step in parse_template_str(template_str, StepKind::Initial) {
        match step.kind {
            // steps
            StepKind::Selectors => push_selectors(&mut results, template_str, step),
            StepKind::SelectorQuote => push_selector_quote(&mut results, template_str, step),
            StepKind::SelectorQuoteClose => push_selector_quote(&mut results, template_str, step),
            StepKind::SelectorDoubleQuote => {
                push_selector_double_quote(&mut results, template_str, step)
            }
            StepKind::SelectorDoubleQuoteClose => {
                push_selector_double_quote(&mut results, template_str, step)
            }
            StepKind::AtRule => push_at_rule(&mut results, template_str, step),
            StepKind::AtRuleQuote => push_selector_quote(&mut results, template_str, step),
            StepKind::AtRuleQuoteClose => push_selector_quote(&mut results, template_str, step),
            StepKind::AtRuleDoubleQuote => {
                push_selector_double_quote(&mut results, template_str, step)
            }
            StepKind::AtRuleDoubleQuoteClose => {
                push_selector_double_quote(&mut results, template_str, step)
            }
            StepKind::Declaration => push_declaration(&mut results, template_str, step),
            StepKind::DeclarationClose => push_declaration_close(&mut results, template_str, step),
            StepKind::AtRuleScope => push_declaration(&mut results, template_str, step),
            StepKind::AtRuleScopeClose => push_declaration_close(&mut results, template_str, step),
            StepKind::PropertyAssignment => {
                push_property_assignment(&mut results, template_str, step)
            }
            StepKind::ValueClose => push_value_close(&mut results, template_str, step),
            StepKind::AtRuleClose => push_value_close(&mut results, template_str, step),
            _ => {}
        }
    }

    results
}

fn push_selectors(results: &mut Results, template_str: &str, step: Step) {
    let selectors = get_text_from_step(template_str, &step).trim();

    // send to a function that will parse selectors into string

    if let Some(last) = results.strs.last_mut() {
        last.push_str(selectors);
    }
}

fn push_at_rule(results: &mut Results, template_str: &str, step: Step) {
    let at_rule = get_text_from_step(template_str, &step).trim();

    // send to a function that will parse at_rule into string

    if let Some(last) = results.strs.last_mut() {
        last.push_str(at_rule);
    }
}

fn push_declaration(results: &mut Results, template_str: &str, step: Step) {
    if let Some(last) = results.strs.last_mut() {
        last.push('{');
    }
}

fn push_declaration_close(results: &mut Results, template_str: &str, step: Step) {
    if let Some(last) = results.strs.last_mut() {
        last.push('}');
    }
}

fn push_property_assignment(results: &mut Results, template_str: &str, step: Step) {
    if let Some(last) = results.strs.last_mut() {
        last.push(':');
    }
}

fn push_value_close(results: &mut Results, template_str: &str, step: Step) {
    if let Some(last) = results.strs.last_mut() {
        last.push(';');
    }
}

fn push_selector_quote(results: &mut Results, template_str: &str, step: Step) {
    if let Some(last) = results.strs.last_mut() {
        last.push('\'');
    }
}

fn push_selector_double_quote(results: &mut Results, template_str: &str, step: Step) {
    if let Some(last) = results.strs.last_mut() {
        last.push('"');
    }
}
