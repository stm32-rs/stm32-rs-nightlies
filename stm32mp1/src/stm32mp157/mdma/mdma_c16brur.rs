#[doc = "Register `MDMA_C16BRUR` reader"]
pub type R = crate::R<MDMA_C16BRURrs>;
#[doc = "Register `MDMA_C16BRUR` writer"]
pub type W = crate::W<MDMA_C16BRURrs>;
#[doc = "Field `SUV` reader - SUV"]
pub type SUV_R = crate::FieldReader<u16>;
#[doc = "Field `SUV` writer - SUV"]
pub type SUV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DUV` reader - DUV"]
pub type DUV_R = crate::FieldReader<u16>;
#[doc = "Field `DUV` writer - DUV"]
pub type DUV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SUV"]
    #[inline(always)]
    pub fn suv(&self) -> SUV_R {
        SUV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DUV"]
    #[inline(always)]
    pub fn duv(&self) -> DUV_R {
        DUV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SUV"]
    #[inline(always)]
    #[must_use]
    pub fn suv(&mut self) -> SUV_W<MDMA_C16BRURrs> {
        SUV_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - DUV"]
    #[inline(always)]
    #[must_use]
    pub fn duv(&mut self) -> DUV_W<MDMA_C16BRURrs> {
        DUV_W::new(self, 16)
    }
}
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x10).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c16brur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c16brur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C16BRURrs;
impl crate::RegisterSpec for MDMA_C16BRURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c16brur::R`](R) reader structure"]
impl crate::Readable for MDMA_C16BRURrs {}
#[doc = "`write(|w| ..)` method takes [`mdma_c16brur::W`](W) writer structure"]
impl crate::Writable for MDMA_C16BRURrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMA_C16BRUR to value 0"]
impl crate::Resettable for MDMA_C16BRURrs {
    const RESET_VALUE: u32 = 0;
}
