#[doc = "Register `SECOBKKEYR` writer"]
pub type W = crate::W<SECOBKKEYRrs>;
#[doc = "Field `SECOBKKEY` writer - FLASH secure option bytes keys control access unlock key"]
pub type SECOBKKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - FLASH secure option bytes keys control access unlock key"]
    #[inline(always)]
    #[must_use]
    pub fn secobkkey(&mut self) -> SECOBKKEY_W<SECOBKKEYRrs> {
        SECOBKKEY_W::new(self, 0)
    }
}
#[doc = "FLASH secure OBK key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secobkkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECOBKKEYRrs;
impl crate::RegisterSpec for SECOBKKEYRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`secobkkeyr::W`](W) writer structure"]
impl crate::Writable for SECOBKKEYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECOBKKEYR to value 0"]
impl crate::Resettable for SECOBKKEYRrs {
    const RESET_VALUE: u32 = 0;
}
