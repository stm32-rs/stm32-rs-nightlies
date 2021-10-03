#[doc = "Register `LTDC_GC2R` reader"]
pub struct R(crate::R<LTDC_GC2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_GC2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_GC2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_GC2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EDCEN` reader - EDCEN"]
pub struct EDCEN_R(crate::FieldReader<bool, bool>);
impl EDCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STSAEN` reader - STSAEN"]
pub struct STSAEN_R(crate::FieldReader<bool, bool>);
impl STSAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        STSAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STSAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DVAEN` reader - DVAEN"]
pub struct DVAEN_R(crate::FieldReader<bool, bool>);
impl DVAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DVAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DVAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPAEN` reader - DPAEN"]
pub struct DPAEN_R(crate::FieldReader<bool, bool>);
impl DPAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BW` reader - BW"]
pub struct BW_R(crate::FieldReader<u8, u8>);
impl BW_R {
    pub(crate) fn new(bits: u8) -> Self {
        BW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDCA` reader - EDCA"]
pub struct EDCA_R(crate::FieldReader<bool, bool>);
impl EDCA_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDCA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDCA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - EDCEN"]
    #[inline(always)]
    pub fn edcen(&self) -> EDCEN_R {
        EDCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - STSAEN"]
    #[inline(always)]
    pub fn stsaen(&self) -> STSAEN_R {
        STSAEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DVAEN"]
    #[inline(always)]
    pub fn dvaen(&self) -> DVAEN_R {
        DVAEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DPAEN"]
    #[inline(always)]
    pub fn dpaen(&self) -> DPAEN_R {
        DPAEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - BW"]
    #[inline(always)]
    pub fn bw(&self) -> BW_R {
        BW_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - EDCA"]
    #[inline(always)]
    pub fn edca(&self) -> EDCA_R {
        EDCA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "LTDC global configuration 2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_gc2r](index.html) module"]
pub struct LTDC_GC2R_SPEC;
impl crate::RegisterSpec for LTDC_GC2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_gc2r::R](R) reader structure"]
impl crate::Readable for LTDC_GC2R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LTDC_GC2R to value 0x30"]
impl crate::Resettable for LTDC_GC2R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x30
    }
}
