#[doc = "Register `HWCFGR2` reader"]
pub struct R(crate::R<HWCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PTIONREG_OUT` reader - PTIONREG_OUT"]
pub struct PTIONREG_OUT_R(crate::FieldReader<u8, u8>);
impl PTIONREG_OUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PTIONREG_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTIONREG_OUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRUST_ZONE` reader - TRUST_ZONE"]
pub struct TRUST_ZONE_R(crate::FieldReader<u8, u8>);
impl TRUST_ZONE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRUST_ZONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRUST_ZONE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - PTIONREG_OUT"]
    #[inline(always)]
    pub fn ptionreg_out(&self) -> PTIONREG_OUT_R {
        PTIONREG_OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - TRUST_ZONE"]
    #[inline(always)]
    pub fn trust_zone(&self) -> TRUST_ZONE_R {
        TRUST_ZONE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "TAMP hardware configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr2](index.html) module"]
pub struct HWCFGR2_SPEC;
impl crate::RegisterSpec for HWCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr2::R](R) reader structure"]
impl crate::Readable for HWCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWCFGR2 to value 0"]
impl crate::Resettable for HWCFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
