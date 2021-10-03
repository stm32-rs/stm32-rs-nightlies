#[doc = "Register `MDMA_C12ESR` reader"]
pub struct R(crate::R<MDMA_C12ESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDMA_C12ESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDMA_C12ESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDMA_C12ESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TEA` reader - TEA"]
pub struct TEA_R(crate::FieldReader<u8, u8>);
impl TEA_R {
    pub(crate) fn new(bits: u8) -> Self {
        TEA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TED` reader - TED"]
pub struct TED_R(crate::FieldReader<bool, bool>);
impl TED_R {
    pub(crate) fn new(bits: bool) -> Self {
        TED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TELD` reader - TELD"]
pub struct TELD_R(crate::FieldReader<bool, bool>);
impl TELD_R {
    pub(crate) fn new(bits: bool) -> Self {
        TELD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TELD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEMD` reader - TEMD"]
pub struct TEMD_R(crate::FieldReader<bool, bool>);
impl TEMD_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASE` reader - ASE"]
pub struct ASE_R(crate::FieldReader<bool, bool>);
impl ASE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BSE` reader - BSE"]
pub struct BSE_R(crate::FieldReader<bool, bool>);
impl BSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:6 - TEA"]
    #[inline(always)]
    pub fn tea(&self) -> TEA_R {
        TEA_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - TED"]
    #[inline(always)]
    pub fn ted(&self) -> TED_R {
        TED_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TELD"]
    #[inline(always)]
    pub fn teld(&self) -> TELD_R {
        TELD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TEMD"]
    #[inline(always)]
    pub fn temd(&self) -> TEMD_R {
        TEMD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ASE"]
    #[inline(always)]
    pub fn ase(&self) -> ASE_R {
        ASE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BSE"]
    #[inline(always)]
    pub fn bse(&self) -> BSE_R {
        BSE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
#[doc = "MDMA channel 12 error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c12esr](index.html) module"]
pub struct MDMA_C12ESR_SPEC;
impl crate::RegisterSpec for MDMA_C12ESR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdma_c12esr::R](R) reader structure"]
impl crate::Readable for MDMA_C12ESR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MDMA_C12ESR to value 0"]
impl crate::Resettable for MDMA_C12ESR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
