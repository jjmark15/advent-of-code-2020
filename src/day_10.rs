use std::ops::{Add, Sub};
use std::str::FromStr;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Joltage(i64);

impl Add for Joltage {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Joltage(self.0 + rhs.0)
    }
}

impl Sub for Joltage {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Joltage(self.0 - rhs.0)
    }
}

#[derive(Copy, Clone)]
struct JoltageAdapter {
    output_joltage: Joltage,
}

impl FromStr for JoltageAdapter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let output_voltage: Joltage = Joltage(s.parse()?);
        Ok(JoltageAdapter::new(output_voltage))
    }
}

impl JoltageAdapter {
    fn new(output_joltage: Joltage) -> Self {
        JoltageAdapter { output_joltage }
    }

    fn supports_input_joltage(&self, input_joltage: Joltage) -> bool {
        let lower_bound = self.output_joltage - Joltage(3);
        let upper_bound = self.output_joltage;

        input_joltage >= lower_bound && input_joltage < upper_bound
    }

    fn output_joltage(&self) -> Joltage {
        self.output_joltage
    }
}

struct JoltageAdapterChain {
    adapters: Vec<JoltageAdapter>,
}

impl JoltageAdapterChain {
    fn new(adapters: Vec<JoltageAdapter>) -> Self {
        JoltageAdapterChain { adapters }
    }

    fn joltage_differences(&self) -> Vec<Joltage> {
        self.adapters
            .iter()
            .map(JoltageAdapter::output_joltage)
            .enumerate()
            .map(|(index, joltage)| {
                let previous_joltage;
                if index == 0 {
                    previous_joltage = Joltage(0);
                } else {
                    previous_joltage = self.adapters.get(index - 1).unwrap().output_joltage();
                }
                joltage - previous_joltage
            })
            .collect()
    }
}

struct JoltageAdapterChainBuilder;

impl JoltageAdapterChainBuilder {
    fn new() -> Self {
        JoltageAdapterChainBuilder
    }

    fn build_chain_using_every_adapter(
        &self,
        mut joltage_adapters: Vec<JoltageAdapter>,
    ) -> anyhow::Result<JoltageAdapterChain> {
        joltage_adapters.sort_by_key(|&a| a.output_joltage());

        let mut chain_adapters: Vec<JoltageAdapter> = vec![];
        let chain_building_result =
            joltage_adapters
                .iter()
                .try_for_each(|adapter| match chain_adapters.last() {
                    Some(previous_adapter) => {
                        if adapter.supports_input_joltage(previous_adapter.output_joltage()) {
                            Ok(chain_adapters.push(*adapter))
                        } else {
                            Err(anyhow::Error::msg("Could not use all adapters in chain"))
                        }
                    }
                    None => Ok(chain_adapters.push(*adapter)),
                });

        match chain_building_result {
            Ok(_) => Ok(JoltageAdapterChain::new(chain_adapters)),
            Err(e) => Err(e),
        }
    }
}

fn calculate_built_in_joltage_adapter(joltage_adapters: &[JoltageAdapter]) -> JoltageAdapter {
    let output_joltage = joltage_adapters
        .iter()
        .map(JoltageAdapter::output_joltage)
        .max()
        .unwrap_or(Joltage(0))
        + Joltage(3);
    JoltageAdapter::new(output_joltage)
}

pub fn product_of_1_and_3_joltage_differences_using_every_adapter_and_built_in(
    joltage_adapter_strings: Vec<String>,
) -> anyhow::Result<u64> {
    let mut joltage_adapters = joltage_adapter_strings
        .iter()
        .map(|s| s.parse())
        .collect::<anyhow::Result<Vec<JoltageAdapter>>>()?;
    joltage_adapters.push(calculate_built_in_joltage_adapter(&joltage_adapters));

    let joltage_adapter_chain_builder = JoltageAdapterChainBuilder::new();

    let joltage_differences = joltage_adapter_chain_builder
        .build_chain_using_every_adapter(joltage_adapters)?
        .joltage_differences();

    let joltage_difference_of_3_count = joltage_differences
        .iter()
        .filter(|difference| difference.0 == 3)
        .count();
    let joltage_difference_of_1_count = joltage_differences
        .iter()
        .filter(|difference| difference.0 == 1)
        .count();

    Ok((joltage_difference_of_1_count * joltage_difference_of_3_count) as u64)
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn calculates_product_of_1_and_3_joltage_differences_using_every_adapter_and_built_in() {
        let joltage_adapter_strings =
            vec!["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>();

        assert_that(
            &product_of_1_and_3_joltage_differences_using_every_adapter_and_built_in(
                joltage_adapter_strings,
            )
            .unwrap(),
        )
        .is_equal_to(35);
    }
}
