///Register `BKPSRAMEDEAR` writer
pub type W = crate::W<BKPSRAMEDEARrs>;
///Field `EDEA` writer - ECC double error address
pub type EDEA_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl core::fmt::Debug for crate::generic::Reg<BKPSRAMEDEARrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:10 - ECC double error address
    #[inline(always)]
    pub fn edea(&mut self) -> EDEA_W<'_, BKPSRAMEDEARrs> {
        EDEA_W::new(self, 0)
    }
}
/**RAMCFG BKPSRAM double error address register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkpsramedear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:BKPSRAMEDEAR)*/
pub struct BKPSRAMEDEARrs;
impl crate::RegisterSpec for BKPSRAMEDEARrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`bkpsramedear::W`](W) writer structure
impl crate::Writable for BKPSRAMEDEARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BKPSRAMEDEAR to value 0
impl crate::Resettable for BKPSRAMEDEARrs {}
