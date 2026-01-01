///Register `CSQCR` writer
pub type W = crate::W<CSQCRrs>;
///Field `CSQSTART` writer - Command Sequencer Enable
pub type CSQSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CSQCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Command Sequencer Enable
    #[inline(always)]
    pub fn csqstart(&mut self) -> CSQSTART_W<'_, CSQCRrs> {
        CSQSTART_W::new(self, 0)
    }
}
/**FMC NAND command sequencer control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:CSQCR)*/
pub struct CSQCRrs;
impl crate::RegisterSpec for CSQCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`csqcr::W`](W) writer structure
impl crate::Writable for CSQCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSQCR to value 0
impl crate::Resettable for CSQCRrs {}
