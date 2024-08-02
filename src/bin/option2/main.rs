mod iface;
use iface::{EthernetIface, IfaceRef, IfaceTrait, VlanIface};

fn main() {
    let vlan_iface = VlanIface::new();
    println!("{}", vlan_iface.name());
    vlan_iface.do_vlan_only_action();

    let ethernet_iface = EthernetIface::new();
    println!("{}", ethernet_iface.name());
    ethernet_iface.do_ethernet_only_action();

    // we can use dynamic traits
    let iface = &vlan_iface as &dyn IfaceTrait;
    println!("{}", iface.name());

    // Cast to concrete type (option 1A): using Any
    if let Some(vlan_iface) = iface.as_any().downcast_ref::<VlanIface>() {
        vlan_iface.do_vlan_only_action();
    } else {
        println!("Not VLAN!");
    }
    if let Some(ethernet_iface) = iface.as_any().downcast_ref::<EthernetIface>() {
        ethernet_iface.do_ethernet_only_action();
    } else {
        println!("Not Ethernet!");
    }

    // Cast to concrete type (option 1B): using AsIfaceType (which uses Any)
    if let Ok(vlan_iface) = iface.as_iface_type::<VlanIface>() {
        vlan_iface.do_vlan_only_action();
    } else {
        println!("Not VLAN!");
    }
    if let Ok(ethernet_iface) = iface.as_iface_type::<EthernetIface>() {
        ethernet_iface.do_ethernet_only_action();
    } else {
        println!("Not Ethernet!");
    }

    // Cast to concrete type (option 2): using as_CONCRETE_iface()
    if let Ok(vlan_iface) = iface.as_vlan_iface() {
        vlan_iface.do_vlan_only_action();
    } else {
        println!("Not VLAN!");
    }
    if let Ok(ethernet_iface) = iface.as_ethernet_iface() {
        ethernet_iface.do_ethernet_only_action();
    } else {
        println!("Not Ethernet!");
    }

    // Cast to concrete type (option 3): using IfaceRef
    match iface.as_iface_ref() {
        IfaceRef::Vlan(vlan_iface) => vlan_iface.do_vlan_only_action(),
        IfaceRef::Ethernet(ethernet_iface) => ethernet_iface.do_ethernet_only_action(),
    }

    // Different interface types can be put together in a container as &dyn IfaceTrait
    let ifaces: Vec<&dyn IfaceTrait> = vec![&vlan_iface, &ethernet_iface];
    for iface in ifaces {
        println!("{}", iface.name());
        match iface.as_iface_ref() {
            IfaceRef::Vlan(vlan_iface) => vlan_iface.do_vlan_only_action(),
            IfaceRef::Ethernet(ethernet_iface) => ethernet_iface.do_ethernet_only_action(),
        }
    }
}
