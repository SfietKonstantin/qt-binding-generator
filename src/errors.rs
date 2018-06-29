use serde_json::Error as JsonError;
use std::io::Error as IoError;

error_chain!(
    foreign_links {
        JsonError(JsonError);
        IoError(IoError);
    }
);
