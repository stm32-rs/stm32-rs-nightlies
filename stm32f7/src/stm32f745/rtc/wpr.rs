#[doc = "Register `WPR` writer"]
pub type W = crate::W<WPRrs>;
#[doc = "Field `KEY` writer - Write protection key"]
pub type KEY_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Write protection key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<WPRrs> {
        KEY_W::new(self, 0)
    }
}
#[doc = "write protection register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPRrs;
impl crate::RegisterSpec for WPRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wpr::W`](W) writer structure"]
impl crate::Writable for WPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WPR to value 0"]
impl crate::Resettable for WPRrs {
    const RESET_VALUE: u32 = 0;
}
