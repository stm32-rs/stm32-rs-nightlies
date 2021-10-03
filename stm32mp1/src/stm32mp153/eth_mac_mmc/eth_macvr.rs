#[doc = "Register `ETH_MACVR` reader"]
pub struct R(crate::R<ETH_MACVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SNPSVER` reader - SNPSVER"]
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
#[doc = "Field `USERVER` reader - USERVER"]
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
    #[doc = "Bits 0:7 - SNPSVER"]
    #[inline(always)]
    pub fn snpsver(&self) -> SNPSVER_R {
        SNPSVER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - USERVER"]
    #[inline(always)]
    pub fn userver(&self) -> USERVER_R {
        USERVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "The version register identifies the version of the Ethernet peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macvr](index.html) module"]
pub struct ETH_MACVR_SPEC;
impl crate::RegisterSpec for ETH_MACVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macvr::R](R) reader structure"]
impl crate::Readable for ETH_MACVR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MACVR to value 0x4042"]
impl crate::Resettable for ETH_MACVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4042
    }
}
