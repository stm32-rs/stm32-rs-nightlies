#[doc = "Register `DMA_S4M0AR` reader"]
pub type R = crate::R<DMA_S4M0ARrs>;
#[doc = "Register `DMA_S4M0AR` writer"]
pub type W = crate::W<DMA_S4M0ARrs>;
#[doc = "Field `M0A` reader - M0A"]
pub type M0A_R = crate::FieldReader<u32>;
#[doc = "Field `M0A` writer - M0A"]
pub type M0A_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - M0A"]
    #[inline(always)]
    pub fn m0a(&self) -> M0A_R {
        M0A_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - M0A"]
    #[inline(always)]
    #[must_use]
    pub fn m0a(&mut self) -> M0A_W<DMA_S4M0ARrs> {
        M0A_W::new(self, 0)
    }
}
#[doc = "DMA stream 4 memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s4m0ar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s4m0ar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_S4M0ARrs;
impl crate::RegisterSpec for DMA_S4M0ARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_s4m0ar::R`](R) reader structure"]
impl crate::Readable for DMA_S4M0ARrs {}
#[doc = "`write(|w| ..)` method takes [`dma_s4m0ar::W`](W) writer structure"]
impl crate::Writable for DMA_S4M0ARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_S4M0AR to value 0"]
impl crate::Resettable for DMA_S4M0ARrs {
    const RESET_VALUE: u32 = 0;
}
