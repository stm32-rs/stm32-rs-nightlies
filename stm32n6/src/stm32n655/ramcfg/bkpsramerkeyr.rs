///Register `BKPSRAMERKEYR` writer
pub type W = crate::W<BKPSRAMERKEYRrs>;
///Field `ERASEKEY` writer - Erase write protection key
pub type ERASEKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<BKPSRAMERKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Erase write protection key
    #[inline(always)]
    pub fn erasekey(&mut self) -> ERASEKEY_W<BKPSRAMERKEYRrs> {
        ERASEKEY_W::new(self, 0)
    }
}
/**RAMCFG BKPSRAM erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkpsramerkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RAMCFG:BKPSRAMERKEYR)*/
pub struct BKPSRAMERKEYRrs;
impl crate::RegisterSpec for BKPSRAMERKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`bkpsramerkeyr::W`](W) writer structure
impl crate::Writable for BKPSRAMERKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BKPSRAMERKEYR to value 0
impl crate::Resettable for BKPSRAMERKEYRrs {}
