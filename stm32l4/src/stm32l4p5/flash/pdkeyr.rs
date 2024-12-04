///Register `PDKEYR` writer
pub type W = crate::W<PDKEYRrs>;
///Field `PDKEY` writer - RUN_PD in FLASH_ACR key
pub type PDKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<PDKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - RUN_PD in FLASH_ACR key
    #[inline(always)]
    pub fn pdkey(&mut self) -> PDKEY_W<PDKEYRrs> {
        PDKEY_W::new(self, 0)
    }
}
/**Power down key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#FLASH:PDKEYR)*/
pub struct PDKEYRrs;
impl crate::RegisterSpec for PDKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pdkeyr::W`](W) writer structure
impl crate::Writable for PDKEYRrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PDKEYR to value 0
impl crate::Resettable for PDKEYRrs {
    const RESET_VALUE: u32 = 0;
}
