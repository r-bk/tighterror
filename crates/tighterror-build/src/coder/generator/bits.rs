use crate::{
    coder::generator::repr_type::ReprType,
    errors::{kinds::coder::*, TbError},
    spec::{ModuleSpec, Spec},
};

#[derive(Debug)]
pub struct Bits {
    /// number of category bits
    pub category: usize,
    /// number of variant bits
    pub variant: usize,
    /// number of kind bits: category + variant
    pub kind: usize,
    /// the mask of category bits
    pub category_mask: u64,
    /// the mask of variant bits
    pub variant_mask: u64,
    /// the representation type (calculated or forced)
    pub repr_type: ReprType,
}

impl Bits {
    pub fn calculate(_spec: &Spec, module: &ModuleSpec) -> Result<Self, TbError> {
        let n_categories = module.categories.len();
        let n_category_bits = calc_n_category_bits(n_categories)?;
        let n_variant_bits = calc_n_variant_bits(module)?;
        assert!(n_variant_bits > 0);

        let n_kind_bits = n_category_bits + n_variant_bits;
        let repr_type = ReprType::from_n_bits(n_kind_bits).inspect_err(|_| {
            let ltn = ReprType::largest_type_name();
            log::error!(
                "not enough bits in largest supported underlying type {ltn}: {n_kind_bits}"
            );
        })?;

        assert!(n_category_bits < repr_type.bits());
        assert!(n_variant_bits <= repr_type.bits());
        assert!(n_kind_bits <= repr_type.bits());

        let variant_mask = 1u64
            .checked_shl(n_variant_bits as u32)
            .map(|v| v - 1)
            .unwrap_or(u64::MAX);
        let category_mask = 1u64
            .checked_shl(n_category_bits as u32)
            .map(|v| v - 1)
            .unwrap_or(u64::MAX)
            .checked_shl(n_variant_bits as u32)
            .unwrap_or(0);

        Ok(Bits {
            category: n_category_bits,
            variant: n_variant_bits,
            kind: n_kind_bits,
            category_mask,
            variant_mask,
            repr_type,
        })
    }
}

fn calc_n_category_bits(n_categories: usize) -> Result<usize, TbError> {
    match n_categories {
        0 => {
            log::error!("at least one category must be defined");
            CATEGORY_REQUIRED.into()
        }
        1 => Ok(0),
        n => calc_n_bits(n, "categories"),
    }
}

fn calc_n_variant_bits(module: &ModuleSpec) -> Result<usize, TbError> {
    let n = match module.n_errors_in_largest_category() {
        Some(n) => n,
        None => {
            log::error!("at least one category must be defined");
            return CATEGORY_REQUIRED.into();
        }
    };

    match n {
        0 => {
            log::error!("at least one error must be defined");
            ERROR_REQUIRED.into()
        }
        1 => Ok(1),
        n => calc_n_bits(n, "errors in largest category"),
    }
}

fn calc_n_bits(n: usize, name: &str) -> Result<usize, TbError> {
    if let Some(po2) = n.checked_next_power_of_two() {
        Ok(usize::try_from(po2.trailing_zeros()).unwrap())
    } else {
        log::error!("too many {name}: {n}");
        TOO_MANY_BITS.into()
    }
}
