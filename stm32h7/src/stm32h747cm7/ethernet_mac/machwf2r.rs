#[doc = "Register `MACHWF2R` reader"]
pub struct R(crate::R<MACHWF2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACHWF2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACHWF2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACHWF2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXQCNT` reader - Number of MTL Receive Queues"]
pub struct RXQCNT_R(crate::FieldReader<u8, u8>);
impl RXQCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXQCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXQCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXQCNT` reader - Number of MTL Transmit Queues"]
pub struct TXQCNT_R(crate::FieldReader<u8, u8>);
impl TXQCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXQCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXQCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXCHCNT` reader - Number of DMA Receive Channels"]
pub struct RXCHCNT_R(crate::FieldReader<u8, u8>);
impl RXCHCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXCHCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXCHCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCHCNT` reader - Number of DMA Transmit Channels"]
pub struct TXCHCNT_R(crate::FieldReader<u8, u8>);
impl TXCHCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXCHCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCHCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPSOUTNUM` reader - Number of PPS Outputs"]
pub struct PPSOUTNUM_R(crate::FieldReader<u8, u8>);
impl PPSOUTNUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        PPSOUTNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPSOUTNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUXSNAPNUM` reader - Number of Auxiliary Snapshot Inputs"]
pub struct AUXSNAPNUM_R(crate::FieldReader<u8, u8>);
impl AUXSNAPNUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        AUXSNAPNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUXSNAPNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Number of MTL Receive Queues"]
    #[inline(always)]
    pub fn rxqcnt(&self) -> RXQCNT_R {
        RXQCNT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - Number of MTL Transmit Queues"]
    #[inline(always)]
    pub fn txqcnt(&self) -> TXQCNT_R {
        TXQCNT_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Number of DMA Receive Channels"]
    #[inline(always)]
    pub fn rxchcnt(&self) -> RXCHCNT_R {
        RXCHCNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - Number of DMA Transmit Channels"]
    #[inline(always)]
    pub fn txchcnt(&self) -> TXCHCNT_R {
        TXCHCNT_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Number of PPS Outputs"]
    #[inline(always)]
    pub fn ppsoutnum(&self) -> PPSOUTNUM_R {
        PPSOUTNUM_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - Number of Auxiliary Snapshot Inputs"]
    #[inline(always)]
    pub fn auxsnapnum(&self) -> AUXSNAPNUM_R {
        AUXSNAPNUM_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
#[doc = "HW feature 2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [machwf2r](index.html) module"]
pub struct MACHWF2R_SPEC;
impl crate::RegisterSpec for MACHWF2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [machwf2r::R](R) reader structure"]
impl crate::Readable for MACHWF2R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MACHWF2R to value 0x4100_0000"]
impl crate::Resettable for MACHWF2R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4100_0000
    }
}
