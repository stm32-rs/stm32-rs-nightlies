#[doc = "Reader of register GICH_MISR"]
pub type R = crate::R<u32, super::GICH_MISR>;
#[doc = "Reader of field `EOI`"]
pub type EOI_R = crate::R<bool, bool>;
#[doc = "Reader of field `U`"]
pub type U_R = crate::R<bool, bool>;
#[doc = "Reader of field `LRENP`"]
pub type LRENP_R = crate::R<bool, bool>;
#[doc = "Reader of field `NP`"]
pub type NP_R = crate::R<bool, bool>;
#[doc = "Reader of field `VGRP0E`"]
pub type VGRP0E_R = crate::R<bool, bool>;
#[doc = "Reader of field `VGRP0D`"]
pub type VGRP0D_R = crate::R<bool, bool>;
#[doc = "Reader of field `VGRP1E`"]
pub type VGRP1E_R = crate::R<bool, bool>;
#[doc = "Reader of field `VGRP1D`"]
pub type VGRP1D_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - EOI"]
    #[inline(always)]
    pub fn eoi(&self) -> EOI_R {
        EOI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - U"]
    #[inline(always)]
    pub fn u(&self) -> U_R {
        U_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LRENP"]
    #[inline(always)]
    pub fn lrenp(&self) -> LRENP_R {
        LRENP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - NP"]
    #[inline(always)]
    pub fn np(&self) -> NP_R {
        NP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - VGRP0E"]
    #[inline(always)]
    pub fn vgrp0e(&self) -> VGRP0E_R {
        VGRP0E_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - VGRP0D"]
    #[inline(always)]
    pub fn vgrp0d(&self) -> VGRP0D_R {
        VGRP0D_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - VGRP1E"]
    #[inline(always)]
    pub fn vgrp1e(&self) -> VGRP1E_R {
        VGRP1E_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - VGRP1D"]
    #[inline(always)]
    pub fn vgrp1d(&self) -> VGRP1D_R {
        VGRP1D_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
