///Register `AHBSRAM1ERKEYR` writer
pub type W = crate::W<AHBSRAM1ERKEYRrs>;
///Field `ERASEKEY` writer - Erase write protection key
pub type ERASEKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<AHBSRAM1ERKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Erase write protection key
    #[inline(always)]
    pub fn erasekey(&mut self) -> ERASEKEY_W<'_, AHBSRAM1ERKEYRrs> {
        ERASEKEY_W::new(self, 0)
    }
}
/**RAMCFG AHBSRAM1 erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbsram1erkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RAMCFG:AHBSRAM1ERKEYR)*/
pub struct AHBSRAM1ERKEYRrs;
impl crate::RegisterSpec for AHBSRAM1ERKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahbsram1erkeyr::W`](W) writer structure
impl crate::Writable for AHBSRAM1ERKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHBSRAM1ERKEYR to value 0
impl crate::Resettable for AHBSRAM1ERKEYRrs {}
