enum Intel {
    I3,
    I5,
    I7,
}
#[derive(Debug)]
enum AmdCpu {
    Ryzen3,
    Ryzen5,
    Ryzen7,
}

enum Apple {
    M1,
    M1Pro,
    M1ProMax,
}
#[derive(Debug)]
enum CPU {
    Intel,
    Amd(AmdCpu),
    Apple,
}

enum RTX {
    RTX3060,
    RTX3070,
    RTX3080,
    RTX3090,
}

#[derive(Debug)]
enum AmdGpu {
    RX6600XT,
    RX6000,
}

#[derive(Debug)]
enum GPU {
    RTX,
    Amd(AmdGpu),
    Apple,
}

enum Power {
    Off,
    On,
}

struct Computer {
    cpu: CPU,
    gpu: GPU,
    power: Power,

}
impl Computer {
    fn run(&self) -> Result<(), &'static str> {
        match &self.power {
            Power::Off => { 
                return Err("Computer has not been turned on. It cannot be run");
            }
            Power::On => {
                println!("Computer CPU {:?}, GPU {:?} has been running", &self.cpu, &self.gpu);
                return Ok(());
            }
        }
    }
    fn on(mut self) -> Result<(), &'static str> {
        match &self.power {
            Power::On => { 
                return Err("Computer has been already turned on.");
            }
            Power::Off => {
                self.power = Power::On; 
                return Ok(()); 
            }
        }
    }
    fn off(mut self) -> Result<(), &'static str> {
        match &self.power {
            Power::On => {
                self.power = Power::Off;
                return Ok(());
            }
            Power::Off =>  { 
                return Err("Computer has been already turn off.");
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
