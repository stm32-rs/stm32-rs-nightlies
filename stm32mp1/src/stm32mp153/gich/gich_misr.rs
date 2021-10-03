#[doc = "Register `GICH_MISR` reader"]
pub struct R(crate::R<GICH_MISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICH_MISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICH_MISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICH_MISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EOI` reader - EOI"]
pub struct EOI_R(crate::FieldReader<bool, bool>);
impl EOI_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U` reader - U"]
pub struct U_R(crate::FieldReader<bool, bool>);
impl U_R {
    pub(crate) fn new(bits: bool) -> Self {
        U_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LRENP` reader - LRENP"]
pub struct LRENP_R(crate::FieldReader<bool, bool>);
impl LRENP_R {
    pub(crate) fn new(bits: bool) -> Self {
        LRENP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LRENP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NP` reader - NP"]
pub struct NP_R(crate::FieldReader<bool, bool>);
impl NP_R {
    pub(crate) fn new(bits: bool) -> Self {
        NP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VGRP0E` reader - VGRP0E"]
pub struct VGRP0E_R(crate::FieldReader<bool, bool>);
impl VGRP0E_R {
    pub(crate) fn new(bits: bool) -> Self {
        VGRP0E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VGRP0E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VGRP0D` reader - VGRP0D"]
pub struct VGRP0D_R(crate::FieldReader<bool, bool>);
impl VGRP0D_R {
    pub(crate) fn new(bits: bool) -> Self {
        VGRP0D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VGRP0D_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VGRP1E` reader - VGRP1E"]
pub struct VGRP1E_R(crate::FieldReader<bool, bool>);
impl VGRP1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        VGRP1E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VGRP1E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VGRP1D` reader - VGRP1D"]
pub struct VGRP1D_R(crate::FieldReader<bool, bool>);
impl VGRP1D_R {
    pub(crate) fn new(bits: bool) -> Self {
        VGRP1D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VGRP1D_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "GICH maintenance interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gich_misr](index.html) module"]
pub struct GICH_MISR_SPEC;
impl crate::RegisterSpec for GICH_MISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gich_misr::R](R) reader structure"]
impl crate::Readable for GICH_MISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICH_MISR to value 0"]
impl crate::Resettable for GICH_MISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
