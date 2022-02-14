use cssom::*;
use dom::*;

pub fn apply_styling(dom: &Node, style_sheet: &StyleSheet) -> Node {
    let rules = &style_sheet.rules;
    style_node(dom, rules)
}

fn style_node(node: &Node, rules: &Vec<Rule>) -> Node {
    match &node.node_type {
        NodeType::Element(element_data) => Node {
            node_type: NodeType::Element(ElementData {
                tag_name: element_data.tag_name.clone(),
                attributes: element_data.attributes.clone(),
                style_values: build_style(&element_data, &rules),
            }),
            children: node
                .children
                .iter()
                .map(|child| style_node(child, &rules))
                .collect(),
        },
        _ => node.clone(),
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
