use std::sync::Once;

use fortuna::init_v8;

static INIT: Once = Once::new();

pub fn setup() {
    INIT.call_once(|| init_v8());
}
