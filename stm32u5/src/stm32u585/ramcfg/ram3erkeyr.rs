#[doc = "Register `RAM3ERKEYR` writer"]
pub type W = crate::W<RAM3ERKEYRrs>;
#[doc = "Field `ERASEKEY` writer - ERASEKEY"]
pub type ERASEKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - ERASEKEY"]
    #[inline(always)]
    #[must_use]
    pub fn erasekey(&mut self) -> ERASEKEY_W<RAM3ERKEYRrs> {
        ERASEKEY_W::new(self, 0)
    }
}
#[doc = "RAMCFG SRAM x erase key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram3erkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAM3ERKEYRrs;
impl crate::RegisterSpec for RAM3ERKEYRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ram3erkeyr::W`](W) writer structure"]
impl crate::Writable for RAM3ERKEYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAM3ERKEYR to value 0"]
impl crate::Resettable for RAM3ERKEYRrs {
    const RESET_VALUE: u32 = 0;
}
