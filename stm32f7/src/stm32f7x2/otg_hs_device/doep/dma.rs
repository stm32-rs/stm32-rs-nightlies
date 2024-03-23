#[doc = "Register `DMA` reader"]
pub type R = crate::R<DMArs>;
#[doc = "Register `DMA` writer"]
pub type W = crate::W<DMArs>;
#[doc = "Field `DMAADDR` reader - DMA address"]
pub type DMAADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR` writer - DMA address"]
pub type DMAADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA address"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<DMArs> {
        DMAADDR_W::new(self, 0)
    }
}
#[doc = "OTG_HS device endpoint-1 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMArs;
impl crate::RegisterSpec for DMArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma::R`](R) reader structure"]
impl crate::Readable for DMArs {}
#[doc = "`write(|w| ..)` method takes [`dma::W`](W) writer structure"]
impl crate::Writable for DMArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA to value 0"]
impl crate::Resettable for DMArs {
    const RESET_VALUE: u32 = 0;
}
