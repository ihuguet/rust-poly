use crate::iface::{Iface, IfaceRef, IfaceTrait, IfaceType};

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

    fn into_iface(self) -> super::Iface {
        Iface::Ethernet(self)
    }

    // Cast to concrete type (option 1): use Any, better combined with `trait AsIfaceType`
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    // Cast to concrete type (option 2): one method per interface type
    fn as_ethernet_iface(&self) -> Result<&EthernetIface, &'static str> {
        Ok(self)
    }

    // Cast to concrete type (option 3): needs an `enum IfaceRef`
    fn as_iface_ref(&self) -> IfaceRef {
        IfaceRef::Ethernet(self)
    }

    // Find out the concrete type: using a separate enum IfaceType. Does not perform the type cast
    fn iface_type(&self) -> IfaceType {
        IfaceType::Ethernet
    }
}
