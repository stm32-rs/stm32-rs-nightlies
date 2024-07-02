///Register `FLASH_OEM2KEYR2` writer
pub type W = crate::W<FLASH_OEM2KEYR2rs>;
///Field `OEM2KEY` writer - OEM2 most significant bytes key
pub type OEM2KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<FLASH_OEM2KEYR2rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - OEM2 most significant bytes key
    #[inline(always)]
    #[must_use]
    pub fn oem2key(&mut self) -> OEM2KEY_W<FLASH_OEM2KEYR2rs> {
        OEM2KEY_W::new(self, 0)
    }
}
/**FLASH OEM2 key register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_oem2keyr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#FLASH:FLASH_OEM2KEYR2)*/
pub struct FLASH_OEM2KEYR2rs;
impl crate::RegisterSpec for FLASH_OEM2KEYR2rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`flash_oem2keyr2::W`](W) writer structure
impl crate::Writable for FLASH_OEM2KEYR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FLASH_OEM2KEYR2 to value 0
impl crate::Resettable for FLASH_OEM2KEYR2rs {
    const RESET_VALUE: u32 = 0;
}
