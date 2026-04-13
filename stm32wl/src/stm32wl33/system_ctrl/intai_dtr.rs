///Register `INTAI_DTR` reader
pub type R = crate::R<INTAI_DTRrs>;
///Register `INTAI_DTR` writer
pub type W = crate::W<INTAI_DTRrs>;
///Field `TX_DT` reader - TX_DT: detection type on TX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level
pub type TX_DT_R = crate::BitReader;
///Field `TX_DT` writer - TX_DT: detection type on TX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level
pub type TX_DT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_DT` reader - RX_DT: detection type on RX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level
pub type RX_DT_R = crate::BitReader;
///Field `RX_DT` writer - RX_DT: detection type on RX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level
pub type RX_DT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP_DT` reader - COMP_DT: detection type on COMP_OUT (after COMP_POL selection) signal: 0: detection on edge (default). 1: detection on level
pub type COMP_DT_R = crate::BitReader;
///Field `COMP_DT` writer - COMP_DT: detection type on COMP_OUT (after COMP_POL selection) signal: 0: detection on edge (default). 1: detection on level
pub type COMP_DT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFIP_BUSY_STATUS_DT` reader - RFIP_BUSY_STATUS_DT: detection type on RFIP_BUSY_STATUS signal: 0: detection on edge (default). 1: detection on level
pub type RFIP_BUSY_STATUS_DT_R = crate::BitReader;
///Field `RFIP_BUSY_STATUS_DT` writer - RFIP_BUSY_STATUS_DT: detection type on RFIP_BUSY_STATUS signal: 0: detection on edge (default). 1: detection on level
pub type RFIP_BUSY_STATUS_DT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TX_DT: detection type on TX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level
    #[inline(always)]
    pub fn tx_dt(&self) -> TX_DT_R {
        TX_DT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RX_DT: detection type on RX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level
    #[inline(always)]
    pub fn rx_dt(&self) -> RX_DT_R {
        RX_DT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - COMP_DT: detection type on COMP_OUT (after COMP_POL selection) signal: 0: detection on edge (default). 1: detection on level
    #[inline(always)]
    pub fn comp_dt(&self) -> COMP_DT_R {
        COMP_DT_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RFIP_BUSY_STATUS_DT: detection type on RFIP_BUSY_STATUS signal: 0: detection on edge (default). 1: detection on level
    #[inline(always)]
    pub fn rfip_busy_status_dt(&self) -> RFIP_BUSY_STATUS_DT_R {
        RFIP_BUSY_STATUS_DT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTAI_DTR")
            .field("tx_dt", &self.tx_dt())
            .field("rx_dt", &self.rx_dt())
            .field("comp_dt", &self.comp_dt())
            .field("rfip_busy_status_dt", &self.rfip_busy_status_dt())
            .finish()
    }
}
impl W {
    ///Bit 0 - TX_DT: detection type on TX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level
    #[inline(always)]
    pub fn tx_dt(&mut self) -> TX_DT_W<'_, INTAI_DTRrs> {
        TX_DT_W::new(self, 0)
    }
    ///Bit 1 - RX_DT: detection type on RX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level
    #[inline(always)]
    pub fn rx_dt(&mut self) -> RX_DT_W<'_, INTAI_DTRrs> {
        RX_DT_W::new(self, 1)
    }
    ///Bit 4 - COMP_DT: detection type on COMP_OUT (after COMP_POL selection) signal: 0: detection on edge (default). 1: detection on level
    #[inline(always)]
    pub fn comp_dt(&mut self) -> COMP_DT_W<'_, INTAI_DTRrs> {
        COMP_DT_W::new(self, 4)
    }
    ///Bit 5 - RFIP_BUSY_STATUS_DT: detection type on RFIP_BUSY_STATUS signal: 0: detection on edge (default). 1: detection on level
    #[inline(always)]
    pub fn rfip_busy_status_dt(&mut self) -> RFIP_BUSY_STATUS_DT_W<'_, INTAI_DTRrs> {
        RFIP_BUSY_STATUS_DT_W::new(self, 5)
    }
}
/**INTAI_DTR register

You can [`read`](crate::Reg::read) this register and get [`intai_dtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intai_dtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:INTAI_DTR)*/
pub struct INTAI_DTRrs;
impl crate::RegisterSpec for INTAI_DTRrs {
    type Ux = u32;
}
///`read()` method returns [`intai_dtr::R`](R) reader structure
impl crate::Readable for INTAI_DTRrs {}
///`write(|w| ..)` method takes [`intai_dtr::W`](W) writer structure
impl crate::Writable for INTAI_DTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INTAI_DTR to value 0
impl crate::Resettable for INTAI_DTRrs {}
