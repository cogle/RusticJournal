#[macro_use]
mod macros {
    macro_rules! journal_try {
        ($func:expr) => {
            {
                let rc = unsafe { $func };
                
                if (rc != 0) {

                }

                rc
            }
        };
    }
}
