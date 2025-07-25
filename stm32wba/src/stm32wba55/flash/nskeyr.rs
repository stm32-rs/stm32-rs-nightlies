///Register `NSKEYR` writer
pub type W = crate::W<NSKEYRrs>;
///Field `NSKEY` writer - Flash memory non-secure key The following values must be written consecutively to unlock the FLASH_NSCR1 register, allowing the Flash memory non-secure programming/erasing operations: KEY1: 0x45670123 KEY2: 0xCDEF89AB
pub type NSKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<NSKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Flash memory non-secure key The following values must be written consecutively to unlock the FLASH_NSCR1 register, allowing the Flash memory non-secure programming/erasing operations: KEY1: 0x45670123 KEY2: 0xCDEF89AB
    #[inline(always)]
    pub fn nskey(&mut self) -> NSKEY_W<NSKEYRrs> {
        NSKEY_W::new(self, 0)
    }
}
/**FLASH key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nskeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#FLASH:NSKEYR)*/
pub struct NSKEYRrs;
impl crate::RegisterSpec for NSKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`nskeyr::W`](W) writer structure
impl crate::Writable for NSKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NSKEYR to value 0
impl crate::Resettable for NSKEYRrs {}
