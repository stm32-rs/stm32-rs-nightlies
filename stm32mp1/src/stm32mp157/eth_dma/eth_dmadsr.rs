#[doc = "Register `ETH_DMADSR` reader"]
pub struct R(crate::R<ETH_DMADSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMADSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMADSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMADSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AXWHSTS` reader - AHB Master Write Channel"]
pub struct AXWHSTS_R(crate::FieldReader<bool, bool>);
impl AXWHSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        AXWHSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AXWHSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AXRHSTS` reader - AXRHSTS"]
pub struct AXRHSTS_R(crate::FieldReader<bool, bool>);
impl AXRHSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        AXRHSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AXRHSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPS0` reader - RPS0"]
pub struct RPS0_R(crate::FieldReader<u8, u8>);
impl RPS0_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPS0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPS0` reader - TPS0"]
pub struct TPS0_R(crate::FieldReader<u8, u8>);
impl TPS0_R {
    pub(crate) fn new(bits: u8) -> Self {
        TPS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPS0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPS1` reader - RPS1"]
pub struct RPS1_R(crate::FieldReader<u8, u8>);
impl RPS1_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPS1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPS1` reader - TPS1"]
pub struct TPS1_R(crate::FieldReader<u8, u8>);
impl TPS1_R {
    pub(crate) fn new(bits: u8) -> Self {
        TPS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPS1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - AHB Master Write Channel"]
    #[inline(always)]
    pub fn axwhsts(&self) -> AXWHSTS_R {
        AXWHSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AXRHSTS"]
    #[inline(always)]
    pub fn axrhsts(&self) -> AXRHSTS_R {
        AXRHSTS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - RPS0"]
    #[inline(always)]
    pub fn rps0(&self) -> RPS0_R {
        RPS0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - TPS0"]
    #[inline(always)]
    pub fn tps0(&self) -> TPS0_R {
        TPS0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - RPS1"]
    #[inline(always)]
    pub fn rps1(&self) -> RPS1_R {
        RPS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - TPS1"]
    #[inline(always)]
    pub fn tps1(&self) -> TPS1_R {
        TPS1_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[doc = "Debug status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmadsr](index.html) module"]
pub struct ETH_DMADSR_SPEC;
impl crate::RegisterSpec for ETH_DMADSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmadsr::R](R) reader structure"]
impl crate::Readable for ETH_DMADSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_DMADSR to value 0"]
impl crate::Resettable for ETH_DMADSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
