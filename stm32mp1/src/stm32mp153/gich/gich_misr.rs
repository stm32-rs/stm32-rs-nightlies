#[doc = "Register `GICH_MISR` reader"]
pub type R = crate::R<GICH_MISRrs>;
#[doc = "Field `EOI` reader - EOI"]
pub type EOI_R = crate::BitReader;
#[doc = "Field `U` reader - U"]
pub type U_R = crate::BitReader;
#[doc = "Field `LRENP` reader - LRENP"]
pub type LRENP_R = crate::BitReader;
#[doc = "Field `NP` reader - NP"]
pub type NP_R = crate::BitReader;
#[doc = "Field `VGRP0E` reader - VGRP0E"]
pub type VGRP0E_R = crate::BitReader;
#[doc = "Field `VGRP0D` reader - VGRP0D"]
pub type VGRP0D_R = crate::BitReader;
#[doc = "Field `VGRP1E` reader - VGRP1E"]
pub type VGRP1E_R = crate::BitReader;
#[doc = "Field `VGRP1D` reader - VGRP1D"]
pub type VGRP1D_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - EOI"]
    #[inline(always)]
    pub fn eoi(&self) -> EOI_R {
        EOI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - U"]
    #[inline(always)]
    pub fn u(&self) -> U_R {
        U_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LRENP"]
    #[inline(always)]
    pub fn lrenp(&self) -> LRENP_R {
        LRENP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NP"]
    #[inline(always)]
    pub fn np(&self) -> NP_R {
        NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VGRP0E"]
    #[inline(always)]
    pub fn vgrp0e(&self) -> VGRP0E_R {
        VGRP0E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VGRP0D"]
    #[inline(always)]
    pub fn vgrp0d(&self) -> VGRP0D_R {
        VGRP0D_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VGRP1E"]
    #[inline(always)]
    pub fn vgrp1e(&self) -> VGRP1E_R {
        VGRP1E_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VGRP1D"]
    #[inline(always)]
    pub fn vgrp1d(&self) -> VGRP1D_R {
        VGRP1D_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "GICH maintenance interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gich_misr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICH_MISRrs;
impl crate::RegisterSpec for GICH_MISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gich_misr::R`](R) reader structure"]
impl crate::Readable for GICH_MISRrs {}
#[doc = "`reset()` method sets GICH_MISR to value 0"]
impl crate::Resettable for GICH_MISRrs {
    const RESET_VALUE: u32 = 0;
}
