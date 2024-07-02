///Register `FLASH_NSKEYR` writer
pub type W = crate::W<FLASH_NSKEYRrs>;
///Field `NSKEY` writer - Flash memory non-secure key
pub type NSKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<FLASH_NSKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Flash memory non-secure key
    #[inline(always)]
    #[must_use]
    pub fn nskey(&mut self) -> NSKEY_W<FLASH_NSKEYRrs> {
        NSKEY_W::new(self, 0)
    }
}
/**FLASH non-secure key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_nskeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#FLASH:FLASH_NSKEYR)*/
pub struct FLASH_NSKEYRrs;
impl crate::RegisterSpec for FLASH_NSKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`flash_nskeyr::W`](W) writer structure
impl crate::Writable for FLASH_NSKEYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FLASH_NSKEYR to value 0
impl crate::Resettable for FLASH_NSKEYRrs {
    const RESET_VALUE: u32 = 0;
}
