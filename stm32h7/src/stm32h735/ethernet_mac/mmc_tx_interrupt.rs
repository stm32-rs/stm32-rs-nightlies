#[doc = "Register `MMC_TX_INTERRUPT` reader"]
pub struct R(crate::R<MMC_TX_INTERRUPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_TX_INTERRUPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_TX_INTERRUPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_TX_INTERRUPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXSCOLGPIS` reader - MMC Transmit Single Collision Good Packet Counter Interrupt Status"]
pub struct TXSCOLGPIS_R(crate::FieldReader<bool, bool>);
impl TXSCOLGPIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSCOLGPIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSCOLGPIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMCOLGPIS` reader - MMC Transmit Multiple Collision Good Packet Counter Interrupt Status"]
pub struct TXMCOLGPIS_R(crate::FieldReader<bool, bool>);
impl TXMCOLGPIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMCOLGPIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMCOLGPIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXGPKTIS` reader - MMC Transmit Good Packet Counter Interrupt Status"]
pub struct TXGPKTIS_R(crate::FieldReader<bool, bool>);
impl TXGPKTIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXGPKTIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXGPKTIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXLPIUSCIS` reader - MMC Transmit LPI microsecond counter interrupt status"]
pub struct TXLPIUSCIS_R(crate::FieldReader<bool, bool>);
impl TXLPIUSCIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXLPIUSCIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXLPIUSCIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXLPITRCIS` reader - MMC Transmit LPI transition counter interrupt status"]
pub struct TXLPITRCIS_R(crate::FieldReader<bool, bool>);
impl TXLPITRCIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXLPITRCIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXLPITRCIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Packet Counter Interrupt Status"]
    #[inline(always)]
    pub fn txscolgpis(&self) -> TXSCOLGPIS_R {
        TXSCOLGPIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Packet Counter Interrupt Status"]
    #[inline(always)]
    pub fn txmcolgpis(&self) -> TXMCOLGPIS_R {
        TXMCOLGPIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 21 - MMC Transmit Good Packet Counter Interrupt Status"]
    #[inline(always)]
    pub fn txgpktis(&self) -> TXGPKTIS_R {
        TXGPKTIS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 26 - MMC Transmit LPI microsecond counter interrupt status"]
    #[inline(always)]
    pub fn txlpiuscis(&self) -> TXLPIUSCIS_R {
        TXLPIUSCIS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - MMC Transmit LPI transition counter interrupt status"]
    #[inline(always)]
    pub fn txlpitrcis(&self) -> TXLPITRCIS_R {
        TXLPITRCIS_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
#[doc = "MMC Tx interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_tx_interrupt](index.html) module"]
pub struct MMC_TX_INTERRUPT_SPEC;
impl crate::RegisterSpec for MMC_TX_INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmc_tx_interrupt::R](R) reader structure"]
impl crate::Readable for MMC_TX_INTERRUPT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MMC_TX_INTERRUPT to value 0"]
impl crate::Resettable for MMC_TX_INTERRUPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
