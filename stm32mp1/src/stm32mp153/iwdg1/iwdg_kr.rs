#[doc = "Register `IWDG_KR` writer"]
pub type W = crate::W<IWDG_KRrs>;
#[doc = "Field `KEY` writer - KEY"]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - KEY"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<IWDG_KRrs> {
        KEY_W::new(self, 0)
    }
}
#[doc = "Key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdg_kr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IWDG_KRrs;
impl crate::RegisterSpec for IWDG_KRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iwdg_kr::W`](W) writer structure"]
impl crate::Writable for IWDG_KRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IWDG_KR to value 0"]
impl crate::Resettable for IWDG_KRrs {
    const RESET_VALUE: u32 = 0;
}
