#[doc = "Register `MACVR` reader"]
pub struct R(crate::R<MACVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SNPSVER` reader - IP version"]
pub struct SNPSVER_R(crate::FieldReader<u8, u8>);
impl SNPSVER_R {
    pub(crate) fn new(bits: u8) -> Self {
        SNPSVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SNPSVER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USERVER` reader - ST-defined version"]
pub struct USERVER_R(crate::FieldReader<u8, u8>);
impl USERVER_R {
    pub(crate) fn new(bits: u8) -> Self {
        USERVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USERVER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - IP version"]
    #[inline(always)]
    pub fn snpsver(&self) -> SNPSVER_R {
        SNPSVER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ST-defined version"]
    #[inline(always)]
    pub fn userver(&self) -> USERVER_R {
        USERVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macvr](index.html) module"]
pub struct MACVR_SPEC;
impl crate::RegisterSpec for MACVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macvr::R](R) reader structure"]
impl crate::Readable for MACVR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MACVR to value 0x3041"]
impl crate::Resettable for MACVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3041
    }
}
