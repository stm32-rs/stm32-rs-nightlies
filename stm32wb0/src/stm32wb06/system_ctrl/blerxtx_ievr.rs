///Register `BLERXTX_IEVR` reader
pub type R = crate::R<BLERXTX_IEVRrs>;
///Register `BLERXTX_IEVR` writer
pub type W = crate::W<BLERXTX_IEVRrs>;
///Field `TX_IEV` reader - TX_IEV: interrupt polarity event on TX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level
pub type TX_IEV_R = crate::BitReader;
///Field `TX_IEV` writer - TX_IEV: interrupt polarity event on TX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level
pub type TX_IEV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_IEV` reader - RX_IEV: interrupt polarity event on RX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level
pub type RX_IEV_R = crate::BitReader;
///Field `RX_IEV` writer - RX_IEV: interrupt polarity event on RX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level
pub type RX_IEV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TX_IEV: interrupt polarity event on TX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level
    #[inline(always)]
    pub fn tx_iev(&self) -> TX_IEV_R {
        TX_IEV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RX_IEV: interrupt polarity event on RX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level
    #[inline(always)]
    pub fn rx_iev(&self) -> RX_IEV_R {
        RX_IEV_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLERXTX_IEVR")
            .field("tx_iev", &self.tx_iev())
            .field("rx_iev", &self.rx_iev())
            .finish()
    }
}
impl W {
    ///Bit 0 - TX_IEV: interrupt polarity event on TX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level
    #[inline(always)]
    pub fn tx_iev(&mut self) -> TX_IEV_W<'_, BLERXTX_IEVRrs> {
        TX_IEV_W::new(self, 0)
    }
    ///Bit 1 - RX_IEV: interrupt polarity event on RX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level
    #[inline(always)]
    pub fn rx_iev(&mut self) -> RX_IEV_W<'_, BLERXTX_IEVRrs> {
        RX_IEV_W::new(self, 1)
    }
}
/**BLERXTX_IEVR register

You can [`read`](crate::Reg::read) this register and get [`blerxtx_ievr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blerxtx_ievr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#SYSTEM_CTRL:BLERXTX_IEVR)*/
pub struct BLERXTX_IEVRrs;
impl crate::RegisterSpec for BLERXTX_IEVRrs {
    type Ux = u32;
}
///`read()` method returns [`blerxtx_ievr::R`](R) reader structure
impl crate::Readable for BLERXTX_IEVRrs {}
///`write(|w| ..)` method takes [`blerxtx_ievr::W`](W) writer structure
impl crate::Writable for BLERXTX_IEVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BLERXTX_IEVR to value 0
impl crate::Resettable for BLERXTX_IEVRrs {}
