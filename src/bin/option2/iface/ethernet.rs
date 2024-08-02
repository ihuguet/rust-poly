use crate::iface::{Iface, IfaceRef, IfaceTrait};

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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_iface_ref(&self) -> IfaceRef {
        IfaceRef::Ethernet(self)
    }
}

impl From<EthernetIface> for Iface {
    fn from(value: EthernetIface) -> Self {
        Iface::Ethernet(value)
    }
}
