#[doc = "Register `DIEPDMA5` reader"]
pub type R = crate::R<DIEPDMA5rs>;
#[doc = "Register `DIEPDMA5` writer"]
pub type W = crate::W<DIEPDMA5rs>;
#[doc = "Field `DMAADDR` reader - DMAADDR"]
pub type DMAADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR` writer - DMAADDR"]
pub type DMAADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMAADDR"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMAADDR"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<DIEPDMA5rs> {
        DMAADDR_W::new(self, 0)
    }
}
#[doc = "OTG device IN endpoint 5 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPDMA5rs;
impl crate::RegisterSpec for DIEPDMA5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepdma5::R`](R) reader structure"]
impl crate::Readable for DIEPDMA5rs {}
#[doc = "`write(|w| ..)` method takes [`diepdma5::W`](W) writer structure"]
impl crate::Writable for DIEPDMA5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPDMA5 to value 0"]
impl crate::Resettable for DIEPDMA5rs {
    const RESET_VALUE: u32 = 0;
}
