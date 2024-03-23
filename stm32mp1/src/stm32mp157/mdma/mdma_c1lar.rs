#[doc = "Register `MDMA_C1LAR` reader"]
pub type R = crate::R<MDMA_C1LARrs>;
#[doc = "Register `MDMA_C1LAR` writer"]
pub type W = crate::W<MDMA_C1LARrs>;
#[doc = "Field `LAR` reader - LAR"]
pub type LAR_R = crate::FieldReader<u32>;
#[doc = "Field `LAR` writer - LAR"]
pub type LAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - LAR"]
    #[inline(always)]
    pub fn lar(&self) -> LAR_R {
        LAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - LAR"]
    #[inline(always)]
    #[must_use]
    pub fn lar(&mut self) -> LAR_W<MDMA_C1LARrs> {
        LAR_W::new(self, 0)
    }
}
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c1lar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c1lar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C1LARrs;
impl crate::RegisterSpec for MDMA_C1LARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c1lar::R`](R) reader structure"]
impl crate::Readable for MDMA_C1LARrs {}
#[doc = "`write(|w| ..)` method takes [`mdma_c1lar::W`](W) writer structure"]
impl crate::Writable for MDMA_C1LARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMA_C1LAR to value 0"]
impl crate::Resettable for MDMA_C1LARrs {
    const RESET_VALUE: u32 = 0;
}
