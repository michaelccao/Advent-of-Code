use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day23.txt");

    let (robots, strongest) = get_robots(&data);

    let p1: u32 = robots_in_range(&robots, strongest);

    println!("{p1}");

    let p2: i32 = find_best_coords(&robots);

    println!("{p2}");
}

fn get_robots(data: &String) -> (Vec<Robot>, usize) {
    let mut robots: Vec<Robot> = Vec::new();

    let mut largest_radius: u32 = 0;
    let mut strongest_robot: usize = 0;

    for (i, line) in data.lines().enumerate() {
        let open_bracket = line.find("<").unwrap();
        let close_bracket = line.find(">").unwrap();

        let mut coords = line[open_bracket + 1..close_bracket].split(",");
        let x: i32 = coords.next().unwrap().parse().unwrap();
        let y: i32 = coords.next().unwrap().parse().unwrap();
        let z: i32 = coords.next().unwrap().parse().unwrap();

        let radius: u32 = line.split("r=").last().unwrap().parse().unwrap();

        robots.push(Robot {
            x: x,
            y: y,
            z: z,
            r: radius,
        });

        if radius > largest_radius {
            largest_radius = radius;
            strongest_robot = i;
        }
    }

    (robots, strongest_robot)
}

fn robots_in_range(robots: &Vec<Robot>, robot: usize) -> u32 {
    let r1: &Robot = &robots[robot];

    let mut in_radius: u32 = 0;

    for r2 in robots {
        if r1.distance_from_robot(r2) <= r1.r {
            in_radius += 1;
        }
    }

    in_radius
}

struct Partition {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
}

struct Robot {
    x: i32,
    y: i32,
    z: i32,
    r: u32,
}

impl Robot {
    fn intersects_partition(&self, p: &Partition) -> bool {
        let mut dist: u32 = 0;

        if self.x < p.x_min {
            dist += self.x.abs_diff(p.x_min);
        } else if self.x >= p.x_max {
            dist += self.x.abs_diff(p.x_max - 1);
        }

        if self.y < p.y_min {
            dist += self.y.abs_diff(p.y_min);
        } else if self.y >= p.y_max {
            dist += self.y.abs_diff(p.y_max - 1);
        }

        if self.z < p.z_min {
            dist += self.z.abs_diff(p.z_min);
        } else if self.z >= p.z_max {
            dist += self.z.abs_diff(p.z_max - 1);
        }

        dist <= self.r
    }

    fn distance_from_robot(&self, other: &Robot) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }
}

impl Partition {
    fn partition(&self) -> Vec<Partition> {
        let mut partitions: Vec<Partition> = Vec::new();

        let mut xvals: Vec<i32> = vec![self.x_min];
        if self.x_max - self.x_min > 1 {
            xvals.push((self.x_max + self.x_min) / 2);
        }
        xvals.push(self.x_max);

        let mut yvals: Vec<i32> = vec![self.y_min];
        if self.y_max - self.y_min > 1 {
            yvals.push((self.y_max + self.y_min) / 2);
        }
        yvals.push(self.y_max);

        let mut zvals: Vec<i32> = vec![self.z_min];
        if self.z_max - self.z_min > 1 {
            zvals.push((self.z_max + self.z_min) / 2);
        }
        zvals.push(self.z_max);

        for i in 0..xvals.len() - 1 {
            let xmin2: i32 = xvals[i];
            let xmax2: i32 = xvals[i + 1];
            for j in 0..yvals.len() - 1 {
                let ymin2: i32 = yvals[j];
                let ymax2: i32 = yvals[j + 1];

                for k in 0..zvals.len() - 1 {
                    let zmin2: i32 = zvals[k];
                    let zmax2: i32 = zvals[k + 1];

                    partitions.push(Partition {
                        x_min: xmin2,
                        x_max: xmax2,
                        y_min: ymin2,
                        y_max: ymax2,
                        z_min: zmin2,
                        z_max: zmax2,
                    });
                }
            }
        }

        partitions
    }
}

fn find_best_coords(robots: &Vec<Robot>) -> i32 {
    let mut x_min: i32 = robots[0].x;
    let mut x_max: i32 = robots[0].x;
    let mut y_min: i32 = robots[0].y;
    let mut y_max: i32 = robots[0].y;
    let mut z_min: i32 = robots[0].z;
    let mut z_max: i32 = robots[0].z;

    for robot in robots {
        x_min = x_min.min(robot.x);
        x_max = x_max.max(robot.x);
        y_min = y_min.min(robot.y);
        y_max = y_max.max(robot.y);
        z_min = z_min.min(robot.z);
        z_max = z_max.max(robot.z);
    }

    let partition = Partition {
        x_min,
        x_max,
        y_min,
        y_max,
        z_min,
        z_max,
    };

    let mut partitions: Vec<Partition> = partition.partition();

    while partitions.len() > 1 {
        let mut most_connections: u32 = 0;
        let mut best_partitions: Vec<usize> = Vec::new();

        for i in 0..partitions.len() {
            let mut robots_in_range: u32 = 0;

            for robot in robots {
                if robot.intersects_partition(&partitions[i]) {
                    robots_in_range += 1;
                }
            }

            if robots_in_range > most_connections {
                most_connections = robots_in_range;
                best_partitions = vec![i];
            } else if robots_in_range == most_connections {
                best_partitions.push(i);
            }
        }

        let mut partitions2 = Vec::new();
        for i in best_partitions {
            partitions2.append(&mut partitions[i].partition());
        }

        partitions = partitions2;
    }

    partitions[0].x_min.abs() + partitions[0].y_min.abs() + partitions[0].z_min.abs()
}
