use crate::StepKind;

// Names based roughly on:
// https://html.spec.whatwg.org/multipage/parsing.html

pub fn route(glyph: char, prev_kind: &StepKind) -> StepKind {
    match prev_kind {
        StepKind::AtRule => get_kind_from_at_rule(glyph),
        StepKind::AtRuleScope => get_kind_from_at_rule_scope(glyph),
        StepKind::Selectors => get_kind_from_selectors(glyph),
        StepKind::Declaration => get_kind_from_delcaration(glyph),
        StepKind::Property => get_kind_from_property(glyph),
        StepKind::PropertyAssignment => get_kind_from_property_assignment(glyph),
        StepKind::Value => get_kind_from_value(glyph),
        StepKind::ValueClose => get_kind_from_value_close(glyph),
        StepKind::DeclarationClose => get_kind_from_declaration_close(glyph),
        // injections
        StepKind::Injection => get_kind_from_injection(glyph),
        StepKind::InjectionSpace => get_kind_from_injection_space(glyph),
        // quotes
        StepKind::SelectorQuote => get_kind_from_selector_quote(glyph),
        StepKind::SelectorQuoteClose => get_kind_from_selectors(glyph),
        StepKind::SelectorDoubleQuote => get_kind_from_selector_double_quote(glyph),
        StepKind::SelectorDoubleQuoteClose => get_kind_from_selectors(glyph),
        StepKind::AtRuleQuote => get_kind_from_at_rule_quote(glyph),
        StepKind::AtRuleQuoteClose => get_kind_from_at_rule(glyph),
        StepKind::AtRuleDoubleQuote => get_kind_from_at_rule_double_quote(glyph),
        StepKind::AtRuleDoubleQuoteClose => get_kind_from_at_rule(glyph),
        // injection confirmed
        // at rule close
        _ => get_kind_from_base_scope(glyph),
    }
}

fn get_kind_from_base_scope(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::BaseScope;
    }

    match glyph {
        '@' => StepKind::AtRule,
        '{' => StepKind::Injection,
        _ => StepKind::Selectors,
    }
}

/* Injections */
fn get_kind_from_injection(glyph: char) -> StepKind {
    match glyph {
        '}' => StepKind::InjectionConfirmed,
        _ => StepKind::InjectionSpace,
    }
}

fn get_kind_from_injection_space(glyph: char) -> StepKind {
    match glyph {
        '}' => StepKind::InjectionConfirmed,
        _ => StepKind::InjectionSpace,
    }
}

/* at rules */
fn get_kind_from_at_rule(glyph: char) -> StepKind {
    match glyph {
        '\'' => StepKind::AtRuleQuote,
        '"' => StepKind::AtRuleDoubleQuote,
        '{' => StepKind::AtRuleScope,
        ';' => StepKind::AtRuleClose,
        _ => StepKind::AtRule,
    }
}

fn get_kind_from_at_rule_scope(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::BaseScope;
    }
    match glyph {
        '}' => StepKind::AtRuleScopeClose,
        '@' => StepKind::AtRuleScope,
        _ => StepKind::Selectors,
    }
}

/* selectors */
fn get_kind_from_selectors(glyph: char) -> StepKind {
    match glyph {
        '\'' => StepKind::SelectorQuote,
        '"' => StepKind::SelectorDoubleQuote,
        '{' => StepKind::Declaration,
        _ => StepKind::Selectors,
    }
}

fn get_kind_from_delcaration(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::Declaration;
    }

    StepKind::Property
}

fn get_kind_from_property(glyph: char) -> StepKind {
    match glyph {
        ':' => StepKind::PropertyAssignment, // comment starts here
        _ => StepKind::Property,
    }
}

fn get_kind_from_property_assignment(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::PropertyAssignment;
    }
    StepKind::Value
}

fn get_kind_from_value(glyph: char) -> StepKind {
    match glyph {
        ';' => StepKind::ValueClose,
        _ => StepKind::Value,
    }
}

fn get_kind_from_value_close(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::ValueClose;
    }

    match glyph {
        '}' => StepKind::DeclarationClose,
        _ => StepKind::Property,
    }
}

fn get_kind_from_declaration_close(glyph: char) -> StepKind {
    if glyph.is_whitespace() {
        return StepKind::BaseScope;
    }

    match glyph {
        '@' => StepKind::AtRule,
        '{' => StepKind::Injection,
        _ => StepKind::Selectors,
    }
}

/* Quotes */
fn get_kind_from_selector_quote(glyph: char) -> StepKind {
    match glyph {
        '\'' => StepKind::SelectorQuoteClose,
        _ => StepKind::SelectorQuote,
    }
}

fn get_kind_from_selector_double_quote(glyph: char) -> StepKind {
    match glyph {
        '"' => StepKind::SelectorDoubleQuoteClose,
        _ => StepKind::SelectorDoubleQuote,
    }
}

fn get_kind_from_at_rule_quote(glyph: char) -> StepKind {
    match glyph {
        '\'' => StepKind::AtRuleQuoteClose,
        _ => StepKind::AtRuleQuote,
    }
}

fn get_kind_from_at_rule_double_quote(glyph: char) -> StepKind {
    match glyph {
        '"' => StepKind::AtRuleDoubleQuoteClose,
        _ => StepKind::AtRuleDoubleQuote,
    }
}
