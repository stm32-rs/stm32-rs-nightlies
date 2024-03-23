#[doc = "Register `OPTR` reader"]
pub type R = crate::R<OPTRrs>;
#[doc = "Register `OPTR` writer"]
pub type W = crate::W<OPTRrs>;
#[doc = "Field `OR` reader - OR"]
pub type OR_R = crate::FieldReader;
#[doc = "Field `OR` writer - OR"]
pub type OR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - OR"]
    #[inline(always)]
    pub fn or(&self) -> OR_R {
        OR_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - OR"]
    #[inline(always)]
    #[must_use]
    pub fn or(&mut self) -> OR_W<OPTRrs> {
        OR_W::new(self, 0)
    }
}
#[doc = "Option Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTRrs;
impl crate::RegisterSpec for OPTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optr::R`](R) reader structure"]
impl crate::Readable for OPTRrs {}
#[doc = "`write(|w| ..)` method takes [`optr::W`](W) writer structure"]
impl crate::Writable for OPTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPTR to value 0"]
impl crate::Resettable for OPTRrs {
    const RESET_VALUE: u32 = 0;
}
