#[doc = "Register `MMC_RX_INTERRUPT` reader"]
pub type R = crate::R<MMC_RX_INTERRUPTrs>;
#[doc = "Field `RXCRCERPIS` reader - RXCRCERPIS"]
pub type RXCRCERPIS_R = crate::BitReader;
#[doc = "Field `RXALGNERPIS` reader - RXALGNERPIS"]
pub type RXALGNERPIS_R = crate::BitReader;
#[doc = "Field `RXUCGPIS` reader - RXUCGPIS"]
pub type RXUCGPIS_R = crate::BitReader;
#[doc = "Field `RXLPIUSCIS` reader - RXLPIUSCIS"]
pub type RXLPIUSCIS_R = crate::BitReader;
#[doc = "Field `RXLPITRCIS` reader - RXLPITRCIS"]
pub type RXLPITRCIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 5 - RXCRCERPIS"]
    #[inline(always)]
    pub fn rxcrcerpis(&self) -> RXCRCERPIS_R {
        RXCRCERPIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RXALGNERPIS"]
    #[inline(always)]
    pub fn rxalgnerpis(&self) -> RXALGNERPIS_R {
        RXALGNERPIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - RXUCGPIS"]
    #[inline(always)]
    pub fn rxucgpis(&self) -> RXUCGPIS_R {
        RXUCGPIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 26 - RXLPIUSCIS"]
    #[inline(always)]
    pub fn rxlpiuscis(&self) -> RXLPIUSCIS_R {
        RXLPIUSCIS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RXLPITRCIS"]
    #[inline(always)]
    pub fn rxlpitrcis(&self) -> RXLPITRCIS_R {
        RXLPITRCIS_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "This register maintains the interrupts generated from all Receive statistics counters. The MMC Receive Interrupt register maintains the interrupts that are generated when the following occur: Receive statistic counters reach half of their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter). Receive statistic counters cross their maximum values (0xFFFF_FFFF for 32 bit counter and 0xFFFF for 16 bit counter). When the Counter Stop Rollover is set, interrupts are set but the counter remains at all-ones. The MMC Receive Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\\[7:0\\]) of the respective counter must be read to clear the interrupt bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rx_interrupt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMC_RX_INTERRUPTrs;
impl crate::RegisterSpec for MMC_RX_INTERRUPTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_rx_interrupt::R`](R) reader structure"]
impl crate::Readable for MMC_RX_INTERRUPTrs {}
#[doc = "`reset()` method sets MMC_RX_INTERRUPT to value 0"]
impl crate::Resettable for MMC_RX_INTERRUPTrs {
    const RESET_VALUE: u32 = 0;
}
