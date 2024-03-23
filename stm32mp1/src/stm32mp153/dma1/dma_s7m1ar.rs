#[doc = "Register `DMA_S7M1AR` reader"]
pub type R = crate::R<DMA_S7M1ARrs>;
#[doc = "Register `DMA_S7M1AR` writer"]
pub type W = crate::W<DMA_S7M1ARrs>;
#[doc = "Field `M1A` reader - M1A"]
pub type M1A_R = crate::FieldReader<u32>;
#[doc = "Field `M1A` writer - M1A"]
pub type M1A_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - M1A"]
    #[inline(always)]
    pub fn m1a(&self) -> M1A_R {
        M1A_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - M1A"]
    #[inline(always)]
    #[must_use]
    pub fn m1a(&mut self) -> M1A_W<DMA_S7M1ARrs> {
        M1A_W::new(self, 0)
    }
}
#[doc = "DMA stream 7 memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s7m1ar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s7m1ar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_S7M1ARrs;
impl crate::RegisterSpec for DMA_S7M1ARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_s7m1ar::R`](R) reader structure"]
impl crate::Readable for DMA_S7M1ARrs {}
#[doc = "`write(|w| ..)` method takes [`dma_s7m1ar::W`](W) writer structure"]
impl crate::Writable for DMA_S7M1ARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_S7M1AR to value 0"]
impl crate::Resettable for DMA_S7M1ARrs {
    const RESET_VALUE: u32 = 0;
}
