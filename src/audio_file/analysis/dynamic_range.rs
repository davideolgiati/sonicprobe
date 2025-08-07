use std::process;

use crate::{
    audio_file::{Frequency, Signal},
    sonicprobe_error::SonicProbeError,
};

impl super::DynamicRange {
    #[inline]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn process(
        samples: &Signal,
        sample_rate: Frequency,
        peak: f64,
    ) -> Result<f64, SonicProbeError> {
        let chunk_size = match usize::try_from((sample_rate * 15) / 100) {
            Ok(value) => value,
            Err(e) => {
                return Err(SonicProbeError {
                    location: format!("{}:{}", file!(), line!()),
                    message: format!("{e:?}"),
                });
            }
        };
        let reminder = samples.len() % chunk_size;
        let samples_end = samples.len() - reminder;
        let analysable_samples = samples.get(0..samples_end).map_or_else(
            || {
                println!("error: dynamic range can't slice samples in index 0 to {samples_end}");
                process::exit(1);
            },
            |slice| slice,
        );

        let mut rms_array: Vec<f64> = Vec::new();

        for chunk in analysable_samples.chunks(chunk_size) {
            rms_array.push(super::RootMeanSquare::process(chunk)?);
        }

        let rms_end = (rms_array.len() * 20) / 100;

        rms_array.sort_by(|a, b| {
            b.partial_cmp(a)
                .map_or(std::cmp::Ordering::Equal, |value| value)
        });

        let top_20_rms = rms_array.get(0..rms_end).map_or_else(
            || {
                println!("error: dynamic range can't slice rms in index 0 to {rms_end}");
                process::exit(1);
            },
            |rms_slice| rms_slice,
        );

        let size = rms_end as f64;
        if (size as usize) != rms_end {
            return Err(SonicProbeError {
                location: format!("{}:{}", file!(), line!()),
                message: format!("cannot represent usize value {rms_end} exactly in f64"),
            });
        }

        let rms_avarage = top_20_rms.iter().sum::<f64>() / size;

        Ok(peak - rms_avarage)
    }
}
