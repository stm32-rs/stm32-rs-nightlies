#[doc = "Register `AF2` reader"]
pub type R = crate::R<AF2rs>;
#[doc = "Register `AF2` writer"]
pub type W = crate::W<AF2rs>;
#[doc = "Field `OCRSEL` reader - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to for product specific implementation."]
pub type OCRSEL_R = crate::FieldReader;
#[doc = "Field `OCRSEL` writer - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to for product specific implementation."]
pub type OCRSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
impl R {
    #[doc = "Bits 16:18 - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to for product specific implementation."]
    #[inline(always)]
    pub fn ocrsel(&self) -> OCRSEL_R {
        OCRSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to for product specific implementation."]
    #[inline(always)]
    #[must_use]
    pub fn ocrsel(&mut self) -> OCRSEL_W<AF2rs> {
        OCRSEL_W::new(self, 16)
    }
}
#[doc = "TIM5 alternate function register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF2rs;
impl crate::RegisterSpec for AF2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af2::R`](R) reader structure"]
impl crate::Readable for AF2rs {}
#[doc = "`write(|w| ..)` method takes [`af2::W`](W) writer structure"]
impl crate::Writable for AF2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AF2 to value 0"]
impl crate::Resettable for AF2rs {
    const RESET_VALUE: u32 = 0;
}
