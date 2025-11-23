///Register `DMAA4RXACR` reader
pub type R = crate::R<DMAA4RXACRrs>;
///Register `DMAA4RXACR` writer
pub type W = crate::W<DMAA4RXACRrs>;
///Field `RDWC` reader - Receive DMA Write Descriptor Cache Control
pub type RDWC_R = crate::FieldReader;
///Field `RDWC` writer - Receive DMA Write Descriptor Cache Control
pub type RDWC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RPC` reader - Receive DMA Payload Cache Control
pub type RPC_R = crate::FieldReader;
///Field `RPC` writer - Receive DMA Payload Cache Control
pub type RPC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RHC` reader - Receive DMA Header Cache Control
pub type RHC_R = crate::FieldReader;
///Field `RHC` writer - Receive DMA Header Cache Control
pub type RHC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RDC` reader - Receive DMA Buffer Cache Control
pub type RDC_R = crate::FieldReader;
///Field `RDC` writer - Receive DMA Buffer Cache Control
pub type RDC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Receive DMA Write Descriptor Cache Control
    #[inline(always)]
    pub fn rdwc(&self) -> RDWC_R {
        RDWC_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - Receive DMA Payload Cache Control
    #[inline(always)]
    pub fn rpc(&self) -> RPC_R {
        RPC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:19 - Receive DMA Header Cache Control
    #[inline(always)]
    pub fn rhc(&self) -> RHC_R {
        RHC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 24:27 - Receive DMA Buffer Cache Control
    #[inline(always)]
    pub fn rdc(&self) -> RDC_R {
        RDC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAA4RXACR")
            .field("rdwc", &self.rdwc())
            .field("rpc", &self.rpc())
            .field("rhc", &self.rhc())
            .field("rdc", &self.rdc())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Receive DMA Write Descriptor Cache Control
    #[inline(always)]
    pub fn rdwc(&mut self) -> RDWC_W<'_, DMAA4RXACRrs> {
        RDWC_W::new(self, 0)
    }
    ///Bits 8:11 - Receive DMA Payload Cache Control
    #[inline(always)]
    pub fn rpc(&mut self) -> RPC_W<'_, DMAA4RXACRrs> {
        RPC_W::new(self, 8)
    }
    ///Bits 16:19 - Receive DMA Header Cache Control
    #[inline(always)]
    pub fn rhc(&mut self) -> RHC_W<'_, DMAA4RXACRrs> {
        RHC_W::new(self, 16)
    }
    ///Bits 24:27 - Receive DMA Buffer Cache Control
    #[inline(always)]
    pub fn rdc(&mut self) -> RDC_W<'_, DMAA4RXACRrs> {
        RDC_W::new(self, 24)
    }
}
/**AXI4 receive channel ACE control register

You can [`read`](crate::Reg::read) this register and get [`dmaa4rxacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaa4rxacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAA4RXACR)*/
pub struct DMAA4RXACRrs;
impl crate::RegisterSpec for DMAA4RXACRrs {
    type Ux = u32;
}
///`read()` method returns [`dmaa4rxacr::R`](R) reader structure
impl crate::Readable for DMAA4RXACRrs {}
///`write(|w| ..)` method takes [`dmaa4rxacr::W`](W) writer structure
impl crate::Writable for DMAA4RXACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAA4RXACR to value 0
impl crate::Resettable for DMAA4RXACRrs {}
