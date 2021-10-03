#[doc = "Register `ETH_MACDR` reader"]
pub struct R(crate::R<ETH_MACDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RPESTS` reader - RPESTS"]
pub struct RPESTS_R(crate::FieldReader<bool, bool>);
impl RPESTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPESTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPESTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFCFCSTS` reader - RFCFCSTS"]
pub struct RFCFCSTS_R(crate::FieldReader<u8, u8>);
impl RFCFCSTS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RFCFCSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCFCSTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPESTS` reader - TPESTS"]
pub struct TPESTS_R(crate::FieldReader<bool, bool>);
impl TPESTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TPESTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPESTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFCSTS` reader - TFCSTS"]
pub struct TFCSTS_R(crate::FieldReader<u8, u8>);
impl TFCSTS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TFCSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFCSTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - RPESTS"]
    #[inline(always)]
    pub fn rpests(&self) -> RPESTS_R {
        RPESTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - RFCFCSTS"]
    #[inline(always)]
    pub fn rfcfcsts(&self) -> RFCFCSTS_R {
        RFCFCSTS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 16 - TPESTS"]
    #[inline(always)]
    pub fn tpests(&self) -> TPESTS_R {
        TPESTS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - TFCSTS"]
    #[inline(always)]
    pub fn tfcsts(&self) -> TFCSTS_R {
        TFCSTS_R::new(((self.bits >> 17) & 0x03) as u8)
    }
}
#[doc = "The Debug register provides the debug status of various MAC blocks.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macdr](index.html) module"]
pub struct ETH_MACDR_SPEC;
impl crate::RegisterSpec for ETH_MACDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macdr::R](R) reader structure"]
impl crate::Readable for ETH_MACDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MACDR to value 0"]
impl crate::Resettable for ETH_MACDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
