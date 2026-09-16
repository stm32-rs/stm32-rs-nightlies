///Register `AXISRAM1ERKEYR` writer
pub type W = crate::W<AXISRAM1ERKEYRrs>;
///Field `ERASEKEY` writer - Erase write protection key
pub type ERASEKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<AXISRAM1ERKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Erase write protection key
    #[inline(always)]
    pub fn erasekey(&mut self) -> ERASEKEY_W<'_, AXISRAM1ERKEYRrs> {
        ERASEKEY_W::new(self, 0)
    }
}
/**RAMCFG AXISRAM1 erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axisram1erkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RAMCFG:AXISRAM1ERKEYR)*/
pub struct AXISRAM1ERKEYRrs;
impl crate::RegisterSpec for AXISRAM1ERKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`axisram1erkeyr::W`](W) writer structure
impl crate::Writable for AXISRAM1ERKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AXISRAM1ERKEYR to value 0
impl crate::Resettable for AXISRAM1ERKEYRrs {}
