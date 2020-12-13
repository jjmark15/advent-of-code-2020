use std::num::ParseIntError;

#[derive(Copy, Clone)]
struct Bus {
    id: u64,
}

impl Bus {
    fn new(id: u64) -> Self {
        Bus { id }
    }

    fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Copy, Clone)]
struct BusDeparture {
    time: u64,
    bus: Bus,
}

impl BusDeparture {
    fn new(time: u64, bus: Bus) -> Self {
        BusDeparture { time, bus }
    }

    fn time(&self) -> u64 {
        self.time
    }

    fn bus(&self) -> &Bus {
        &self.bus
    }
}

struct BusScheduler {
    buses: Vec<Bus>,
}

impl BusScheduler {
    fn new(buses: Vec<Bus>) -> Self {
        BusScheduler { buses }
    }

    fn get_earliest_departure_for_bus_after(&self, bus: &Bus, earliest_time: u64) -> BusDeparture {
        let mut departure_time = 0;
        while departure_time < earliest_time {
            departure_time += bus.id();
        }
        BusDeparture::new(departure_time, *bus)
    }

    fn earliest_departure(&self, mut departures: Vec<BusDeparture>) -> BusDeparture {
        departures.sort_by_key(|&a| a.time());
        *departures.first().unwrap()
    }

    fn get_bus_earliest_possible_departure(&self, earliest_departure_time: u64) -> BusDeparture {
        let potential_departures: Vec<BusDeparture> = self
            .buses
            .iter()
            .map(|bus| self.get_earliest_departure_for_bus_after(bus, earliest_departure_time))
            .collect();

        self.earliest_departure(potential_departures)
    }
}

fn bus_ids_from_string(string: &str) -> Result<Vec<u64>, ParseIntError> {
    string
        .split(',')
        .into_iter()
        .filter(|split| split != &"x")
        .map(|split| split.parse())
        .collect()
}

fn time_and_buses_from_input_lines(input_strings: Vec<String>) -> anyhow::Result<(u64, Vec<Bus>)> {
    if input_strings.len() != 2 {
        Err(anyhow::Error::msg("Invalid string input"))
    } else {
        let earliest_departure_time: u64 = input_strings.get(0).unwrap().parse()?;
        let buses = bus_ids_from_string(input_strings.get(1).unwrap())?
            .iter()
            .copied()
            .map(Bus::new)
            .collect();

        Ok((earliest_departure_time, buses))
    }
}

pub fn get_product_of_id_of_earliest_bus_and_wait_time(
    input_strings: Vec<String>,
) -> anyhow::Result<u64> {
    let (earliest_departure_time, buses) = time_and_buses_from_input_lines(input_strings)?;
    let bus_scheduler = BusScheduler::new(buses);
    let earliest_departure =
        bus_scheduler.get_bus_earliest_possible_departure(earliest_departure_time);

    Ok(earliest_departure.bus().id() * (earliest_departure.time() - earliest_departure_time))
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::*;

    #[test]
    fn gets_product_of_id_of_earliest_bus_and_bus_stop_wait_time() {
        let input = vec!["939", "7,13,x,x,59,x,31,19"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>();

        assert_that(&get_product_of_id_of_earliest_bus_and_wait_time(input).unwrap())
            .is_equal_to(295);
    }
}
