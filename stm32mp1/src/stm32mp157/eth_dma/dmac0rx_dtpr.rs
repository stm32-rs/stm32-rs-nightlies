///Register `DMAC0RxDTPR` reader
pub type R = crate::R<DMAC0RX_DTPRrs>;
///Register `DMAC0RxDTPR` writer
pub type W = crate::W<DMAC0RX_DTPRrs>;
///Field `RDT` reader - Receive Descriptor Tail Pointer
pub type RDT_R = crate::FieldReader<u32>;
///Field `RDT` writer - Receive Descriptor Tail Pointer
pub type RDT_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    ///Bits 3:31 - Receive Descriptor Tail Pointer
    #[inline(always)]
    pub fn rdt(&self) -> RDT_R {
        RDT_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC0RxDTPR")
            .field("rdt", &self.rdt())
            .finish()
    }
}
impl W {
    ///Bits 3:31 - Receive Descriptor Tail Pointer
    #[inline(always)]
    pub fn rdt(&mut self) -> RDT_W<'_, DMAC0RX_DTPRrs> {
        RDT_W::new(self, 3)
    }
}
/**Channel Rx descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`dmac0rx_dtpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0rx_dtpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_DMA:DMAC0RxDTPR)*/
pub struct DMAC0RX_DTPRrs;
impl crate::RegisterSpec for DMAC0RX_DTPRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac0rx_dtpr::R`](R) reader structure
impl crate::Readable for DMAC0RX_DTPRrs {}
///`write(|w| ..)` method takes [`dmac0rx_dtpr::W`](W) writer structure
impl crate::Writable for DMAC0RX_DTPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC0RxDTPR to value 0
impl crate::Resettable for DMAC0RX_DTPRrs {}
