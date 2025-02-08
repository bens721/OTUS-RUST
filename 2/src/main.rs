use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut home = SMHome {
        name: "HOME_01".to_string(),
        devices: Vec::with_capacity(2),
    };
    println!("Home:{} startup..", home.name);
    println!("Home:{} check saved devices..", home.name);

    home.add_device(Box::new(SMSock {
        model_name: "Smart Socket-01".to_string(),
        mac_addr: "00:11:22:33:44:55".to_string(),
        messure_val: 0.0,
        is_on: false,
    }));

    home.add_device(Box::new(SMTerm {
        model_name: "Smart Thermometer-01".to_string(),
        mac_addr: "11:22:33:44:55:66".to_string(),
        messure_val: 22.5,
        is_on: false,
    }));
    home.home_info();

    for device in home.devices.iter_mut() {
        device.power_on();
    }
    let mut cnt = 0;
    let mut cnt_exit = 0;
    let a = loop {
        if cnt % 10 == 0 {
            if cnt_exit == 2 {
                println!("Shutdown {}", home.name);
                break 1;
            }
            cnt_exit += 1;
        }
        for device in home.devices.iter() {
            device.make_measure();
        }
        let time = Duration::new(1, 0);

        sleep(time);
        cnt += 1;
    };
    if a == 1 {
        for device in home.devices.iter_mut() {
            device.power_off();
        }
        home.remove_devices();
        exit(0);
    }
    exit(1);
}

struct SMHome {
    name: String,
    devices: Vec<Box<dyn SmartDevice>>,
}

impl SMHome {
    fn home_info(&self) {
        for device in &self.devices {
            println!("Device {:?}", device);
        }
        println!("Home:{} device count: {}", &self.name, &self.devices.len());
    }

    fn add_device(&mut self, device: Box<dyn SmartDevice>) {
        self.devices.push(device);
    }

    fn remove_devices(&mut self) {
        self.devices.pop();
    }
}

trait SmartDevice: std::fmt::Debug {
    fn power_on(&mut self);
    fn power_off(&mut self);
    fn make_measure(&self);
}

//sock
#[derive(Debug)]
struct SMSock {
    model_name: String,
    mac_addr: String,
    messure_val: f64,
    is_on: bool,
}

impl SmartDevice for SMSock {
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
}
