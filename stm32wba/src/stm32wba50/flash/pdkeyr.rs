///Register `PDKEYR` writer
pub type W = crate::W<PDKEYRrs>;
///Field `PDKEY1` writer - Flash power-down key The following values must be written consecutively to unlock the PDREQ bit in FLASH_ACR: PDKEY_1: 0x04152637 PDKEY_2: 0xFAFBFCFD
pub type PDKEY1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<PDKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Flash power-down key The following values must be written consecutively to unlock the PDREQ bit in FLASH_ACR: PDKEY_1: 0x04152637 PDKEY_2: 0xFAFBFCFD
    #[inline(always)]
    pub fn pdkey1(&mut self) -> PDKEY1_W<'_, PDKEYRrs> {
        PDKEY1_W::new(self, 0)
    }
}
/**FLASH power-down key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#FLASH:PDKEYR)*/
pub struct PDKEYRrs;
impl crate::RegisterSpec for PDKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pdkeyr::W`](W) writer structure
impl crate::Writable for PDKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDKEYR to value 0
impl crate::Resettable for PDKEYRrs {}
