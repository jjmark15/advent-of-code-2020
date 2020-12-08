use std::collections::HashMap;
use std::str::FromStr;

use lazy_static::lazy_static;
use petgraph::prelude::{Dfs, NodeIndex};
use petgraph::Direction::Incoming;
use petgraph::Graph;
use regex::Regex;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Bag(String);

impl Bag {
    fn new(style: String) -> Self {
        Bag(style)
    }
}

#[cfg_attr(test, derive(Debug))]
struct BagQuantity {
    bag: Bag,
    count: usize,
}

impl BagQuantity {
    fn new(bag: Bag, count: usize) -> Self {
        BagQuantity { bag, count }
    }

    fn bag(&self) -> &Bag {
        &self.bag
    }

    fn count(&self) -> usize {
        self.count
    }
}

#[cfg_attr(test, derive(Debug))]
struct BagContainerRule {
    bag: Bag,
    contained: Vec<BagQuantity>,
}

impl BagContainerRule {
    fn parent(&self) -> &Bag {
        &self.bag
    }

    fn contained(&self) -> &Vec<BagQuantity> {
        &self.contained
    }
}

impl FromStr for BagContainerRule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PARENT_STYLE_REGEX: Regex =
                Regex::new(r"^(?P<style>\w+ \w+) bags contain").unwrap();
            static ref CHILD_STYLE_REGEX: Regex =
                Regex::new(r"(?P<style_count>\d+) (?P<style>\w+ \w+) bags?").unwrap();
        }

        let parent_bag = Bag::new(
            PARENT_STYLE_REGEX
                .captures(s)
                .ok_or_else(|| anyhow::Error::msg("Could not parse bag container rule"))?
                .name("style")
                .unwrap()
                .as_str()
                .to_string(),
        );

        let contained_bags: Vec<BagQuantity> = CHILD_STYLE_REGEX
            .captures_iter(s)
            .map(|captures| {
                BagQuantity::new(
                    Bag::new(captures.name("style").unwrap().as_str().to_string()),
                    captures
                        .name("style_count")
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap(),
                )
            })
            .collect();

        Ok(BagContainerRule {
            bag: parent_bag,
            contained: contained_bags,
        })
    }
}

struct BagRuleWalker {
    rules: Vec<BagContainerRule>,
}

impl BagRuleWalker {
    fn new(rules: Vec<BagContainerRule>) -> Self {
        BagRuleWalker { rules }
    }

    fn build_rule_graph(&self) -> (Graph<Bag, usize>, HashMap<Bag, NodeIndex>) {
        let mut bag_node_indexes: HashMap<Bag, NodeIndex> = HashMap::new();
        let mut graph: Graph<Bag, usize> = Graph::new();

        self.rules.iter().for_each(|rule| {
            let parent = rule.parent();
            if !bag_node_indexes.contains_key(parent) {
                bag_node_indexes.insert(parent.clone(), graph.add_node(parent.clone()));
            }

            rule.contained().iter().for_each(|quantity| {
                let bag = quantity.bag();
                if !bag_node_indexes.contains_key(bag) {
                    bag_node_indexes.insert(bag.clone(), graph.add_node(bag.clone()));
                }

                graph.add_edge(
                    *bag_node_indexes.get(parent).unwrap(),
                    *bag_node_indexes.get(bag).unwrap(),
                    quantity.count(),
                );
            });
        });

        (graph, bag_node_indexes)
    }

    fn count_bags_that_eventually_contain(&self, bag: Bag) -> usize {
        let (mut graph, bag_node_indexes) = self.build_rule_graph();

        let mut count = 0;

        graph.reverse();

        let mut search = Dfs::new(&graph, *bag_node_indexes.get(&bag).unwrap());
        while let Some(_node) = search.next(&graph) {
            count += 1;
        }

        count - 1
    }

    fn count_nested_bags(&self, bag: Bag) -> usize {
        let (mut graph, bag_node_indexes) = self.build_rule_graph();

        let mut count = 0;

        graph.reverse();

        let mut search = Dfs::new(&graph, *bag_node_indexes.get(&bag).unwrap());
        while let Some(node) = search.next(&graph) {
            let mut edges = graph.neighbors_directed(node, Incoming).detach();
            while let Some(edge) = edges.next_edge(&graph) {
                count += graph[edge];
            }
        }

        count
    }
}

pub fn count_bags_that_eventually_contain(
    bag_rule_strings: Vec<String>,
    bag_style: &str,
) -> anyhow::Result<usize> {
    let bag_rules: Vec<BagContainerRule> = bag_rule_strings
        .iter()
        .map(|s| s.parse())
        .collect::<anyhow::Result<Vec<BagContainerRule>>>()?;
    let bag_rule_walker = BagRuleWalker::new(bag_rules);

    Ok(bag_rule_walker.count_bags_that_eventually_contain(Bag::new(bag_style.to_string())))
}

pub fn count_bags_nested_inside_a_bag(
    bag_rule_strings: Vec<String>,
    bag_style: &str,
) -> anyhow::Result<usize> {
    let bag_rules: Vec<BagContainerRule> = bag_rule_strings
        .iter()
        .map(|s| s.parse())
        .collect::<anyhow::Result<Vec<BagContainerRule>>>()?;
    let bag_rule_walker = BagRuleWalker::new(bag_rules);

    Ok(bag_rule_walker.count_nested_bags(Bag::new(bag_style.to_string())))
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn counts_bags_that_eventually_contain_a_bag() {
        let rules: Vec<String> = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain 2 polka dot bags.",
            "dotted black bags contain no other bags.",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();

        assert_that(&count_bags_that_eventually_contain(rules, "shiny gold").unwrap())
            .is_equal_to(4);
    }

    #[test]
    fn counts_how_many_bags_are_nested_inside_a_bag() {
        let rules = vec![
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();

        assert_that(&count_bags_nested_inside_a_bag(rules, "shiny gold").unwrap()).is_equal_to(126);
    }
}
