// @generated
pub mod acme {
    pub mod call {
        // @@protoc_insertion_point(attribute:acme.call.v1)
        pub mod v1 {
            include!("acme.call.v1.rs");
            // @@protoc_insertion_point(acme.call.v1)
        }
    }
}
pub mod sf {
    // @@protoc_insertion_point(attribute:sf.substreams)
    pub mod substreams {
        include!("sf.substreams.rs");
        // @@protoc_insertion_point(sf.substreams)
        pub mod sink {
            pub mod service {
                // @@protoc_insertion_point(attribute:sf.substreams.sink.service.v1)
                pub mod v1 {
                    include!("sf.substreams.sink.service.v1.rs");
                    // @@protoc_insertion_point(sf.substreams.sink.service.v1)
                }
            }
        }
    }
}
