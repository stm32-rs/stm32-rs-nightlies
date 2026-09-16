///Register `INTAI_IEVR` reader
pub type R = crate::R<INTAI_IEVRrs>;
///Register `INTAI_IEVR` writer
pub type W = crate::W<INTAI_IEVRrs>;
///Field `TX_IEV` reader - TX_IEV: interrupt polarity event on TX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level
pub type TX_IEV_R = crate::BitReader;
///Field `TX_IEV` writer - TX_IEV: interrupt polarity event on TX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level
pub type TX_IEV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_IEV` reader - RX_IEV: interrupt polarity event on RX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level
pub type RX_IEV_R = crate::BitReader;
///Field `RX_IEV` writer - RX_IEV: interrupt polarity event on RX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level
pub type RX_IEV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP_IEV` reader - COMP_IEV: interrupt polarity event on COMP_OUT signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level
pub type COMP_IEV_R = crate::BitReader;
///Field `COMP_IEV` writer - COMP_IEV: interrupt polarity event on COMP_OUT signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level
pub type COMP_IEV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFIP_BUSY_STATUS_IEV` reader - RFIP_BUSY_STATUS_IEV: interrupt polarity event on RFIP_BUSY_STATUS signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level
pub type RFIP_BUSY_STATUS_IEV_R = crate::BitReader;
///Field `RFIP_BUSY_STATUS_IEV` writer - RFIP_BUSY_STATUS_IEV: interrupt polarity event on RFIP_BUSY_STATUS signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level
pub type RFIP_BUSY_STATUS_IEV_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 4 - COMP_IEV: interrupt polarity event on COMP_OUT signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level
    #[inline(always)]
    pub fn comp_iev(&self) -> COMP_IEV_R {
        COMP_IEV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RFIP_BUSY_STATUS_IEV: interrupt polarity event on RFIP_BUSY_STATUS signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level
    #[inline(always)]
    pub fn rfip_busy_status_iev(&self) -> RFIP_BUSY_STATUS_IEV_R {
        RFIP_BUSY_STATUS_IEV_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTAI_IEVR")
            .field("tx_iev", &self.tx_iev())
            .field("rx_iev", &self.rx_iev())
            .field("comp_iev", &self.comp_iev())
            .field("rfip_busy_status_iev", &self.rfip_busy_status_iev())
            .finish()
    }
}
impl W {
    ///Bit 0 - TX_IEV: interrupt polarity event on TX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level
    #[inline(always)]
    pub fn tx_iev(&mut self) -> TX_IEV_W<'_, INTAI_IEVRrs> {
        TX_IEV_W::new(self, 0)
    }
    ///Bit 1 - RX_IEV: interrupt polarity event on RX_SEQUENCE signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level
    #[inline(always)]
    pub fn rx_iev(&mut self) -> RX_IEV_W<'_, INTAI_IEVRrs> {
        RX_IEV_W::new(self, 1)
    }
    ///Bit 4 - COMP_IEV: interrupt polarity event on COMP_OUT signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level
    #[inline(always)]
    pub fn comp_iev(&mut self) -> COMP_IEV_W<'_, INTAI_IEVRrs> {
        COMP_IEV_W::new(self, 4)
    }
    ///Bit 5 - RFIP_BUSY_STATUS_IEV: interrupt polarity event on RFIP_BUSY_STATUS signal: 0: detection on falling edge / low level (default). 1: detection on rising edge / high level
    #[inline(always)]
    pub fn rfip_busy_status_iev(&mut self) -> RFIP_BUSY_STATUS_IEV_W<'_, INTAI_IEVRrs> {
        RFIP_BUSY_STATUS_IEV_W::new(self, 5)
    }
}
/**INTAI_IEVR register

You can [`read`](crate::Reg::read) this register and get [`intai_ievr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intai_ievr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:INTAI_IEVR)*/
pub struct INTAI_IEVRrs;
impl crate::RegisterSpec for INTAI_IEVRrs {
    type Ux = u32;
}
///`read()` method returns [`intai_ievr::R`](R) reader structure
impl crate::Readable for INTAI_IEVRrs {}
///`write(|w| ..)` method takes [`intai_ievr::W`](W) writer structure
impl crate::Writable for INTAI_IEVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INTAI_IEVR to value 0
impl crate::Resettable for INTAI_IEVRrs {}
