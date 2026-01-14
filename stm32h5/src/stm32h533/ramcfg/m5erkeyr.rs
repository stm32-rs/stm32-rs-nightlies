///Register `M5ERKEYR` writer
pub type W = crate::W<M5ERKEYRrs>;
///Field `ERASEKEY` writer - Erase write protection key
pub type ERASEKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<M5ERKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Erase write protection key
    #[inline(always)]
    pub fn erasekey(&mut self) -> ERASEKEY_W<'_, M5ERKEYRrs> {
        ERASEKEY_W::new(self, 0)
    }
}
/**RAMCFG memory 5 erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5erkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#RAMCFG:M5ERKEYR)*/
pub struct M5ERKEYRrs;
impl crate::RegisterSpec for M5ERKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`m5erkeyr::W`](W) writer structure
impl crate::Writable for M5ERKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M5ERKEYR to value 0
impl crate::Resettable for M5ERKEYRrs {}
