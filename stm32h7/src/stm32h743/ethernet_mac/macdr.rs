#[doc = "Register `MACDR` reader"]
pub struct R(crate::R<MACDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RPESTS` reader - MAC MII Receive Protocol Engine Status"]
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
#[doc = "Field `RFCFCSTS` reader - MAC Receive Packet Controller FIFO Status"]
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
#[doc = "Field `TPESTS` reader - MAC MII Transmit Protocol Engine Status"]
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
#[doc = "Field `TFCSTS` reader - MAC Transmit Packet Controller Status"]
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
    #[doc = "Bit 0 - MAC MII Receive Protocol Engine Status"]
    #[inline(always)]
    pub fn rpests(&self) -> RPESTS_R {
        RPESTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - MAC Receive Packet Controller FIFO Status"]
    #[inline(always)]
    pub fn rfcfcsts(&self) -> RFCFCSTS_R {
        RFCFCSTS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 16 - MAC MII Transmit Protocol Engine Status"]
    #[inline(always)]
    pub fn tpests(&self) -> TPESTS_R {
        TPESTS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - MAC Transmit Packet Controller Status"]
    #[inline(always)]
    pub fn tfcsts(&self) -> TFCSTS_R {
        TFCSTS_R::new(((self.bits >> 17) & 0x03) as u8)
    }
}
#[doc = "Debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macdr](index.html) module"]
pub struct MACDR_SPEC;
impl crate::RegisterSpec for MACDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macdr::R](R) reader structure"]
impl crate::Readable for MACDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MACDR to value 0"]
impl crate::Resettable for MACDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
