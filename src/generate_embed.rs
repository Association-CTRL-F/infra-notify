use crate::{
    embed::{Field, FieldBuilder},
    parser::ResticProfiles,
};
use human_bytes::human_bytes;

impl ResticProfiles {
    pub fn generate_embed_fields(self, profile: &str) -> Vec<Field> {
        let mut fields = vec![];
        let current_profile = self.profiles.get(profile).unwrap();

        fields.push({
            FieldBuilder::new()
                .name(profile)
                .value(format!(
                    "> Success: {}
                    > Duration: {} seconds
                    > Bytes added: {}
                    > Bytes processed: {}",
                    current_profile.backup.success,
                    current_profile.backup.duration,
                    human_bytes(current_profile.backup.bytes_added as f64),
                    human_bytes(current_profile.backup.bytes_total as f64),
                ))
                .build()
        });

        if !current_profile.backup.success {
            fields.push(
                FieldBuilder::new()
                    .name(format!("{} backup failed !", profile))
                    .value(format!(
                        "> Error: {}\n> Stderr: {}",
                        current_profile.backup.error, current_profile.backup.stderr
                    ))
                    .build(),
            )
        }
        fields
    }

    pub fn color(&self, profile: &str) -> u64 {
        const GREEN: u64 = 5753130;
        const RED: u64 = 13195050;
        let current_profile = self.profiles.get(profile).unwrap();
        match current_profile.backup.success {
            true => GREEN,
            false => RED,
        }
    }
}
