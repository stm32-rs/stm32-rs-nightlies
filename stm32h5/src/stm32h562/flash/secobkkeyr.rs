///Register `SECOBKKEYR` writer
pub type W = crate::W<SECOBKKEYRrs>;
///Field `SECOBKKEY` writer - FLASH secure option bytes keys control access unlock key
pub type SECOBKKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SECOBKKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - FLASH secure option bytes keys control access unlock key
    #[inline(always)]
    pub fn secobkkey(&mut self) -> SECOBKKEY_W<'_, SECOBKKEYRrs> {
        SECOBKKEY_W::new(self, 0)
    }
}
/**FLASH secure OBK key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secobkkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FLASH:SECOBKKEYR)*/
pub struct SECOBKKEYRrs;
impl crate::RegisterSpec for SECOBKKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`secobkkeyr::W`](W) writer structure
impl crate::Writable for SECOBKKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECOBKKEYR to value 0
impl crate::Resettable for SECOBKKEYRrs {}
