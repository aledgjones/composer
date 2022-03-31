mod meta;

use meta::Meta;

pub struct Score {
    pub meta: Meta,
}

impl Score {
    pub fn new() -> Self {
        Score { meta: Meta::new() }
    }
}
