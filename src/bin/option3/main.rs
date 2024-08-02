mod iface;
use iface::{EthernetIface, Iface, IfaceTrait, VlanIface};

fn main() {
    let vlan_iface = VlanIface::new();
    println!("{}", vlan_iface.name());
    vlan_iface.do_vlan_only_action();

    let ethernet_iface = EthernetIface::new();
    println!("{}", ethernet_iface.name());
    ethernet_iface.do_ethernet_only_action();

    // enum_dispatch automatically implements `From<InnerType> for EnumType`
    let iface: Iface = vlan_iface.into();

    // enum_dispatch automatically implements the trait, calling to the inner type method
    println!("{}", iface.name());

    // we can match the enum variants
    match iface {
        Iface::Vlan(ref vlan_iface) => vlan_iface.do_vlan_only_action(),
        Iface::Ethernet(ref ethernet_iface) => ethernet_iface.do_ethernet_only_action(),
    }

    // enum dispatch automatically implements try_into (maybe not very useful, though)
    let vlan_iface: Result<VlanIface, _> = iface.try_into();
    if vlan_iface.is_ok() {
        println!("Is a VLAN!");
    } else {
        println!("Is not a VLAN!");
    }

    // Various `enum Iface` can be put together into a container
    let ifaces: Vec<Iface> = vec![VlanIface::new().into(), EthernetIface::new().into()];
    for iface in &ifaces {
        println!("{}", iface.name());
        match iface {
            Iface::Vlan(ref vlan_iface) => vlan_iface.do_vlan_only_action(),
            Iface::Ethernet(ref ethernet_iface) => ethernet_iface.do_ethernet_only_action(),
        }
    }
}
