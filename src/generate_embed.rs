use std::collections::BTreeMap;

use crate::{
    embed::{Field, FieldBuilder},
    parser::ResticProfiles,
};

use human_bytes::human_bytes;

impl ResticProfiles {
    pub fn generate_embed_fields(self) -> Vec<Field> {
        let mut fields = vec![];
        for s in self.profiles.iter() {
            let backup = &s.1.backup;
            fields.push({
                FieldBuilder::new()
                    .name(s.0)
                    .value(format!(
                        "
                        > Success: {}
                        > Duration: {} seconds
                        > MebiBytes added: {}
                        > MebiBytes proccessed: {}
                        ",
                        backup.success,
                        backup.duration,
                        human_bytes(backup.bytes_added as f64),
                        human_bytes(backup.bytes_total as f64),
                    ))
                    .build()
            });

            if !backup.success {
                fields.push(
                    FieldBuilder::new()
                        .name(format!("{} backup failed !", s.0))
                        .value(format!(
                            "
                            ```diff\n- Error: {}\n- Stderr: {}```",
                            backup.error, backup.stderr
                        ))
                        .build(),
                )
            }
        }
        fields
    }

    pub fn status(&self) -> Status {
        let mut count = BTreeMap::new();
        let results = self
            .profiles
            .iter()
            .map(|profile| profile.1.backup.success)
            .collect::<Vec<bool>>();

        for value in results.iter() {
            let count = count.entry(value).or_insert(0);
            *count += 1;
        }

        match count.get(&true) {
            Some(true_count) => {
                let num_status = results.len();
                if *true_count == num_status {
                    Status::AllSucceded
                } else if *true_count == num_status - 1 {
                    Status::OneFailed
                } else {
                    Status::MultipleFailed
                }
            }
            None => Status::AllFailed,
        }
    }
}

#[derive(Debug)]
pub enum Status {
    AllSucceded,
    MultipleFailed,
    OneFailed,
    AllFailed,
}
