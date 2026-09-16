///Register `MMC_RX_INTERRUPT` reader
pub type R = crate::R<MMC_RX_INTERRUPTrs>;
///Field `RXCRCERPIS` reader - MMC Receive CRC Error Packet Counter Interrupt Status
pub type RXCRCERPIS_R = crate::BitReader;
///Field `RXALGNERPIS` reader - MMC Receive Alignment Error Packet Counter Interrupt Status
pub type RXALGNERPIS_R = crate::BitReader;
///Field `RXUCGPIS` reader - MMC Receive Unicast Good Packet Counter Interrupt Status
pub type RXUCGPIS_R = crate::BitReader;
///Field `RXLPIUSCIS` reader - MMC Receive LPI microsecond counter interrupt status
pub type RXLPIUSCIS_R = crate::BitReader;
///Field `RXLPITRCIS` reader - MMC Receive LPI transition counter interrupt status
pub type RXLPITRCIS_R = crate::BitReader;
impl R {
    ///Bit 5 - MMC Receive CRC Error Packet Counter Interrupt Status
    #[inline(always)]
    pub fn rxcrcerpis(&self) -> RXCRCERPIS_R {
        RXCRCERPIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MMC Receive Alignment Error Packet Counter Interrupt Status
    #[inline(always)]
    pub fn rxalgnerpis(&self) -> RXALGNERPIS_R {
        RXALGNERPIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 17 - MMC Receive Unicast Good Packet Counter Interrupt Status
    #[inline(always)]
    pub fn rxucgpis(&self) -> RXUCGPIS_R {
        RXUCGPIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 26 - MMC Receive LPI microsecond counter interrupt status
    #[inline(always)]
    pub fn rxlpiuscis(&self) -> RXLPIUSCIS_R {
        RXLPIUSCIS_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - MMC Receive LPI transition counter interrupt status
    #[inline(always)]
    pub fn rxlpitrcis(&self) -> RXLPITRCIS_R {
        RXLPITRCIS_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMC_RX_INTERRUPT")
            .field("rxcrcerpis", &self.rxcrcerpis())
            .field("rxalgnerpis", &self.rxalgnerpis())
            .field("rxucgpis", &self.rxucgpis())
            .field("rxlpiuscis", &self.rxlpiuscis())
            .field("rxlpitrcis", &self.rxlpitrcis())
            .finish()
    }
}
/**MMC Rx interrupt register

You can [`read`](crate::Reg::read) this register and get [`mmc_rx_interrupt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#Ethernet_MAC:MMC_RX_INTERRUPT)*/
pub struct MMC_RX_INTERRUPTrs;
impl crate::RegisterSpec for MMC_RX_INTERRUPTrs {
    type Ux = u32;
}
///`read()` method returns [`mmc_rx_interrupt::R`](R) reader structure
impl crate::Readable for MMC_RX_INTERRUPTrs {}
///`reset()` method sets MMC_RX_INTERRUPT to value 0
impl crate::Resettable for MMC_RX_INTERRUPTrs {}
