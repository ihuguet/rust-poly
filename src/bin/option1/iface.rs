mod ethernet;
mod vlan;
pub use ethernet::EthernetIface;
pub use vlan::VlanIface;

use std::any::Any;

pub enum IfaceType {
    Ethernet,
    Vlan,
}

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

    fn into_iface(self) -> Iface;

    // Cast to concrete type (option 1): use Any, better combined with `trait AsIfaceType`
    fn as_any(&self) -> &dyn Any;

    // Cast to concrete type (option 2): one method per interface type
    fn as_vlan_iface(&self) -> Result<&VlanIface, &'static str> {
        Err("Not a VLAN interface")
    }
    fn as_ethernet_iface(&self) -> Result<&EthernetIface, &'static str> {
        Err("Not an Ethernet interface")
    }

    // Cast to concrete type (option 3): needs an `enum IfaceRef`
    fn as_iface_ref(&self) -> IfaceRef;

    // Find out the concrete type: using a separate enum IfaceType. Does not perform the type cast
    fn iface_type(&self) -> IfaceType;
}

pub trait AsIfaceType {
    fn as_iface_type<T: IfaceTrait>(&self) -> Option<&T> {
        None
    }
}

impl AsIfaceType for &dyn IfaceTrait {
    fn as_iface_type<T: IfaceTrait>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }
}
