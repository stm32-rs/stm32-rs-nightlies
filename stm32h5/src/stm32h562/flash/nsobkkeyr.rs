///Register `NSOBKKEYR` writer
pub type W = crate::W<NSOBKKEYRrs>;
///Field `NSOBKKEY` writer - FLASH non-secure option bytes keys control access unlock key
pub type NSOBKKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<NSOBKKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - FLASH non-secure option bytes keys control access unlock key
    #[inline(always)]
    pub fn nsobkkey(&mut self) -> NSOBKKEY_W<'_, NSOBKKEYRrs> {
        NSOBKKEY_W::new(self, 0)
    }
}
/**FLASH non-secure OBK key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsobkkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FLASH:NSOBKKEYR)*/
pub struct NSOBKKEYRrs;
impl crate::RegisterSpec for NSOBKKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`nsobkkeyr::W`](W) writer structure
impl crate::Writable for NSOBKKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NSOBKKEYR to value 0
impl crate::Resettable for NSOBKKEYRrs {}
