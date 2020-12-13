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
    optional_buses: Vec<Option<Bus>>,
}

impl BusScheduler {
    fn new(optional_buses: Vec<Option<Bus>>) -> Self {
        BusScheduler { optional_buses }
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
            .optional_buses
            .iter()
            .filter(|optional_bus| optional_bus.is_some())
            .map(|optional_bus| optional_bus.unwrap())
            .map(|bus| self.get_earliest_departure_for_bus_after(&bus, earliest_departure_time))
            .collect();

        self.earliest_departure(potential_departures)
    }

    fn earliest_timestamp_such_that_all_listed_buses_depart_at_offsets_matching_their_positions(
        &self,
    ) -> u64 {
        let timestamp_increment = self.optional_buses.first().unwrap().unwrap().id();
        let mut first_slot_timestamp = 0;

        loop {
            let consecutive_offset_found = self.optional_buses.iter().enumerate().fold(
                true,
                |accumulator, (index, optional_bus)| {
                    if accumulator && optional_bus.is_some() {
                        let target_timestamp = first_slot_timestamp + index as u64;
                        target_timestamp % optional_bus.unwrap().id() == 0
                    } else {
                        accumulator
                    }
                },
            );

            if consecutive_offset_found {
                break;
            } else {
                first_slot_timestamp += timestamp_increment;
            }
        }

        first_slot_timestamp
    }
}

fn bus_ids_from_string(string: &str) -> Result<Vec<Option<u64>>, ParseIntError> {
    string
        .split(',')
        .into_iter()
        .map(|split| match split {
            "x" => Ok(None),
            _ => match split.parse() {
                Ok(id) => Ok(Some(id)),
                Err(e) => Err(e),
            },
        })
        .collect()
}

fn time_and_buses_from_input_lines(
    input_strings: Vec<String>,
) -> anyhow::Result<(u64, Vec<Option<Bus>>)> {
    if input_strings.len() != 2 {
        Err(anyhow::Error::msg("Invalid string input"))
    } else {
        let earliest_departure_time: u64 = input_strings.get(0).unwrap().parse()?;
        let buses = bus_ids_from_string(input_strings.get(1).unwrap())?
            .iter()
            .copied()
            .map(|optional_id| match optional_id {
                Some(id) => Some(Bus::new(id)),
                None => None,
            })
            .collect();

        Ok((earliest_departure_time, buses))
    }
}

pub fn product_of_id_of_earliest_bus_and_wait_time(
    input_strings: Vec<String>,
) -> anyhow::Result<u64> {
    let (earliest_departure_time, buses) = time_and_buses_from_input_lines(input_strings)?;
    let bus_scheduler = BusScheduler::new(buses);
    let earliest_departure =
        bus_scheduler.get_bus_earliest_possible_departure(earliest_departure_time);

    Ok(earliest_departure.bus().id() * (earliest_departure.time() - earliest_departure_time))
}

pub fn earliest_timestamp_such_that_all_listed_buses_depart_at_offsets_matching_their_positions(
    input_strings: Vec<String>,
) -> anyhow::Result<u64> {
    let (_, optional_buses) = time_and_buses_from_input_lines(input_strings)?;
    let bus_scheduler = BusScheduler::new(optional_buses);
    Ok(bus_scheduler
        .earliest_timestamp_such_that_all_listed_buses_depart_at_offsets_matching_their_positions())
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

        assert_that(&product_of_id_of_earliest_bus_and_wait_time(input).unwrap()).is_equal_to(295);
    }

    #[test]
    fn finds_earliest_timestamp_such_that_all_listed_buses_depart_at_offsets_matching_their_positions(
    ) {
        let input = vec!["939", "7,13,x,x,59,x,31,19"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>();

        assert_that(&earliest_timestamp_such_that_all_listed_buses_depart_at_offsets_matching_their_positions(input).unwrap())
            .is_equal_to(1068781);
    }
}
