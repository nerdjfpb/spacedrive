use capnp::{
	message::{Builder, HeapAllocator},
	serialize::read_message,
};

use crate::service_capnp::client_announcement;

pub(crate) mod config;

// This module includes the code generated by Cap'n Proto in the Rust `build.rs` based on the `service.capnp` file.
pub mod service_capnp {
	include!(concat!(env!("OUT_DIR"), "/service_capnp.rs"));
}

type PeerId = String; // TODO: Pull this in from somewhere???

pub struct ClientAnnouncement<'a> {
	id: &'a str,            // TODO: Convert to this from PeerID
	addresses: Vec<String>, // TODO: Work on refs
}

impl<'a> ClientAnnouncement<'a> {
	pub fn marshal(self) -> Builder<HeapAllocator> {
		let Self { id, addresses } = self;

		let mut message = Builder::new_default();
		let mut client_announcement = message.init_root::<client_announcement::Builder>();
		client_announcement.set_peer_id(&id);

		let mut addresses_builder =
			client_announcement.init_addresses(addresses.len() as u32);
		for (i, address) in addresses.into_iter().enumerate() {
			addresses_builder.set(i as u32, &address);
		}

		message

		// TODO: Can this send straight to stream instead of allocating to vec
		// let mut buffer = vec![];
		// write_message(&mut buffer, &message).unwrap();
	}

	fn unmarshal(self, buf: &'a mut &[u8]) -> Result<Self, ()> {
		let message = read_message(buf, capnp::message::DEFAULT_READER_OPTIONS).unwrap();
		let client_announcement =
			message.get_root::<client_announcement::Reader>().unwrap();

		// println!(
		// 	"{} {}",
		// 	client_announcement.get_peer_id().unwrap(),
		// 	client_announcement.get_addresses().unwrap().get(0).unwrap()
		// );

		Ok(Self {
			id: client_announcement.get_peer_id().unwrap(),
			addresses: vec![], // TODO: client_announcement.get_addresses().unwrap().get(0).unwrap(),
		})
	}
}

// TODO: Clean below

// /// TODO
// pub(crate) const ALPN_QUIC_HTTP: &[&[u8]] = &[b"hq-29"];

// use capnp::{
// 	capability::Promise,
// 	message::Builder,
// 	serialize_packed::{read_message, write_message},
// };

// struct DiscoverySystem;

// impl discovery_system::Server for DiscoverySystem {
// 	fn publish_announcement(
// 		&mut self,
// 		_: PublishAnnouncementParams,
// 		_: PublishAnnouncementResults,
// 	) -> ::capnp::capability::Promise<(), ::capnp::Error> {
// 		::capnp::capability::Promise::err(::capnp::Error::unimplemented(
// 			"method not implemented 123".to_string(),
// 		))
// 	}
// 	fn query_announcement(
// 		&mut self,
// 		_: QueryAnnouncementParams,
// 		_: QueryAnnouncementResults,
// 	) -> ::capnp::capability::Promise<(), ::capnp::Error> {
// 		::capnp::capability::Promise::err(::capnp::Error::unimplemented(
// 			"method not implemented 456".to_string(),
// 		))
// 	}
// }