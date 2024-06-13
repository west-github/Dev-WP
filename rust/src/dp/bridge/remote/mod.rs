use derive_new::new;

trait Device {
    fn is_enabled(&self) -> bool;
    fn print_status(&self);
}

trait Remote<D>
where
    D: Device,
{
    fn get_device(&mut self) -> &mut D;

    fn power(&mut self) {
        if self.get_device().is_enabled() {
            println!("Turned off");
            return;
        }

        println!("Turned on")
    }

    fn get_status(&mut self) {
        self.get_device().print_status();
    }
}

#[derive(new)]
struct Radio {
    enabled: bool,
}

impl Device for Radio {
    fn is_enabled(&self) -> bool {
        return self.enabled;
    }

    fn print_status(&self) {
        if self.enabled {
            println!("I'm working");
            return;
        }

        println!("I'm turned off");
    }
}

#[derive(new)]
struct BasicRemote<D: Device> {
    device: D,
}

impl<D: Device> Remote<D> for BasicRemote<D> {
    fn get_device(&mut self) -> &mut D {
        &mut self.device
    }
}

#[test]
fn use_device() {
    let mut remote = BasicRemote::new(Radio::new(true));

    remote.get_status();
}
