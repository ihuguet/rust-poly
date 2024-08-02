use crate::iface::IfaceTrait;

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
}
