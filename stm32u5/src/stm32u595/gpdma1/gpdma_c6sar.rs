#[doc = "Register `GPDMA_C6SAR` reader"]
pub type R = crate::R<GPDMA_C6SARrs>;
#[doc = "Register `GPDMA_C6SAR` writer"]
pub type W = crate::W<GPDMA_C6SARrs>;
#[doc = "Field `SA` reader - source address"]
pub type SA_R = crate::FieldReader<u32>;
#[doc = "Field `SA` writer - source address"]
pub type SA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - source address"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - source address"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<GPDMA_C6SARrs> {
        SA_W::new(self, 0)
    }
}
#[doc = "GPDMA channel x source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdma_c6sar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdma_c6sar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPDMA_C6SARrs;
impl crate::RegisterSpec for GPDMA_C6SARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdma_c6sar::R`](R) reader structure"]
impl crate::Readable for GPDMA_C6SARrs {}
#[doc = "`write(|w| ..)` method takes [`gpdma_c6sar::W`](W) writer structure"]
impl crate::Writable for GPDMA_C6SARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPDMA_C6SAR to value 0"]
impl crate::Resettable for GPDMA_C6SARrs {
    const RESET_VALUE: u32 = 0;
}
