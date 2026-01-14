///Register `MMC_RX_INTERRUPT` reader
pub type R = crate::R<MMC_RX_INTERRUPTrs>;
///Register `MMC_RX_INTERRUPT` writer
pub type W = crate::W<MMC_RX_INTERRUPTrs>;
/**Field `RXCRCERPIS` reader - MMC Receive CRC Error Packet Counter Interrupt Status

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type RXCRCERPIS_R = crate::BitReader;
///Field `RXCRCERPIS` writer - MMC Receive CRC Error Packet Counter Interrupt Status
pub type RXCRCERPIS_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `RXALGNERPIS` reader - MMC Receive Alignment Error Packet Counter Interrupt Status

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type RXALGNERPIS_R = crate::BitReader;
///Field `RXALGNERPIS` writer - MMC Receive Alignment Error Packet Counter Interrupt Status
pub type RXALGNERPIS_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `RXUCGPIS` reader - MMC Receive Unicast Good Packet Counter Interrupt Status

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type RXUCGPIS_R = crate::BitReader;
///Field `RXUCGPIS` writer - MMC Receive Unicast Good Packet Counter Interrupt Status
pub type RXUCGPIS_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `RXLPIUSCIS` reader - MMC Receive LPI microsecond counter interrupt status

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type RXLPIUSCIS_R = crate::BitReader;
///Field `RXLPIUSCIS` writer - MMC Receive LPI microsecond counter interrupt status
pub type RXLPIUSCIS_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `RXLPITRCIS` reader - MMC Receive LPI transition counter interrupt status

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type RXLPITRCIS_R = crate::BitReader;
///Field `RXLPITRCIS` writer - MMC Receive LPI transition counter interrupt status
pub type RXLPITRCIS_W<'a, REG> = crate::BitWriter<'a, REG>;
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
        f.debug_struct("MMC_RX_INTERRUPT").finish()
    }
}
impl W {
    ///Bit 5 - MMC Receive CRC Error Packet Counter Interrupt Status
    #[inline(always)]
    pub fn rxcrcerpis(&mut self) -> RXCRCERPIS_W<'_, MMC_RX_INTERRUPTrs> {
        RXCRCERPIS_W::new(self, 5)
    }
    ///Bit 6 - MMC Receive Alignment Error Packet Counter Interrupt Status
    #[inline(always)]
    pub fn rxalgnerpis(&mut self) -> RXALGNERPIS_W<'_, MMC_RX_INTERRUPTrs> {
        RXALGNERPIS_W::new(self, 6)
    }
    ///Bit 17 - MMC Receive Unicast Good Packet Counter Interrupt Status
    #[inline(always)]
    pub fn rxucgpis(&mut self) -> RXUCGPIS_W<'_, MMC_RX_INTERRUPTrs> {
        RXUCGPIS_W::new(self, 17)
    }
    ///Bit 26 - MMC Receive LPI microsecond counter interrupt status
    #[inline(always)]
    pub fn rxlpiuscis(&mut self) -> RXLPIUSCIS_W<'_, MMC_RX_INTERRUPTrs> {
        RXLPIUSCIS_W::new(self, 26)
    }
    ///Bit 27 - MMC Receive LPI transition counter interrupt status
    #[inline(always)]
    pub fn rxlpitrcis(&mut self) -> RXLPITRCIS_W<'_, MMC_RX_INTERRUPTrs> {
        RXLPITRCIS_W::new(self, 27)
    }
}
/**MMC Rx interrupt register

You can [`read`](crate::Reg::read) this register and get [`mmc_rx_interrupt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_rx_interrupt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ETH:MMC_RX_INTERRUPT)*/
pub struct MMC_RX_INTERRUPTrs;
impl crate::RegisterSpec for MMC_RX_INTERRUPTrs {
    type Ux = u32;
}
///`read()` method returns [`mmc_rx_interrupt::R`](R) reader structure
impl crate::Readable for MMC_RX_INTERRUPTrs {}
///`write(|w| ..)` method takes [`mmc_rx_interrupt::W`](W) writer structure
impl crate::Writable for MMC_RX_INTERRUPTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMC_RX_INTERRUPT to value 0
impl crate::Resettable for MMC_RX_INTERRUPTrs {}
