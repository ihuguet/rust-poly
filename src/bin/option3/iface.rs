mod ethernet;
mod vlan;
pub use ethernet::EthernetIface;
pub use vlan::VlanIface;

use enum_dispatch::enum_dispatch;

// This example uses enum dispatch to allow calling a trait's method on the
// right enum variant without having to manually do the `match` for each one.
// Instead of storing `&dyn IfaceTrait` in a container, you can store the `Iface`
// and call directly to the trait's method.

#[enum_dispatch]
pub enum Iface {
    Ethernet(EthernetIface),
    Vlan(VlanIface),
}

#[enum_dispatch(Iface)]
pub trait IfaceTrait {
    fn name(&self) -> &str;
}
