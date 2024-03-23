#[doc = "Register `FLASH_NSKEYR` writer"]
pub type W = crate::W<FLASH_NSKEYRrs>;
#[doc = "Field `NSKEY` writer - Flash memory non-secure key"]
pub type NSKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Flash memory non-secure key"]
    #[inline(always)]
    #[must_use]
    pub fn nskey(&mut self) -> NSKEY_W<FLASH_NSKEYRrs> {
        NSKEY_W::new(self, 0)
    }
}
#[doc = "FLASH non-secure key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_nskeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_NSKEYRrs;
impl crate::RegisterSpec for FLASH_NSKEYRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flash_nskeyr::W`](W) writer structure"]
impl crate::Writable for FLASH_NSKEYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_NSKEYR to value 0"]
impl crate::Resettable for FLASH_NSKEYRrs {
    const RESET_VALUE: u32 = 0;
}
