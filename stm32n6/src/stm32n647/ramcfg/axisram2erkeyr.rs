///Register `AXISRAM2ERKEYR` writer
pub type W = crate::W<AXISRAM2ERKEYRrs>;
///Field `ERASEKEY` writer - Erase write protection key
pub type ERASEKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<AXISRAM2ERKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Erase write protection key
    #[inline(always)]
    pub fn erasekey(&mut self) -> ERASEKEY_W<'_, AXISRAM2ERKEYRrs> {
        ERASEKEY_W::new(self, 0)
    }
}
/**RAMCFG AXISRAM2 erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axisram2erkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RAMCFG:AXISRAM2ERKEYR)*/
pub struct AXISRAM2ERKEYRrs;
impl crate::RegisterSpec for AXISRAM2ERKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`axisram2erkeyr::W`](W) writer structure
impl crate::Writable for AXISRAM2ERKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AXISRAM2ERKEYR to value 0
impl crate::Resettable for AXISRAM2ERKEYRrs {}
