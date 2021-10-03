#[doc = "Register `MTLTxQDR` reader"]
pub struct R(crate::R<MTLTXQDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTLTXQDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTLTXQDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTLTXQDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXQPAUSED` reader - Transmit Queue in Pause"]
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
#[doc = "Field `TRCSTS` reader - MTL Tx Queue Read Controller Status"]
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
#[doc = "Field `TWCSTS` reader - MTL Tx Queue Write Controller Status"]
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
#[doc = "Field `TXQSTS` reader - MTL Tx Queue Not Empty Status"]
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
#[doc = "Field `TXSTSFSTS` reader - MTL Tx Status FIFO Full Status"]
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
#[doc = "Field `PTXQ` reader - Number of Packets in the Transmit Queue"]
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
#[doc = "Field `STXSTSF` reader - Number of Status Words in Tx Status FIFO of Queue"]
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
    #[doc = "Bit 0 - Transmit Queue in Pause"]
    #[inline(always)]
    pub fn txqpaused(&self) -> TXQPAUSED_R {
        TXQPAUSED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - MTL Tx Queue Read Controller Status"]
    #[inline(always)]
    pub fn trcsts(&self) -> TRCSTS_R {
        TRCSTS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - MTL Tx Queue Write Controller Status"]
    #[inline(always)]
    pub fn twcsts(&self) -> TWCSTS_R {
        TWCSTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MTL Tx Queue Not Empty Status"]
    #[inline(always)]
    pub fn txqsts(&self) -> TXQSTS_R {
        TXQSTS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MTL Tx Status FIFO Full Status"]
    #[inline(always)]
    pub fn txstsfsts(&self) -> TXSTSFSTS_R {
        TXSTSFSTS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Number of Packets in the Transmit Queue"]
    #[inline(always)]
    pub fn ptxq(&self) -> PTXQ_R {
        PTXQ_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Number of Status Words in Tx Status FIFO of Queue"]
    #[inline(always)]
    pub fn stxstsf(&self) -> STXSTSF_R {
        STXSTSF_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
#[doc = "Tx queue debug Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtltx_qdr](index.html) module"]
pub struct MTLTXQDR_SPEC;
impl crate::RegisterSpec for MTLTXQDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtltx_qdr::R](R) reader structure"]
impl crate::Readable for MTLTXQDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MTLTxQDR to value 0"]
impl crate::Resettable for MTLTXQDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
