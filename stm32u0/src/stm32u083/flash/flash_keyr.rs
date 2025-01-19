///Register `FLASH_KEYR` writer
pub type W = crate::W<FLASH_KEYRrs>;
///Field `KEY` writer - FLASH key The following values must be written consecutively to unlock the FLASH control register (FLASH_CR), thus enabling programming/erasing operations: KEY1: 0x4567 0123 KEY2: 0xCDEF 89AB
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<FLASH_KEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - FLASH key The following values must be written consecutively to unlock the FLASH control register (FLASH_CR), thus enabling programming/erasing operations: KEY1: 0x4567 0123 KEY2: 0xCDEF 89AB
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<FLASH_KEYRrs> {
        KEY_W::new(self, 0)
    }
}
/**FLASH key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_keyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#FLASH:FLASH_KEYR)*/
pub struct FLASH_KEYRrs;
impl crate::RegisterSpec for FLASH_KEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`flash_keyr::W`](W) writer structure
impl crate::Writable for FLASH_KEYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FLASH_KEYR to value 0
impl crate::Resettable for FLASH_KEYRrs {
    const RESET_VALUE: u32 = 0;
}
