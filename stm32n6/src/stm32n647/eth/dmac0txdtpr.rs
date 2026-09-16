///Register `DMAC0TXDTPR` reader
pub type R = crate::R<DMAC0TXDTPRrs>;
///Register `DMAC0TXDTPR` writer
pub type W = crate::W<DMAC0TXDTPRrs>;
///Field `TDT` reader - Transmit Descriptor Tail Pointer
pub type TDT_R = crate::FieldReader<u32>;
///Field `TDT` writer - Transmit Descriptor Tail Pointer
pub type TDT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Transmit Descriptor Tail Pointer
    #[inline(always)]
    pub fn tdt(&self) -> TDT_R {
        TDT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC0TXDTPR")
            .field("tdt", &self.tdt())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Transmit Descriptor Tail Pointer
    #[inline(always)]
    pub fn tdt(&mut self) -> TDT_W<'_, DMAC0TXDTPRrs> {
        TDT_W::new(self, 0)
    }
}
/**Channel 0 T0 descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`dmac0txdtpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0txdtpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ETH:DMAC0TXDTPR)*/
pub struct DMAC0TXDTPRrs;
impl crate::RegisterSpec for DMAC0TXDTPRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac0txdtpr::R`](R) reader structure
impl crate::Readable for DMAC0TXDTPRrs {}
///`write(|w| ..)` method takes [`dmac0txdtpr::W`](W) writer structure
impl crate::Writable for DMAC0TXDTPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC0TXDTPR to value 0
impl crate::Resettable for DMAC0TXDTPRrs {}
