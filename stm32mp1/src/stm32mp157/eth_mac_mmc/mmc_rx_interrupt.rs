#[doc = "Register `MMC_RX_INTERRUPT` reader"]
pub struct R(crate::R<MMC_RX_INTERRUPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_RX_INTERRUPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_RX_INTERRUPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_RX_INTERRUPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXCRCERPIS` reader - RXCRCERPIS"]
pub struct RXCRCERPIS_R(crate::FieldReader<bool, bool>);
impl RXCRCERPIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXCRCERPIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXCRCERPIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXALGNERPIS` reader - RXALGNERPIS"]
pub struct RXALGNERPIS_R(crate::FieldReader<bool, bool>);
impl RXALGNERPIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXALGNERPIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXALGNERPIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUCGPIS` reader - RXUCGPIS"]
pub struct RXUCGPIS_R(crate::FieldReader<bool, bool>);
impl RXUCGPIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXUCGPIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUCGPIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXLPIUSCIS` reader - RXLPIUSCIS"]
pub struct RXLPIUSCIS_R(crate::FieldReader<bool, bool>);
impl RXLPIUSCIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXLPIUSCIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXLPIUSCIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXLPITRCIS` reader - RXLPITRCIS"]
pub struct RXLPITRCIS_R(crate::FieldReader<bool, bool>);
impl RXLPITRCIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXLPITRCIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXLPITRCIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 5 - RXCRCERPIS"]
    #[inline(always)]
    pub fn rxcrcerpis(&self) -> RXCRCERPIS_R {
        RXCRCERPIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RXALGNERPIS"]
    #[inline(always)]
    pub fn rxalgnerpis(&self) -> RXALGNERPIS_R {
        RXALGNERPIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RXUCGPIS"]
    #[inline(always)]
    pub fn rxucgpis(&self) -> RXUCGPIS_R {
        RXUCGPIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 26 - RXLPIUSCIS"]
    #[inline(always)]
    pub fn rxlpiuscis(&self) -> RXLPIUSCIS_R {
        RXLPIUSCIS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - RXLPITRCIS"]
    #[inline(always)]
    pub fn rxlpitrcis(&self) -> RXLPITRCIS_R {
        RXLPITRCIS_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
#[doc = "This register maintains the interrupts generated from all Receive statistics counters. The MMC Receive Interrupt register maintains the interrupts that are generated when the following occur: Receive statistic counters reach half of their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter). Receive statistic counters cross their maximum values (0xFFFF_FFFF for 32 bit counter and 0xFFFF for 16 bit counter). When the Counter Stop Rollover is set, interrupts are set but the counter remains at all-ones. The MMC Receive Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\\[7:0\\]) of the respective counter must be read to clear the interrupt bit.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_rx_interrupt](index.html) module"]
pub struct MMC_RX_INTERRUPT_SPEC;
impl crate::RegisterSpec for MMC_RX_INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmc_rx_interrupt::R](R) reader structure"]
impl crate::Readable for MMC_RX_INTERRUPT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MMC_RX_INTERRUPT to value 0"]
impl crate::Resettable for MMC_RX_INTERRUPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
