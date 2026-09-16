///Register `DMAC0RXDTPR` reader
pub type R = crate::R<DMAC0RXDTPRrs>;
///Register `DMAC0RXDTPR` writer
pub type W = crate::W<DMAC0RXDTPRrs>;
///Field `RDT` reader - Receive Descriptor Tail Pointer
pub type RDT_R = crate::FieldReader<u32>;
///Field `RDT` writer - Receive Descriptor Tail Pointer
pub type RDT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Receive Descriptor Tail Pointer
    #[inline(always)]
    pub fn rdt(&self) -> RDT_R {
        RDT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC0RXDTPR")
            .field("rdt", &self.rdt())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Receive Descriptor Tail Pointer
    #[inline(always)]
    pub fn rdt(&mut self) -> RDT_W<'_, DMAC0RXDTPRrs> {
        RDT_W::new(self, 0)
    }
}
/**Channel 0 R0 descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`dmac0rxdtpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0rxdtpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ETH:DMAC0RXDTPR)*/
pub struct DMAC0RXDTPRrs;
impl crate::RegisterSpec for DMAC0RXDTPRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac0rxdtpr::R`](R) reader structure
impl crate::Readable for DMAC0RXDTPRrs {}
///`write(|w| ..)` method takes [`dmac0rxdtpr::W`](W) writer structure
impl crate::Writable for DMAC0RXDTPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC0RXDTPR to value 0
impl crate::Resettable for DMAC0RXDTPRrs {}
