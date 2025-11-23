///Register `DMA` reader
pub type R = crate::R<DMArs>;
///Register `DMA` writer
pub type W = crate::W<DMArs>;
///Field `DMAADDR` reader - DMA address
pub type DMAADDR_R = crate::FieldReader<u32>;
///Field `DMAADDR` writer - DMA address
pub type DMAADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DMA address
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DMA address
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<'_, DMArs> {
        DMAADDR_W::new(self, 0)
    }
}
/**OTG_HS host channel-0 DMA address register

You can [`read`](crate::Reg::read) this register and get [`dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMArs;
impl crate::RegisterSpec for DMArs {
    type Ux = u32;
}
///`read()` method returns [`dma::R`](R) reader structure
impl crate::Readable for DMArs {}
///`write(|w| ..)` method takes [`dma::W`](W) writer structure
impl crate::Writable for DMArs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMA to value 0
impl crate::Resettable for DMArs {}
