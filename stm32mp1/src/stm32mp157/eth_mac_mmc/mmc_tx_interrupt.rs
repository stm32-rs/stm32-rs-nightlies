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
#[doc = "Field `TXSCOLGPIS` reader - TXSCOLGPIS"]
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
#[doc = "Field `TXMCOLGPIS` reader - TXMCOLGPIS"]
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
#[doc = "Field `TXGPKTIS` reader - TXGPKTIS"]
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
#[doc = "Field `TXLPIUSCIS` reader - TXLPIUSCIS"]
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
#[doc = "Field `TXLPITRCIS` reader - TXLPITRCIS"]
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
    #[doc = "Bit 14 - TXSCOLGPIS"]
    #[inline(always)]
    pub fn txscolgpis(&self) -> TXSCOLGPIS_R {
        TXSCOLGPIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TXMCOLGPIS"]
    #[inline(always)]
    pub fn txmcolgpis(&self) -> TXMCOLGPIS_R {
        TXMCOLGPIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TXGPKTIS"]
    #[inline(always)]
    pub fn txgpktis(&self) -> TXGPKTIS_R {
        TXGPKTIS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TXLPIUSCIS"]
    #[inline(always)]
    pub fn txlpiuscis(&self) -> TXLPIUSCIS_R {
        TXLPIUSCIS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TXLPITRCIS"]
    #[inline(always)]
    pub fn txlpitrcis(&self) -> TXLPITRCIS_R {
        TXLPITRCIS_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
#[doc = "This register maintains the interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt register maintains the interrupts generated when transmit statistic counters reach half their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter), and when they cross their maximum values (0xFFFF_FFFF for 32-bit counter and 0xFFFF for 16-bit counter). When Counter Stop Rollover is set, the interrupts are set but the counter remains at all-ones. The MMC Transmit Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\\[7:0\\]) of the respective counter must be read to clear the interrupt bit.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_tx_interrupt](index.html) module"]
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
