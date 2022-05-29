use smart_home::{device, home::Home, room::Room};

fn main() {
    let smart_socket = device::smart_socket::SmartSocket::new("#1", 24.);
    let thermometer = device::thermometer::Thermometer::new("#2");
    let mut home = Home::new("my home");
    println!("Home: `{}`", home.get_name());
    let mut kitchen = Room::new("kitchen");
    kitchen.add_device(smart_socket).unwrap();
    kitchen.add_device(thermometer).unwrap();
    home.add_room(kitchen).unwrap();
    println!("{}", home.report());
}
