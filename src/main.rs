use netlink_packet::{LinkMessage, LinkNla, LinkInfo, LinkInfoKind};
use futures::{Future, Stream};
use rtnetlink::new_connection;
use tokio_core::reactor::Core;

fn main() {
    let (connection, handle) = new_connection().unwrap();
    let mut core = Core::new().unwrap();
    core.handle().spawn(connection.map_err(|_| ()));

    // Create a netlink request
    let request = handle.link().get().execute().for_each(|link| {
        let kind = extract_link_kind(&link);
        let name = extract_link_name(&link);
        println!("{:?} - {:?}", name, kind);
        Ok(())
    });

    core.run(request).unwrap();
}

fn extract_link_kind(link: &LinkMessage) -> Option<&LinkInfoKind> {
    for nla in link.nlas.iter() {
        if let LinkNla::LinkInfo(info) = nla {
            for i in info.iter() {
                if let LinkInfo::Kind(kind) = i {
                    return Some(kind);
                }
            }

        }
    }
    return None;
}

fn extract_link_name(link: &LinkMessage) -> Option<&str> {
    for nla in link.nlas.iter() {
        if let LinkNla::IfName(name) = nla {
            return Some(name);
        }
    }
    return None;
}
