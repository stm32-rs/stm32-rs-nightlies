///Register `PEKEYR` writer
pub type W = crate::W<PEKEYRrs>;
///Field `PEKEYR` writer - FLASH_PEC and data EEPROM key
pub type PEKEYR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<PEKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - FLASH_PEC and data EEPROM key
    #[inline(always)]
    pub fn pekeyr(&mut self) -> PEKEYR_W<'_, PEKEYRrs> {
        PEKEYR_W::new(self, 0)
    }
}
/**Program/erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pekeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x2.html#FLASH:PEKEYR)*/
pub struct PEKEYRrs;
impl crate::RegisterSpec for PEKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pekeyr::W`](W) writer structure
impl crate::Writable for PEKEYRrs {
    type Safety = crate::Safe;
}
///`reset()` method sets PEKEYR to value 0
impl crate::Resettable for PEKEYRrs {}
