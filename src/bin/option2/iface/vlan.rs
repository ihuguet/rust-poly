use crate::iface::{Iface, IfaceRef, IfaceTrait};

pub struct VlanIface {}

impl VlanIface {
    pub fn new() -> Self {
        VlanIface {}
    }

    pub fn do_vlan_only_action(&self) {
        println!("This is an action exclusive of VLAN");
    }
}

impl IfaceTrait for VlanIface {
    fn name(&self) -> &str {
        "vlan"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_iface_ref(&self) -> IfaceRef {
        IfaceRef::Vlan(self)
    }
}

impl From<VlanIface> for Iface {
    fn from(value: VlanIface) -> Self {
        Iface::Vlan(value)
    }
}
