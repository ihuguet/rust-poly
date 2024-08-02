use crate::iface::IfaceTrait;

pub struct EthernetIface {}

impl EthernetIface {
    pub fn new() -> Self {
        EthernetIface {}
    }

    pub fn do_ethernet_only_action(&self) {
        println!("This is an action exclusive of Ethernet");
    }
}

impl IfaceTrait for EthernetIface {
    fn name(&self) -> &str {
        "ethernet"
    }
}
