#[doc = "Register `CSSA` reader"]
pub type R = crate::R<CSSArs>;
#[doc = "Register `CSSA` writer"]
pub type W = crate::W<CSSArs>;
#[doc = "Field `ADD` reader - code segment start address"]
pub type ADD_R = crate::FieldReader<u16>;
#[doc = "Field `ADD` writer - code segment start address"]
pub type ADD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 8:23 - code segment start address"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:23 - code segment start address"]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<CSSArs> {
        ADD_W::new(self, 8)
    }
}
#[doc = "Code segment start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cssa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cssa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSSArs;
impl crate::RegisterSpec for CSSArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cssa::R`](R) reader structure"]
impl crate::Readable for CSSArs {}
#[doc = "`write(|w| ..)` method takes [`cssa::W`](W) writer structure"]
impl crate::Writable for CSSArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSSA to value 0"]
impl crate::Resettable for CSSArs {
    const RESET_VALUE: u32 = 0;
}
