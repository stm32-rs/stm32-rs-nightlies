///Register `FLASH_SECKEYR` writer
pub type W = crate::W<FLASH_SECKEYRrs>;
///Field `SECKEY` writer - Flash memory secure key The following values must be written consecutively to unlock the FLASH_SECCR register, allowing the Flash memory secure programming/erasing operations: KEY1: 0x4567 0123 KEY2: 0xCDEF 89AB
pub type SECKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<FLASH_SECKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Flash memory secure key The following values must be written consecutively to unlock the FLASH_SECCR register, allowing the Flash memory secure programming/erasing operations: KEY1: 0x4567 0123 KEY2: 0xCDEF 89AB
    #[inline(always)]
    #[must_use]
    pub fn seckey(&mut self) -> SECKEY_W<FLASH_SECKEYRrs> {
        SECKEY_W::new(self, 0)
    }
}
/**FLASH secure key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_seckeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:FLASH_SECKEYR)*/
pub struct FLASH_SECKEYRrs;
impl crate::RegisterSpec for FLASH_SECKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`flash_seckeyr::W`](W) writer structure
impl crate::Writable for FLASH_SECKEYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FLASH_SECKEYR to value 0
impl crate::Resettable for FLASH_SECKEYRrs {
    const RESET_VALUE: u32 = 0;
}
