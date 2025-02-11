use std::process::exit;
//use std::thread::sleep;
//use std::time::Duration;

fn main() {
    let mut home: SMHomeItem = SMHome::new("Home-01");
    let mut room: SMRoom = SMRoom::new(&home.name, 01);
    let term = Box::new(SMTerm::new(
        "Smart Thermometer-01",
        "11:22:33:44:55:66",
        22.5,
    ));
    let sock = Box::new(SMSock::new("Smart Socket-01", "00:11:22:33:44:55", 0.0));
    room.add_device(term);
    room.add_device(sock);
    home.add_room(room);
    home.home_info();
    //home.remove_room();
    exit(0);
}

/*trait DeviceInfoProvider {
    fn get_device_state_by_name(room_name: &str, device_name: &str);
}*/

trait SMHome {
    fn new(name: &str) -> Self;
    //fn get_rooms(&self) -> Vec<SmartDevice>;
    // fn devices(&self, room: &str) -> [&str; 3];
    // fn create_report(&self,SMRoom) -> String;
    fn home_info(&self);
    fn add_room(&mut self, room: SMRoom);
    fn remove_room(&mut self);
}

//SmartHome
struct SMHomeItem {
    name: String,
    rooms: Vec<SMRoom>,
}

impl SMHome for SMHomeItem {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rooms: Vec::with_capacity(4),
        }
    }

    fn home_info(&self) {
        for room in &self.rooms {
            println!("Rooms {:?}", room);
        }
        println!("Home:{} rooms count: {}", &self.name, &self.rooms.len());
    }

    fn add_room(&mut self, room: SMRoom) {
        println!("{}", room.name);
        for _item in &room.devices {
            println!("Devices");
        }
        self.rooms.push(room);
    }

    fn remove_room(&mut self) {
        self.rooms.pop();
    }

    //fn get_rooms(&self) -> [&str; 2];
    //fn devices(&self, room: &str) -> [&str; 3];
    //fn create_report(&self,SMDevice) -> String;
}
//Smart room
#[derive(Debug)]
struct SMRoom {
    name: String,
    devices: Vec<Box<dyn SmartDevice>>,
}
impl SMRoom {
    fn new(home_name: &str, room_number: u32) -> Self {
        Self {
            name: format!("{}/{}", home_name, room_number.to_string()),
            devices: Vec::with_capacity(2),
        }
    }

    fn add_device(&mut self, device: Box<dyn SmartDevice>) {
        println!("{} adding device", &self.name);
        self.devices.push(device);
    }
}

//Smart device
trait SmartDevice: std::fmt::Debug {
    fn new(model_name: &str, mac_addr: &str, messure_val: f64) -> Self
    where
        Self: Sized;
    fn power_on(&mut self);
    fn power_off(&mut self);
    fn make_measure(&self);
    fn info(&self);
}
//SmartDevice
//sock
#[derive(Debug)]
struct SMSock {
    model_name: String,
    mac_addr: String,
    messure_val: f64,
    is_on: bool,
}

impl SmartDevice for SMSock {
    fn new(model_name: &str, mac_addr: &str, messure_val: f64) -> Self {
        Self {
            model_name: model_name.to_string(),
            mac_addr: mac_addr.to_string(),
            messure_val,
            is_on: false,
        }
    }

    fn power_on(&mut self) {
        self.is_on = true;
        println!(
            "Socket device name:{} MAC{} is powered up",
            &self.model_name, &self.mac_addr
        );
    }

    fn power_off(&mut self) {
        self.is_on = false;
        println!("Socket is powered off");
    }

    fn make_measure(&self) {
        println!("Current power: {}", self.messure_val);
    }
    fn info(&self) {
        println!("DeviceInfo: {}", &self.model_name);
    }
}

//term
#[derive(Debug)]
struct SMTerm {
    model_name: String,
    mac_addr: String,
    messure_val: f64,
    is_on: bool,
}

impl SmartDevice for SMTerm {
    fn new(model_name: &str, mac_addr: &str, messure_val: f64) -> Self {
        Self {
            model_name: model_name.to_string(),
            mac_addr: mac_addr.to_string(),
            messure_val,
            is_on: false,
        }
    }

    fn power_on(&mut self) {
        self.is_on = true;
        println!(
            "Term device name:{} MAC:{} is powered up",
            &self.model_name, &self.mac_addr
        );
    }

    fn power_off(&mut self) {
        self.is_on = false;
        println!("Term device is powered off");
    }

    fn make_measure(&self) {
        println!("Current temperature: {}", self.messure_val);
    }
    fn info(&self) {
        println!("DeviceInfo: {}", &self.model_name);
    }
}
