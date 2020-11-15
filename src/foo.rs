use super::registers::port;

pub struct Port;
impl Port {
    fn device_is_present(&self) -> bool {
        self.parse_port_rg(|r| {
            let tfd = 3;
            let ssts = r.ssts.read();

            !tfd.busy()
                && (ssts.device_detection() =
                    3 || [2, 6, 8].contains(ssts.interface_power_management()))
        })
    }
    fn parse_port_rg<T>(&self, f: T)
    where
        T: Fn(&port::Registers),
    {
    }
}
