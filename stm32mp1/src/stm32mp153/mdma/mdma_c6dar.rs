#[doc = "Register `MDMA_C6DAR` reader"]
pub type R = crate::R<MDMA_C6DARrs>;
#[doc = "Register `MDMA_C6DAR` writer"]
pub type W = crate::W<MDMA_C6DARrs>;
#[doc = "Field `DAR` reader - DAR"]
pub type DAR_R = crate::FieldReader<u32>;
#[doc = "Field `DAR` writer - DAR"]
pub type DAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DAR"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DAR"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DAR_W<MDMA_C6DARrs> {
        DAR_W::new(self, 0)
    }
}
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c6dar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c6dar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C6DARrs;
impl crate::RegisterSpec for MDMA_C6DARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c6dar::R`](R) reader structure"]
impl crate::Readable for MDMA_C6DARrs {}
#[doc = "`write(|w| ..)` method takes [`mdma_c6dar::W`](W) writer structure"]
impl crate::Writable for MDMA_C6DARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMA_C6DAR to value 0"]
impl crate::Resettable for MDMA_C6DARrs {
    const RESET_VALUE: u32 = 0;
}
