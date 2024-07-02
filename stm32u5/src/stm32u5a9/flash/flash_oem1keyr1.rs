///Register `FLASH_OEM1KEYR1` writer
pub type W = crate::W<FLASH_OEM1KEYR1rs>;
///Field `OEM1KEY` writer - OEM1 least significant bytes key
pub type OEM1KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<FLASH_OEM1KEYR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - OEM1 least significant bytes key
    #[inline(always)]
    #[must_use]
    pub fn oem1key(&mut self) -> OEM1KEY_W<FLASH_OEM1KEYR1rs> {
        OEM1KEY_W::new(self, 0)
    }
}
/**FLASH OEM1 key register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_oem1keyr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#FLASH:FLASH_OEM1KEYR1)*/
pub struct FLASH_OEM1KEYR1rs;
impl crate::RegisterSpec for FLASH_OEM1KEYR1rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`flash_oem1keyr1::W`](W) writer structure
impl crate::Writable for FLASH_OEM1KEYR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FLASH_OEM1KEYR1 to value 0
impl crate::Resettable for FLASH_OEM1KEYR1rs {
    const RESET_VALUE: u32 = 0;
}
