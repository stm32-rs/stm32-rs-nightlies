///Register `DMA_S3M0AR` reader
pub type R = crate::R<DMA_S3M0ARrs>;
///Register `DMA_S3M0AR` writer
pub type W = crate::W<DMA_S3M0ARrs>;
///Field `M0A` reader - M0A
pub type M0A_R = crate::FieldReader<u32>;
///Field `M0A` writer - M0A
pub type M0A_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - M0A
    #[inline(always)]
    pub fn m0a(&self) -> M0A_R {
        M0A_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_S3M0AR")
            .field("m0a", &self.m0a())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - M0A
    #[inline(always)]
    #[must_use]
    pub fn m0a(&mut self) -> M0A_W<DMA_S3M0ARrs> {
        M0A_W::new(self, 0)
    }
}
/**DMA stream 3 memory 0 address register

You can [`read`](crate::Reg::read) this register and get [`dma_s3m0ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_s3m0ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DMA1:DMA_S3M0AR)*/
pub struct DMA_S3M0ARrs;
impl crate::RegisterSpec for DMA_S3M0ARrs {
    type Ux = u32;
}
///`read()` method returns [`dma_s3m0ar::R`](R) reader structure
impl crate::Readable for DMA_S3M0ARrs {}
///`write(|w| ..)` method takes [`dma_s3m0ar::W`](W) writer structure
impl crate::Writable for DMA_S3M0ARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMA_S3M0AR to value 0
impl crate::Resettable for DMA_S3M0ARrs {
    const RESET_VALUE: u32 = 0;
}
