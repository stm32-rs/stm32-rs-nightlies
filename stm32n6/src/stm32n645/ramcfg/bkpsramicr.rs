///Register `BKPSRAMICR` writer
pub type W = crate::W<BKPSRAMICRrs>;
///Field `CSED` writer - Clear ECC single-error interrupt
pub type CSED_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDED` writer - Clear ECC double-error interrupt
pub type CDED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<BKPSRAMICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear ECC single-error interrupt
    #[inline(always)]
    pub fn csed(&mut self) -> CSED_W<'_, BKPSRAMICRrs> {
        CSED_W::new(self, 0)
    }
    ///Bit 1 - Clear ECC double-error interrupt
    #[inline(always)]
    pub fn cded(&mut self) -> CDED_W<'_, BKPSRAMICRrs> {
        CDED_W::new(self, 1)
    }
}
/**RAMCFG BKPSRAM interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkpsramicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RAMCFG:BKPSRAMICR)*/
pub struct BKPSRAMICRrs;
impl crate::RegisterSpec for BKPSRAMICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`bkpsramicr::W`](W) writer structure
impl crate::Writable for BKPSRAMICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BKPSRAMICR to value 0
impl crate::Resettable for BKPSRAMICRrs {}
