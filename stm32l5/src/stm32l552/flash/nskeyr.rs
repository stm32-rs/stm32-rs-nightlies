#[doc = "Register `NSKEYR` writer"]
pub type W = crate::W<NSKEYRrs>;
#[doc = "Field `NSKEYR` writer - NSKEYR"]
pub type NSKEYR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - NSKEYR"]
    #[inline(always)]
    #[must_use]
    pub fn nskeyr(&mut self) -> NSKEYR_W<NSKEYRrs> {
        NSKEYR_W::new(self, 0)
    }
}
#[doc = "Flash non-secure key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nskeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NSKEYRrs;
impl crate::RegisterSpec for NSKEYRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`nskeyr::W`](W) writer structure"]
impl crate::Writable for NSKEYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NSKEYR to value 0"]
impl crate::Resettable for NSKEYRrs {
    const RESET_VALUE: u32 = 0;
}
