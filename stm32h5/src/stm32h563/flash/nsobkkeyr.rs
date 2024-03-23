#[doc = "Register `NSOBKKEYR` writer"]
pub type W = crate::W<NSOBKKEYRrs>;
#[doc = "Field `NSOBKKEY` writer - FLASH non-secure option bytes keys control access unlock key"]
pub type NSOBKKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - FLASH non-secure option bytes keys control access unlock key"]
    #[inline(always)]
    #[must_use]
    pub fn nsobkkey(&mut self) -> NSOBKKEY_W<NSOBKKEYRrs> {
        NSOBKKEY_W::new(self, 0)
    }
}
#[doc = "FLASH non-secure OBK key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nsobkkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NSOBKKEYRrs;
impl crate::RegisterSpec for NSOBKKEYRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`nsobkkeyr::W`](W) writer structure"]
impl crate::Writable for NSOBKKEYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NSOBKKEYR to value 0"]
impl crate::Resettable for NSOBKKEYRrs {
    const RESET_VALUE: u32 = 0;
}
