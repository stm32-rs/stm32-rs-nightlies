///Register `INTAI_IBER` reader
pub type R = crate::R<INTAI_IBERrs>;
///Register `INTAI_IBER` writer
pub type W = crate::W<INTAI_IBERrs>;
///Field `TX_IBE` reader - TX_IBE: interrupt edge register on TX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges
pub type TX_IBE_R = crate::BitReader;
///Field `TX_IBE` writer - TX_IBE: interrupt edge register on TX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges
pub type TX_IBE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_IBE` reader - RX_IBE: interrupt edge register on RX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges
pub type RX_IBE_R = crate::BitReader;
///Field `RX_IBE` writer - RX_IBE: interrupt edge register on RX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges
pub type RX_IBE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP_IBE` reader - COMP_IBE: interrupt edge register on COMP_OUT signal: 0: detection on single edge (default). 1: detection on both edges
pub type COMP_IBE_R = crate::BitReader;
///Field `COMP_IBE` writer - COMP_IBE: interrupt edge register on COMP_OUT signal: 0: detection on single edge (default). 1: detection on both edges
pub type COMP_IBE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFIP_BUSY_STATUS_IBE` reader - RFIP_BUSY_STATUS_IBE: interrupt edge register on RFIP_BUSY_STATUS signal: 0: detection on single edge (default). 1: detection on both edges
pub type RFIP_BUSY_STATUS_IBE_R = crate::BitReader;
///Field `RFIP_BUSY_STATUS_IBE` writer - RFIP_BUSY_STATUS_IBE: interrupt edge register on RFIP_BUSY_STATUS signal: 0: detection on single edge (default). 1: detection on both edges
pub type RFIP_BUSY_STATUS_IBE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 4 - COMP_IBE: interrupt edge register on COMP_OUT signal: 0: detection on single edge (default). 1: detection on both edges
    #[inline(always)]
    pub fn comp_ibe(&self) -> COMP_IBE_R {
        COMP_IBE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RFIP_BUSY_STATUS_IBE: interrupt edge register on RFIP_BUSY_STATUS signal: 0: detection on single edge (default). 1: detection on both edges
    #[inline(always)]
    pub fn rfip_busy_status_ibe(&self) -> RFIP_BUSY_STATUS_IBE_R {
        RFIP_BUSY_STATUS_IBE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTAI_IBER")
            .field("tx_ibe", &self.tx_ibe())
            .field("rx_ibe", &self.rx_ibe())
            .field("comp_ibe", &self.comp_ibe())
            .field("rfip_busy_status_ibe", &self.rfip_busy_status_ibe())
            .finish()
    }
}
impl W {
    ///Bit 0 - TX_IBE: interrupt edge register on TX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges
    #[inline(always)]
    pub fn tx_ibe(&mut self) -> TX_IBE_W<'_, INTAI_IBERrs> {
        TX_IBE_W::new(self, 0)
    }
    ///Bit 1 - RX_IBE: interrupt edge register on RX_SEQUENCE signal: 0: detection on single edge (default). 1: detection on both edges
    #[inline(always)]
    pub fn rx_ibe(&mut self) -> RX_IBE_W<'_, INTAI_IBERrs> {
        RX_IBE_W::new(self, 1)
    }
    ///Bit 4 - COMP_IBE: interrupt edge register on COMP_OUT signal: 0: detection on single edge (default). 1: detection on both edges
    #[inline(always)]
    pub fn comp_ibe(&mut self) -> COMP_IBE_W<'_, INTAI_IBERrs> {
        COMP_IBE_W::new(self, 4)
    }
    ///Bit 5 - RFIP_BUSY_STATUS_IBE: interrupt edge register on RFIP_BUSY_STATUS signal: 0: detection on single edge (default). 1: detection on both edges
    #[inline(always)]
    pub fn rfip_busy_status_ibe(&mut self) -> RFIP_BUSY_STATUS_IBE_W<'_, INTAI_IBERrs> {
        RFIP_BUSY_STATUS_IBE_W::new(self, 5)
    }
}
/**INTAI_IBER register

You can [`read`](crate::Reg::read) this register and get [`intai_iber::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intai_iber::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SYSTEM_CTRL:INTAI_IBER)*/
pub struct INTAI_IBERrs;
impl crate::RegisterSpec for INTAI_IBERrs {
    type Ux = u32;
}
///`read()` method returns [`intai_iber::R`](R) reader structure
impl crate::Readable for INTAI_IBERrs {}
///`write(|w| ..)` method takes [`intai_iber::W`](W) writer structure
impl crate::Writable for INTAI_IBERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INTAI_IBER to value 0
impl crate::Resettable for INTAI_IBERrs {}
