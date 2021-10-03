#[doc = "Register `FDCAN_HPMS` reader"]
pub struct R(crate::R<FDCAN_HPMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_HPMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_HPMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_HPMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BIDX` reader - Buffer Index"]
pub struct BIDX_R(crate::FieldReader<u8, u8>);
impl BIDX_R {
    pub(crate) fn new(bits: u8) -> Self {
        BIDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIDX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSI` reader - Message Storage Indicator"]
pub struct MSI_R(crate::FieldReader<u8, u8>);
impl MSI_R {
    pub(crate) fn new(bits: u8) -> Self {
        MSI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIDX` reader - Filter Index"]
pub struct FIDX_R(crate::FieldReader<u8, u8>);
impl FIDX_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIDX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLST` reader - Filter List"]
pub struct FLST_R(crate::FieldReader<bool, bool>);
impl FLST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Buffer Index"]
    #[inline(always)]
    pub fn bidx(&self) -> BIDX_R {
        BIDX_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - Message Storage Indicator"]
    #[inline(always)]
    pub fn msi(&self) -> MSI_R {
        MSI_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:12 - Filter Index"]
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Filter List"]
    #[inline(always)]
    pub fn flst(&self) -> FLST_R {
        FLST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "FDCAN High Priority Message Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_hpms](index.html) module"]
pub struct FDCAN_HPMS_SPEC;
impl crate::RegisterSpec for FDCAN_HPMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_hpms::R](R) reader structure"]
impl crate::Readable for FDCAN_HPMS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_HPMS to value 0"]
impl crate::Resettable for FDCAN_HPMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}