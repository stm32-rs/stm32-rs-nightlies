#[doc = "Register `ETH_MACHWF2R` reader"]
pub struct R(crate::R<ETH_MACHWF2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACHWF2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACHWF2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACHWF2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXQCNT` reader - RXQCNT"]
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
#[doc = "Field `TXQCNT` reader - TXQCNT"]
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
#[doc = "Field `RXCHCNT` reader - RXCHCNT"]
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
#[doc = "Field `TXCHCNT` reader - TXCHCNT"]
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
#[doc = "Field `PPSOUTNUM` reader - PPSOUTNUM"]
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
#[doc = "Field `AUXSNAPNUM` reader - AUXSNAPNUM"]
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
    #[doc = "Bits 0:3 - RXQCNT"]
    #[inline(always)]
    pub fn rxqcnt(&self) -> RXQCNT_R {
        RXQCNT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - TXQCNT"]
    #[inline(always)]
    pub fn txqcnt(&self) -> TXQCNT_R {
        TXQCNT_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - RXCHCNT"]
    #[inline(always)]
    pub fn rxchcnt(&self) -> RXCHCNT_R {
        RXCHCNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - TXCHCNT"]
    #[inline(always)]
    pub fn txchcnt(&self) -> TXCHCNT_R {
        TXCHCNT_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - PPSOUTNUM"]
    #[inline(always)]
    pub fn ppsoutnum(&self) -> PPSOUTNUM_R {
        PPSOUTNUM_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - AUXSNAPNUM"]
    #[inline(always)]
    pub fn auxsnapnum(&self) -> AUXSNAPNUM_R {
        AUXSNAPNUM_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
#[doc = "This register indicates the presence of third set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_machwf2r](index.html) module"]
pub struct ETH_MACHWF2R_SPEC;
impl crate::RegisterSpec for ETH_MACHWF2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_machwf2r::R](R) reader structure"]
impl crate::Readable for ETH_MACHWF2R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_MACHWF2R to value 0x4104_0041"]
impl crate::Resettable for ETH_MACHWF2R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4104_0041
    }
}
