#[doc = "Register `ETH_MTLTxQ0DR` reader"]
pub struct R(crate::R<ETH_MTLTXQ0DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLTXQ0DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLTXQ0DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLTXQ0DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXQPAUSED` reader - TXQPAUSED"]
pub struct TXQPAUSED_R(crate::FieldReader<bool, bool>);
impl TXQPAUSED_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXQPAUSED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXQPAUSED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRCSTS` reader - TRCSTS"]
pub struct TRCSTS_R(crate::FieldReader<u8, u8>);
impl TRCSTS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRCSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRCSTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWCSTS` reader - TWCSTS"]
pub struct TWCSTS_R(crate::FieldReader<bool, bool>);
impl TWCSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TWCSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWCSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXQSTS` reader - TXQSTS"]
pub struct TXQSTS_R(crate::FieldReader<bool, bool>);
impl TXQSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXQSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXQSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSTSFSTS` reader - TXSTSFSTS"]
pub struct TXSTSFSTS_R(crate::FieldReader<bool, bool>);
impl TXSTSFSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSTSFSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSTSFSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTXQ` reader - PTXQ"]
pub struct PTXQ_R(crate::FieldReader<u8, u8>);
impl PTXQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        PTXQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTXQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STXSTSF` reader - STXSTSF"]
pub struct STXSTSF_R(crate::FieldReader<u8, u8>);
impl STXSTSF_R {
    pub(crate) fn new(bits: u8) -> Self {
        STXSTSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STXSTSF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TXQPAUSED"]
    #[inline(always)]
    pub fn txqpaused(&self) -> TXQPAUSED_R {
        TXQPAUSED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - TRCSTS"]
    #[inline(always)]
    pub fn trcsts(&self) -> TRCSTS_R {
        TRCSTS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - TWCSTS"]
    #[inline(always)]
    pub fn twcsts(&self) -> TWCSTS_R {
        TWCSTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TXQSTS"]
    #[inline(always)]
    pub fn txqsts(&self) -> TXQSTS_R {
        TXQSTS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TXSTSFSTS"]
    #[inline(always)]
    pub fn txstsfsts(&self) -> TXSTSFSTS_R {
        TXSTSFSTS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - PTXQ"]
    #[inline(always)]
    pub fn ptxq(&self) -> PTXQ_R {
        PTXQ_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - STXSTSF"]
    #[inline(always)]
    pub fn stxstsf(&self) -> STXSTSF_R {
        STXSTSF_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
#[doc = "Tx queue 0 underflow register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q0dr](index.html) module"]
pub struct ETH_MTLTXQ0DR_SPEC;
impl crate::RegisterSpec for ETH_MTLTXQ0DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtltx_q0dr::R](R) reader structure"]
impl crate::Readable for ETH_MTLTXQ0DR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MTLTxQ0DR to value 0"]
impl crate::Resettable for ETH_MTLTXQ0DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
