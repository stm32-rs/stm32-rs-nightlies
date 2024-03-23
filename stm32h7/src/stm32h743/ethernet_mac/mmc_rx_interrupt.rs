#[doc = "Register `MMC_RX_INTERRUPT` reader"]
pub type R = crate::R<MMC_RX_INTERRUPTrs>;
#[doc = "Field `RXCRCERPIS` reader - MMC Receive CRC Error Packet Counter Interrupt Status"]
pub type RXCRCERPIS_R = crate::BitReader;
#[doc = "Field `RXALGNERPIS` reader - MMC Receive Alignment Error Packet Counter Interrupt Status"]
pub type RXALGNERPIS_R = crate::BitReader;
#[doc = "Field `RXUCGPIS` reader - MMC Receive Unicast Good Packet Counter Interrupt Status"]
pub type RXUCGPIS_R = crate::BitReader;
#[doc = "Field `RXLPIUSCIS` reader - MMC Receive LPI microsecond counter interrupt status"]
pub type RXLPIUSCIS_R = crate::BitReader;
#[doc = "Field `RXLPITRCIS` reader - MMC Receive LPI transition counter interrupt status"]
pub type RXLPITRCIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 5 - MMC Receive CRC Error Packet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxcrcerpis(&self) -> RXCRCERPIS_R {
        RXCRCERPIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Packet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxalgnerpis(&self) -> RXALGNERPIS_R {
        RXALGNERPIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Packet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxucgpis(&self) -> RXUCGPIS_R {
        RXUCGPIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 26 - MMC Receive LPI microsecond counter interrupt status"]
    #[inline(always)]
    pub fn rxlpiuscis(&self) -> RXLPIUSCIS_R {
        RXLPIUSCIS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MMC Receive LPI transition counter interrupt status"]
    #[inline(always)]
    pub fn rxlpitrcis(&self) -> RXLPITRCIS_R {
        RXLPITRCIS_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "MMC Rx interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rx_interrupt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
