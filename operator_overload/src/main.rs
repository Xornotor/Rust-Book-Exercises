use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Millimeters(f64);

#[derive(Debug, Copy, Clone)]
struct Meters(f64);

impl Add<Meters> for Millimeters{
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000.0))
    }
}

impl Add<Millimeters> for Meters{
    type Output = Meters;

    fn add(self, other: Millimeters) -> Meters {
        Meters(self.0 + (other.0 / 1000.0))
    }
}

impl Add<Meters> for Meters{
    type Output = Meters;

    fn add(self, other: Meters) -> Meters {
        Meters(self.0 + other.0)
    }
}


impl Add<Millimeters> for Millimeters{
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 + other.0)
    }
}

fn main(){
    let meter_1 = Meters(2.5);
    let meter_2 = Meters(3.2);
    let mil_1 = Millimeters(5.2);
    let mil_2 = Millimeters(30.0);

    println!("Meters + Meters = {:?}", (meter_1 + meter_2).0);
    println!("Mil + Mil = {:?}", (mil_1 + mil_2).0);
    println!("Mil + Meters = {:?}", (mil_1 + meter_2).0);
    println!("Meters + Mil = {:?}", (meter_1 + mil_2).0);
}