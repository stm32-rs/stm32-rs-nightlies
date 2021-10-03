#[doc = "Register `ETH_MACSTNR` reader"]
pub struct R(crate::R<ETH_MACSTNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACSTNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACSTNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACSTNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSSS` reader - TSSS"]
pub struct TSSS_R(crate::FieldReader<u32, u32>);
impl TSSS_R {
    pub(crate) fn new(bits: u32) -> Self {
        TSSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSSS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:30 - TSSS"]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
#[doc = "The System Time Nanoseconds register, along with System Time Seconds register, indicates the current value of the system time maintained by the MAC. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macstnr](index.html) module"]
pub struct ETH_MACSTNR_SPEC;
impl crate::RegisterSpec for ETH_MACSTNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macstnr::R](R) reader structure"]
impl crate::Readable for ETH_MACSTNR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MACSTNR to value 0"]
impl crate::Resettable for ETH_MACSTNR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
