#[doc = "Register `UR13` reader"]
pub struct R(crate::R<UR13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SDRS` reader - Secured DTCM RAM Size"]
pub struct SDRS_R(crate::FieldReader<u8, u8>);
impl SDRS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDRS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D1SBRST` reader - D1 Standby reset"]
pub struct D1SBRST_R(crate::FieldReader<bool, bool>);
impl D1SBRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        D1SBRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D1SBRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Secured DTCM RAM Size"]
    #[inline(always)]
    pub fn sdrs(&self) -> SDRS_R {
        SDRS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 16 - D1 Standby reset"]
    #[inline(always)]
    pub fn d1sbrst(&self) -> D1SBRST_R {
        D1SBRST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "SYSCFG user register 13\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur13](index.html) module"]
pub struct UR13_SPEC;
impl crate::RegisterSpec for UR13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur13::R](R) reader structure"]
impl crate::Readable for UR13_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UR13 to value 0"]
impl crate::Resettable for UR13_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
