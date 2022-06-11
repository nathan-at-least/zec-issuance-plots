use crate::consts::{BLOSSOM_ACTIVATION, POST_BLOSSOM_POW_TARGET_SPACING};
use crate::subsidy::Subsidy;
use crate::units::{Height, Zat};

#[derive(Debug)]
pub struct TailEmissionSubsidy {
    activation_height: Height,
    activation_supply: f64,
}

impl TailEmissionSubsidy {
    pub fn subsidy_from_activation_height(h: Height) -> Subsidy {
        Subsidy::TailEmission(Self::from_activation_height(h))
    }

    fn from_activation_height(activation_height: Height) -> Self {
        assert!(
            activation_height >= BLOSSOM_ACTIVATION,
            "Only post-blossom activation implemented. activation {:?} < blossom {:?}",
            activation_height,
            BLOSSOM_ACTIVATION
        );

        let activation_supply = (0..activation_height)
            .map(|h| Subsidy::NU5.block_subsidy(h))
            .sum::<Zat>() as f64;

        dbg!(TailEmissionSubsidy {
            activation_height,
            activation_supply,
        })
    }

    pub fn block_subsidy(&self, h: Height) -> Zat {
        assert!(h >= self.activation_height);

        const ANNUAL_GROWTH_RATE: f64 = 1.02;
        const BLOCKS_PER_YEAR: Height = 365 * 24 * 60 * 60 / POST_BLOSSOM_POW_TARGET_SPACING;
        let block_growth_rate: f64 = ANNUAL_GROWTH_RATE.powf(1f64 / (BLOCKS_PER_YEAR as f64));

        let delta = h - self.activation_height;
        let growth_factor = block_growth_rate.powi(delta as i32);
        let supply = (self.activation_supply as f64) * growth_factor;
        let subsidy = supply * (block_growth_rate - 1.0f64);

        subsidy as Zat
    }
}
