///Register `FMC_CSQCR` writer
pub type W = crate::W<FMC_CSQCRrs>;
///Field `CSQSTART` writer - CSQSTART
pub type CSQSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FMC_CSQCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - CSQSTART
    #[inline(always)]
    #[must_use]
    pub fn csqstart(&mut self) -> CSQSTART_W<FMC_CSQCRrs> {
        CSQSTART_W::new(self, 0)
    }
}
/**FMC NAND Command Sequencer Control Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc_csqcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FMC:FMC_CSQCR)*/
pub struct FMC_CSQCRrs;
impl crate::RegisterSpec for FMC_CSQCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fmc_csqcr::W`](W) writer structure
impl crate::Writable for FMC_CSQCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FMC_CSQCR to value 0
impl crate::Resettable for FMC_CSQCRrs {
    const RESET_VALUE: u32 = 0;
}
