#[doc = "Register `ETH_MTLRxQ0DR` reader"]
pub struct R(crate::R<ETH_MTLRXQ0DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLRXQ0DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLRXQ0DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLRXQ0DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RWCSTS` reader - RWCSTS"]
pub struct RWCSTS_R(crate::FieldReader<bool, bool>);
impl RWCSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWCSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWCSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRCSTS` reader - RRCSTS"]
pub struct RRCSTS_R(crate::FieldReader<u8, u8>);
impl RRCSTS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RRCSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RRCSTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXQSTS` reader - RXQSTS"]
pub struct RXQSTS_R(crate::FieldReader<u8, u8>);
impl RXQSTS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXQSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXQSTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRXQ` reader - PRXQ"]
pub struct PRXQ_R(crate::FieldReader<u16, u16>);
impl PRXQ_R {
    pub(crate) fn new(bits: u16) -> Self {
        PRXQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRXQ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - RWCSTS"]
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - RRCSTS"]
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - RXQSTS"]
    #[inline(always)]
    pub fn rxqsts(&self) -> RXQSTS_R {
        RXQSTS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 16:29 - PRXQ"]
    #[inline(always)]
    pub fn prxq(&self) -> PRXQ_R {
        PRXQ_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[doc = "Rx queue i debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlrx_q0dr](index.html) module"]
pub struct ETH_MTLRXQ0DR_SPEC;
impl crate::RegisterSpec for ETH_MTLRXQ0DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtlrx_q0dr::R](R) reader structure"]
impl crate::Readable for ETH_MTLRXQ0DR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MTLRxQ0DR to value 0"]
impl crate::Resettable for ETH_MTLRXQ0DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
