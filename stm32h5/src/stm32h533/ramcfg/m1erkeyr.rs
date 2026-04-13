///Register `M1ERKEYR` writer
pub type W = crate::W<M1ERKEYRrs>;
///Field `ERASEKEY` writer - Erase write protection key
pub type ERASEKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<M1ERKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Erase write protection key
    #[inline(always)]
    pub fn erasekey(&mut self) -> ERASEKEY_W<'_, M1ERKEYRrs> {
        ERASEKEY_W::new(self, 0)
    }
}
/**RAMCFG memory 1 erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1erkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#RAMCFG:M1ERKEYR)*/
pub struct M1ERKEYRrs;
impl crate::RegisterSpec for M1ERKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`m1erkeyr::W`](W) writer structure
impl crate::Writable for M1ERKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M1ERKEYR to value 0
impl crate::Resettable for M1ERKEYRrs {}
