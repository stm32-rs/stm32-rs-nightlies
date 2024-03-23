#[doc = "Register `MDMA_C9SAR` reader"]
pub type R = crate::R<MDMA_C9SARrs>;
#[doc = "Register `MDMA_C9SAR` writer"]
pub type W = crate::W<MDMA_C9SARrs>;
#[doc = "Field `SAR` reader - SAR"]
pub type SAR_R = crate::FieldReader<u32>;
#[doc = "Field `SAR` writer - SAR"]
pub type SAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SAR"]
    #[inline(always)]
    pub fn sar(&self) -> SAR_R {
        SAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SAR"]
    #[inline(always)]
    #[must_use]
    pub fn sar(&mut self) -> SAR_W<MDMA_C9SARrs> {
        SAR_W::new(self, 0)
    }
}
#[doc = "In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x08).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c9sar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c9sar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C9SARrs;
impl crate::RegisterSpec for MDMA_C9SARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c9sar::R`](R) reader structure"]
impl crate::Readable for MDMA_C9SARrs {}
#[doc = "`write(|w| ..)` method takes [`mdma_c9sar::W`](W) writer structure"]
impl crate::Writable for MDMA_C9SARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMA_C9SAR to value 0"]
impl crate::Resettable for MDMA_C9SARrs {
    const RESET_VALUE: u32 = 0;
}
