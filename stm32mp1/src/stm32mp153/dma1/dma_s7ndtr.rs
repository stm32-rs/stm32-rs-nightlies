#[doc = "Register `DMA_S7NDTR` reader"]
pub type R = crate::R<DMA_S7NDTRrs>;
#[doc = "Register `DMA_S7NDTR` writer"]
pub type W = crate::W<DMA_S7NDTRrs>;
#[doc = "Field `NDT` reader - NDT"]
pub type NDT_R = crate::FieldReader<u16>;
#[doc = "Field `NDT` writer - NDT"]
pub type NDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - NDT"]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - NDT"]
    #[inline(always)]
    #[must_use]
    pub fn ndt(&mut self) -> NDT_W<DMA_S7NDTRrs> {
        NDT_W::new(self, 0)
    }
}
#[doc = "DMA stream 7 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s7ndtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s7ndtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_S7NDTRrs;
impl crate::RegisterSpec for DMA_S7NDTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_s7ndtr::R`](R) reader structure"]
impl crate::Readable for DMA_S7NDTRrs {}
#[doc = "`write(|w| ..)` method takes [`dma_s7ndtr::W`](W) writer structure"]
impl crate::Writable for DMA_S7NDTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_S7NDTR to value 0"]
impl crate::Resettable for DMA_S7NDTRrs {
    const RESET_VALUE: u32 = 0;
}
