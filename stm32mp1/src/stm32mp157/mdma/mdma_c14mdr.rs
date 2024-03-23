#[doc = "Register `MDMA_C14MDR` reader"]
pub type R = crate::R<MDMA_C14MDRrs>;
#[doc = "Register `MDMA_C14MDR` writer"]
pub type W = crate::W<MDMA_C14MDRrs>;
#[doc = "Field `MDR` reader - MDR"]
pub type MDR_R = crate::FieldReader<u32>;
#[doc = "Field `MDR` writer - MDR"]
pub type MDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MDR"]
    #[inline(always)]
    pub fn mdr(&self) -> MDR_R {
        MDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MDR"]
    #[inline(always)]
    #[must_use]
    pub fn mdr(&mut self) -> MDR_W<MDMA_C14MDRrs> {
        MDR_W::new(self, 0)
    }
}
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c14mdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c14mdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C14MDRrs;
impl crate::RegisterSpec for MDMA_C14MDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c14mdr::R`](R) reader structure"]
impl crate::Readable for MDMA_C14MDRrs {}
#[doc = "`write(|w| ..)` method takes [`mdma_c14mdr::W`](W) writer structure"]
impl crate::Writable for MDMA_C14MDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMA_C14MDR to value 0"]
impl crate::Resettable for MDMA_C14MDRrs {
    const RESET_VALUE: u32 = 0;
}
