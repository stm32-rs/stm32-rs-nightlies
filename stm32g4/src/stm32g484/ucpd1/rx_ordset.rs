#[doc = "Register `RX_ORDSET` reader"]
pub struct R(crate::R<RX_ORDSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_ORDSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_ORDSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_ORDSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXORDSET` reader - RXORDSET"]
pub struct RXORDSET_R(crate::FieldReader<u8, u8>);
impl RXORDSET_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXORDSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXORDSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSOP3OF4` reader - RXSOP3OF4"]
pub struct RXSOP3OF4_R(crate::FieldReader<bool, bool>);
impl RXSOP3OF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXSOP3OF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSOP3OF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSOPKINVALID` reader - RXSOPKINVALID"]
pub struct RXSOPKINVALID_R(crate::FieldReader<u8, u8>);
impl RXSOPKINVALID_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXSOPKINVALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSOPKINVALID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - RXORDSET"]
    #[inline(always)]
    pub fn rxordset(&self) -> RXORDSET_R {
        RXORDSET_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - RXSOP3OF4"]
    #[inline(always)]
    pub fn rxsop3of4(&self) -> RXSOP3OF4_R {
        RXSOP3OF4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - RXSOPKINVALID"]
    #[inline(always)]
    pub fn rxsopkinvalid(&self) -> RXSOPKINVALID_R {
        RXSOPKINVALID_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
#[doc = "UCPD Rx Ordered Set Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_ordset](index.html) module"]
pub struct RX_ORDSET_SPEC;
impl crate::RegisterSpec for RX_ORDSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_ordset::R](R) reader structure"]
impl crate::Readable for RX_ORDSET_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_ORDSET to value 0"]
impl crate::Resettable for RX_ORDSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
