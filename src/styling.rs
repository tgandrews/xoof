use std::collections::HashMap;

use cssom::*;
use dom::*;

pub(crate) type PropertyMap = HashMap<String, String>;
pub(crate) struct StyledNode<'a> {
    node: &'a Node, // pointer to a DOM node
    specified_values: PropertyMap,
    children: Vec<StyledNode<'a>>,
}

pub(crate) fn apply_styling<'a>(dom: &'a Node, style_sheet: &StyleSheet) -> StyledNode<'a> {
    let rules = &style_sheet.rules;
    style_node(dom, rules)
}

fn style_node<'a>(node: &'a Node, rules: &Vec<Rule>) -> StyledNode<'a> {
    StyledNode {
        node,
        specified_values: match node.node_type {
            NodeType::Element(ref element) => build_style(element, &rules),
            _ => PropertyMap::new(),
        },
        children: node
            .children
            .iter()
            .map(|child| style_node(child, &rules))
            .collect(),
    }
}

type MatchedRule<'a> = (Specificity, &'a Rule);

fn match_rule<'a>(element: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    rule.selectors
        .iter()
        .find(|selector| selector.matches(element))
        .map(|selector| (selector.specificity(), rule))
}

fn determine_matching_rules<'a>(
    element: &ElementData,
    rules: &'a Vec<Rule>,
) -> Vec<MatchedRule<'a>> {
    let res = rules
        .iter()
        .filter_map(|rule| match_rule(element, rule))
        .collect();
    res
}

fn build_style(element: &ElementData, rules: &Vec<Rule>) -> PropertyMap {
    let mut styles = PropertyMap::new();

    let mut matching_rules = determine_matching_rules(element, &rules);

    matching_rules.sort_by(|(a, _), (b, _)| {
        let (result, _) = a.cmp(&b);
        result
    });

    for (_, rule) in matching_rules {
        for declaration in &rule.declarations {
            let name = declaration.name.clone();
            let value = declaration.value.clone();
            styles.insert(name, value);
        }
    }

    styles
}
