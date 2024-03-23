#[doc = "Register `FLASH_OPTKEYR` writer"]
pub type W = crate::W<FLASH_OPTKEYRrs>;
#[doc = "Field `OPTKEY` writer - Option byte key The following values must be written consecutively to unlock the FLASH_OPTR register allowing option byte programming/erasing operations: KEY1: 0x0819 2A3B KEY2: 0x4C5D 6E7F"]
pub type OPTKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Option byte key The following values must be written consecutively to unlock the FLASH_OPTR register allowing option byte programming/erasing operations: KEY1: 0x0819 2A3B KEY2: 0x4C5D 6E7F"]
    #[inline(always)]
    #[must_use]
    pub fn optkey(&mut self) -> OPTKEY_W<FLASH_OPTKEYRrs> {
        OPTKEY_W::new(self, 0)
    }
}
#[doc = "FLASH option key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_optkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_OPTKEYRrs;
impl crate::RegisterSpec for FLASH_OPTKEYRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flash_optkeyr::W`](W) writer structure"]
impl crate::Writable for FLASH_OPTKEYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_OPTKEYR to value 0"]
impl crate::Resettable for FLASH_OPTKEYRrs {
    const RESET_VALUE: u32 = 0;
}
