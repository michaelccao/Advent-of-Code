use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day13.txt");

    let p1: u32 = earliest_bus(&data);

    println!("{p1}");

    let p2: u64 = earliest_sync(&data);

    println!("{p2}");
}

fn earliest_bus(data: &String) -> u32 {
    let data: Vec<&str> = data.lines().collect();

    let earliest_time: u32 = data[0].parse().unwrap();

    let mut earliest_bus_time: u32 = u32::MAX;
    let mut earliest_bus: u32 = 0;

    for bus in data[1].split(",") {
        if bus != "x" {
            let bus: u32 = bus.parse().unwrap();

            if earliest_time % bus == 0 {
                return 0
            }

            let bus_time = (earliest_time/bus + 1)*bus;

            if bus_time < earliest_bus_time {
                earliest_bus_time = bus_time;
                earliest_bus = bus;
            }
        }
    }

    (earliest_bus_time - earliest_time)*earliest_bus
    
}

fn earliest_sync(data: &String) -> u64 {

    let data: Vec<&str> = data.lines().collect();

    let mut buses: Vec<(u64, u64)> = Vec::new();

    for (t, bus) in data[1].split(",").enumerate() {
        if bus != "x" {
            buses.push((bus.parse().unwrap(), t as u64))
        }
    }

    let mut best_time: (u64, u64) = (0, buses[0].0);

    for i in 1..buses.len() {
        let (bus, offset) = buses[i];
        while (best_time.0 + offset) % bus != 0 {
            best_time.0 += best_time.1;
        }

        best_time.1 = best_time.1*bus;
    }

    best_time.0
}