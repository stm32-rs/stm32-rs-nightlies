///Register `BLERXTX_IBER` reader
pub type R = crate::R<BLERXTX_IBERrs>;
///Register `BLERXTX_IBER` writer
pub type W = crate::W<BLERXTX_IBERrs>;
///Field `TX_IBE` reader - TX_IBE: interrupt edge register on TX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges
pub type TX_IBE_R = crate::BitReader;
///Field `TX_IBE` writer - TX_IBE: interrupt edge register on TX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges
pub type TX_IBE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_IBE` reader - RX_IBE: interrupt edge register on RX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges
pub type RX_IBE_R = crate::BitReader;
///Field `RX_IBE` writer - RX_IBE: interrupt edge register on RX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges
pub type RX_IBE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TX_IBE: interrupt edge register on TX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges
    #[inline(always)]
    pub fn tx_ibe(&self) -> TX_IBE_R {
        TX_IBE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RX_IBE: interrupt edge register on RX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges
    #[inline(always)]
    pub fn rx_ibe(&self) -> RX_IBE_R {
        RX_IBE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLERXTX_IBER")
            .field("tx_ibe", &self.tx_ibe())
            .field("rx_ibe", &self.rx_ibe())
            .finish()
    }
}
impl W {
    ///Bit 0 - TX_IBE: interrupt edge register on TX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges
    #[inline(always)]
    pub fn tx_ibe(&mut self) -> TX_IBE_W<'_, BLERXTX_IBERrs> {
        TX_IBE_W::new(self, 0)
    }
    ///Bit 1 - RX_IBE: interrupt edge register on RX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges
    #[inline(always)]
    pub fn rx_ibe(&mut self) -> RX_IBE_W<'_, BLERXTX_IBERrs> {
        RX_IBE_W::new(self, 1)
    }
}
/**BLERXTX_IBER register

You can [`read`](crate::Reg::read) this register and get [`blerxtx_iber::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blerxtx_iber::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#SYSTEM_CTRL:BLERXTX_IBER)*/
pub struct BLERXTX_IBERrs;
impl crate::RegisterSpec for BLERXTX_IBERrs {
    type Ux = u32;
}
///`read()` method returns [`blerxtx_iber::R`](R) reader structure
impl crate::Readable for BLERXTX_IBERrs {}
///`write(|w| ..)` method takes [`blerxtx_iber::W`](W) writer structure
impl crate::Writable for BLERXTX_IBERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BLERXTX_IBER to value 0
impl crate::Resettable for BLERXTX_IBERrs {}
