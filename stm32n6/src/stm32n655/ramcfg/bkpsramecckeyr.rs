///Register `BKPSRAMECCKEYR` writer
pub type W = crate::W<BKPSRAMECCKEYRrs>;
///Field `ECCKEY` writer - ECC write protection key
pub type ECCKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<BKPSRAMECCKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - ECC write protection key
    #[inline(always)]
    pub fn ecckey(&mut self) -> ECCKEY_W<'_, BKPSRAMECCKEYRrs> {
        ECCKEY_W::new(self, 0)
    }
}
/**RAMCFG BKPSRAM ECC key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkpsramecckeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RAMCFG:BKPSRAMECCKEYR)*/
pub struct BKPSRAMECCKEYRrs;
impl crate::RegisterSpec for BKPSRAMECCKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`bkpsramecckeyr::W`](W) writer structure
impl crate::Writable for BKPSRAMECCKEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BKPSRAMECCKEYR to value 0
impl crate::Resettable for BKPSRAMECCKEYRrs {}
