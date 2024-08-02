mod ethernet;
mod vlan;
pub use ethernet::EthernetIface;
pub use vlan::VlanIface;

use std::any::Any;

pub enum Iface {
    Ethernet(EthernetIface),
    Vlan(VlanIface),
}

pub enum IfaceRef<'a> {
    Ethernet(&'a EthernetIface),
    Vlan(&'a VlanIface),
}

pub trait IfaceTrait: Any {
    fn name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;

    // This is only useful for &dyn IfaceTrait, but implemented here for performance
    // and being more convenient.
    fn as_iface_ref(&self) -> IfaceRef;
}

// In this example, as_iface_type and as_*_iface are not part of the trait, but
// part of the implementation of the trait object.
// That means that these methods are available in a `&dyn IfaceTrait`, but not
// on the concrete types that implement IfaceTrait.
// Rationale: if you have a variable of type `&dyn IfaceTrait` you may want to
// find out its concrete type. However, if you have a variable of the concrete
// type, you already know it, so you don't need these methods.
impl dyn IfaceTrait {
    pub fn as_iface_type<T: IfaceTrait>(&self) -> Result<&T, &'static str> {
        self.as_any()
            .downcast_ref()
            .ok_or("Interface type mismatch")
    }

    pub fn as_vlan_iface(&self) -> Result<&VlanIface, &'static str> {
        self.as_iface_type()
            .map_err(|_| "Interface type is not VLAN")
    }

    pub fn as_ethernet_iface(&self) -> Result<&EthernetIface, &'static str> {
        self.as_iface_type()
            .map_err(|_| "Interface type is not Ethernet")
    }
}
