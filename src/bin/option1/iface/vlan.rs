use crate::iface::{Iface, IfaceRef, IfaceTrait, IfaceType};

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

    // Cast to concrete type (option 1): use Any, better combined with `trait AsIfaceType`
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    // Cast to concrete type (option 2): one method per interface type
    fn as_vlan_iface(&self) -> Result<&VlanIface, &'static str> {
        Ok(self)
    }

    // Cast to concrete type (option 3): needs an `enum IfaceRef`
    fn as_iface_ref(&self) -> IfaceRef {
        IfaceRef::Vlan(self)
    }

    // Find out the concrete type: using a separate enum IfaceType. Does not perform the type cast
    fn iface_type(&self) -> IfaceType {
        IfaceType::Vlan
    }
}

impl From<VlanIface> for Iface {
    fn from(value: VlanIface) -> Self {
        Iface::Vlan(value)
    }
}
