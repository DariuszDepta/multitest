macro_rules! bin {
    ($msg: expr) => {
        to_json_vec(&msg).unwrap().as_slice()
    };
}

pub use bin;
