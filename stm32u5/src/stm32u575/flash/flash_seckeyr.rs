#[doc = "Register `FLASH_SECKEYR` writer"]
pub type W = crate::W<FLASH_SECKEYRrs>;
#[doc = "Field `SECKEY` writer - Flash memory secure key"]
pub type SECKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Flash memory secure key"]
    #[inline(always)]
    #[must_use]
    pub fn seckey(&mut self) -> SECKEY_W<FLASH_SECKEYRrs> {
        SECKEY_W::new(self, 0)
    }
}
#[doc = "FLASH secure key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_seckeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_SECKEYRrs;
impl crate::RegisterSpec for FLASH_SECKEYRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flash_seckeyr::W`](W) writer structure"]
impl crate::Writable for FLASH_SECKEYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_SECKEYR to value 0"]
impl crate::Resettable for FLASH_SECKEYRrs {
    const RESET_VALUE: u32 = 0;
}
