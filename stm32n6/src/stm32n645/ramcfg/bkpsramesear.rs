///Register `BKPSRAMESEAR` writer
pub type W = crate::W<BKPSRAMESEARrs>;
///Field `ESEA` writer - ECC single error address
pub type ESEA_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl core::fmt::Debug for crate::generic::Reg<BKPSRAMESEARrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:10 - ECC single error address
    #[inline(always)]
    pub fn esea(&mut self) -> ESEA_W<'_, BKPSRAMESEARrs> {
        ESEA_W::new(self, 0)
    }
}
/**RAMCFG BKPSRAM single error address register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkpsramesear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RAMCFG:BKPSRAMESEAR)*/
pub struct BKPSRAMESEARrs;
impl crate::RegisterSpec for BKPSRAMESEARrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`bkpsramesear::W`](W) writer structure
impl crate::Writable for BKPSRAMESEARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BKPSRAMESEAR to value 0
impl crate::Resettable for BKPSRAMESEARrs {}
