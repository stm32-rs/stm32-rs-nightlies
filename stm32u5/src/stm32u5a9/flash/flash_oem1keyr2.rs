#[doc = "Register `FLASH_OEM1KEYR2` writer"]
pub type W = crate::W<FLASH_OEM1KEYR2rs>;
#[doc = "Field `OEM1KEY` writer - OEM1 most significant bytes key"]
pub type OEM1KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - OEM1 most significant bytes key"]
    #[inline(always)]
    #[must_use]
    pub fn oem1key(&mut self) -> OEM1KEY_W<FLASH_OEM1KEYR2rs> {
        OEM1KEY_W::new(self, 0)
    }
}
#[doc = "FLASH OEM1 key register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_oem1keyr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_OEM1KEYR2rs;
impl crate::RegisterSpec for FLASH_OEM1KEYR2rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flash_oem1keyr2::W`](W) writer structure"]
impl crate::Writable for FLASH_OEM1KEYR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_OEM1KEYR2 to value 0"]
impl crate::Resettable for FLASH_OEM1KEYR2rs {
    const RESET_VALUE: u32 = 0;
}
