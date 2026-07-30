pub trait SessionId {
    fn new() -> Self;
    fn as_str(&self) -> &str;
}

pub trait SessionMeta {
    fn new() ->self;
    fn as_str(&self) -> &str;
}

impl SessionId for String {
    fn new() -> Self {
        uuid::Uuid::new_v4().to_string()
    }

    fn as_str(&self) -> &str {
        self
    }
}



