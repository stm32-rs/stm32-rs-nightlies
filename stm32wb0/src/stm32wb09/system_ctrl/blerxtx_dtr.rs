///Register `BLERXTX_DTR` reader
pub type R = crate::R<BLERXTX_DTRrs>;
///Register `BLERXTX_DTR` writer
pub type W = crate::W<BLERXTX_DTRrs>;
///Field `TX_DT` reader - TX_DT: detection type on TX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level
pub type TX_DT_R = crate::BitReader;
///Field `TX_DT` writer - TX_DT: detection type on TX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level
pub type TX_DT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_DT` reader - RX_DT: detection type on RX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level
pub type RX_DT_R = crate::BitReader;
///Field `RX_DT` writer - RX_DT: detection type on RX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level
pub type RX_DT_W<'a, REG> = crate::BitWriter<'a, REG>;
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLERXTX_DTR")
            .field("tx_dt", &self.tx_dt())
            .field("rx_dt", &self.rx_dt())
            .finish()
    }
}
impl W {
    ///Bit 0 - TX_DT: detection type on TX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level
    #[inline(always)]
    pub fn tx_dt(&mut self) -> TX_DT_W<'_, BLERXTX_DTRrs> {
        TX_DT_W::new(self, 0)
    }
    ///Bit 1 - RX_DT: detection type on RX_SEQUENCE signal: 0: detection on edge (default). 1: detection on level
    #[inline(always)]
    pub fn rx_dt(&mut self) -> RX_DT_W<'_, BLERXTX_DTRrs> {
        RX_DT_W::new(self, 1)
    }
}
/**BLERXTX_DTR register

You can [`read`](crate::Reg::read) this register and get [`blerxtx_dtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blerxtx_dtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#SYSTEM_CTRL:BLERXTX_DTR)*/
pub struct BLERXTX_DTRrs;
impl crate::RegisterSpec for BLERXTX_DTRrs {
    type Ux = u32;
}
///`read()` method returns [`blerxtx_dtr::R`](R) reader structure
impl crate::Readable for BLERXTX_DTRrs {}
///`write(|w| ..)` method takes [`blerxtx_dtr::W`](W) writer structure
impl crate::Writable for BLERXTX_DTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BLERXTX_DTR to value 0
impl crate::Resettable for BLERXTX_DTRrs {}
