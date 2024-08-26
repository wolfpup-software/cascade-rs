mod routes;

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum StepKind {
    Initial,
    BaseScope,
    Property,
    PropertyClose,
    PropertyAssignment,
    Value,
    ValueClose,
    Declaration,
    DeclarationClose,
    Selectors,
    SelectorQuote,
    SelectorQuoteClose,
    SelectorDoubleQuote,
    SelectorDoubleQuoteClose,
    AtRuleScope,
    AtRuleScopeClose,
    AtRuleQuote,
    AtRuleQuoteClose,
    AtRuleDoubleQuote,
    AtRuleDoubleQuoteClose,
    AtRule,
    AtRuleClose,
    Injection,
    InjectionSpace,
    InjectionConfirmed,
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct Step {
    pub kind: StepKind,
    pub origin: usize,
    pub target: usize,
}

pub type Results = Vec<Step>;

pub fn parse_template_str(template_str: &str, intial_kind: StepKind) -> Results {
    let mut steps = Vec::from([Step {
        kind: intial_kind.clone(),
        origin: 0,
        target: 0,
    }]);

    let mut prev_inj_kind = intial_kind;

    for (index, glyph) in template_str.char_indices() {
        let front_step = match steps.last_mut() {
            Some(step) => step,
            _ => return steps,
        };

        let curr_kind = match front_step.kind {
            StepKind::InjectionConfirmed => routes::route(glyph, &prev_inj_kind),
            _ => routes::route(glyph, &front_step.kind),
        };

        if StepKind::Injection == curr_kind {
            prev_inj_kind = front_step.kind.clone();
        }

        if curr_kind == front_step.kind {
            continue;
        }

        front_step.target = index;
        steps.push(Step {
            kind: curr_kind,
            origin: index,
            target: index,
        });
    }

    if let Some(step) = steps.last_mut() {
        step.target = template_str.len();
    }

    steps
}

pub fn get_text_from_step<'a>(template_str: &'a str, step: &Step) -> &'a str {
    &template_str[step.origin..step.target]
}
