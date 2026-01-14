///Register `OPTKEYR` writer
pub type W = crate::W<OPTKEYRrs>;
///Field `OCUKEY` writer - Options configuration unlock key Following values must be written to FLASH_OPTKEYR consecutively to unlock FLASH_OPTCR register: 1st key = 0x0819 2A3B 2nd key = 0x4C5D 6E7F Reads to this register returns zero. If above sequence is performed twice locks up the corresponding register/bit until the next system reset, and generates a bus error.
pub type OCUKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<OPTKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Options configuration unlock key Following values must be written to FLASH_OPTKEYR consecutively to unlock FLASH_OPTCR register: 1st key = 0x0819 2A3B 2nd key = 0x4C5D 6E7F Reads to this register returns zero. If above sequence is performed twice locks up the corresponding register/bit until the next system reset, and generates a bus error.
    #[inline(always)]
    pub fn ocukey(&mut self) -> OCUKEY_W<'_, OPTKEYRrs> {
        OCUKEY_W::new(self, 0)
    }
}
/**FLASH options key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FLASH:OPTKEYR)*/
pub struct OPTKEYRrs;
impl crate::RegisterSpec for OPTKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`optkeyr::W`](W) writer structure
impl crate::Writable for OPTKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTKEYR to value 0
impl crate::Resettable for OPTKEYRrs {}
