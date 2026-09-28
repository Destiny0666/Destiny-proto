pub mod auth {
    pub mod v1 {
        tonic::include_proto!("auth.v1");
    }
}

pub mod events {
    pub mod auth {
        pub mod v1 {
            tonic::include_proto!("events.auth.v1");
        }
    }
}

pub mod common {
    pub mod v1 {
        tonic::include_proto!("common.v1");
    }
}
