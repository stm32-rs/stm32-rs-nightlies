///Register `FLEXRAMERKEYR` writer
pub type W = crate::W<FLEXRAMERKEYRrs>;
///Field `ERASEKEY` writer - Erase write protection key
pub type ERASEKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<FLEXRAMERKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Erase write protection key
    #[inline(always)]
    pub fn erasekey(&mut self) -> ERASEKEY_W<'_, FLEXRAMERKEYRrs> {
        ERASEKEY_W::new(self, 0)
    }
}
/**RAMCFG FLEXRAM erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexramerkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:FLEXRAMERKEYR)*/
pub struct FLEXRAMERKEYRrs;
impl crate::RegisterSpec for FLEXRAMERKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`flexramerkeyr::W`](W) writer structure
impl crate::Writable for FLEXRAMERKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLEXRAMERKEYR to value 0
impl crate::Resettable for FLEXRAMERKEYRrs {}
