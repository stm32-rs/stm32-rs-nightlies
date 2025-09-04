///Register `OEM2KEYR1` writer
pub type W = crate::W<OEM2KEYR1rs>;
///Field `OEM2KEY` writer - OEM2 key least significant bytes
pub type OEM2KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<OEM2KEYR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - OEM2 key least significant bytes
    #[inline(always)]
    pub fn oem2key(&mut self) -> OEM2KEY_W<OEM2KEYR1rs> {
        OEM2KEY_W::new(self, 0)
    }
}
/**FLASH OEM2 key register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oem2keyr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#FLASH:OEM2KEYR1)*/
pub struct OEM2KEYR1rs;
impl crate::RegisterSpec for OEM2KEYR1rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`oem2keyr1::W`](W) writer structure
impl crate::Writable for OEM2KEYR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OEM2KEYR1 to value 0
impl crate::Resettable for OEM2KEYR1rs {}
