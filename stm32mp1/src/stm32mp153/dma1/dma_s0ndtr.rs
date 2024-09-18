///Register `DMA_S0NDTR` reader
pub type R = crate::R<DMA_S0NDTRrs>;
///Register `DMA_S0NDTR` writer
pub type W = crate::W<DMA_S0NDTRrs>;
///Field `NDT` reader - NDT
pub type NDT_R = crate::FieldReader<u16>;
///Field `NDT` writer - NDT
pub type NDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - NDT
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_S0NDTR")
            .field("ndt", &self.ndt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - NDT
    #[inline(always)]
    #[must_use]
    pub fn ndt(&mut self) -> NDT_W<DMA_S0NDTRrs> {
        NDT_W::new(self, 0)
    }
}
/**DMA stream 0 number of data register

You can [`read`](crate::Reg::read) this register and get [`dma_s0ndtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_s0ndtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMA1:DMA_S0NDTR)*/
pub struct DMA_S0NDTRrs;
impl crate::RegisterSpec for DMA_S0NDTRrs {
    type Ux = u32;
}
///`read()` method returns [`dma_s0ndtr::R`](R) reader structure
impl crate::Readable for DMA_S0NDTRrs {}
///`write(|w| ..)` method takes [`dma_s0ndtr::W`](W) writer structure
impl crate::Writable for DMA_S0NDTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMA_S0NDTR to value 0
impl crate::Resettable for DMA_S0NDTRrs {
    const RESET_VALUE: u32 = 0;
}
