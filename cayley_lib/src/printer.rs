use std::fmt;

use crate::frame::frame::Frame;

impl fmt::Debug for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (k, _) in self.columns_map() {
            write!(f, "{: <15} | ", k).unwrap();
        }

        write!(f, "{}\n", "").unwrap();

        for (k, v) in self.rows_map() {
            for c in (*v).cells() {
                write!(f, "{: <15} | ", c.value()).unwrap();
            }
            write!(f, "{}\n", "").unwrap();
        }

        Ok(())
    }
}
