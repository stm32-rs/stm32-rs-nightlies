///Register `FMC_CSQICR` writer
pub type W = crate::W<FMC_CSQICRrs>;
///Field `CTCF` writer - CTCF
pub type CTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSCF` writer - CSCF
pub type CSCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSEF` writer - CSEF
pub type CSEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSUEF` writer - CSUEF
pub type CSUEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCMDTCF` writer - CCMDTCF
pub type CCMDTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FMC_CSQICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - CTCF
    #[inline(always)]
    #[must_use]
    pub fn ctcf(&mut self) -> CTCF_W<FMC_CSQICRrs> {
        CTCF_W::new(self, 0)
    }
    ///Bit 1 - CSCF
    #[inline(always)]
    #[must_use]
    pub fn cscf(&mut self) -> CSCF_W<FMC_CSQICRrs> {
        CSCF_W::new(self, 1)
    }
    ///Bit 2 - CSEF
    #[inline(always)]
    #[must_use]
    pub fn csef(&mut self) -> CSEF_W<FMC_CSQICRrs> {
        CSEF_W::new(self, 2)
    }
    ///Bit 3 - CSUEF
    #[inline(always)]
    #[must_use]
    pub fn csuef(&mut self) -> CSUEF_W<FMC_CSQICRrs> {
        CSUEF_W::new(self, 3)
    }
    ///Bit 4 - CCMDTCF
    #[inline(always)]
    #[must_use]
    pub fn ccmdtcf(&mut self) -> CCMDTCF_W<FMC_CSQICRrs> {
        CCMDTCF_W::new(self, 4)
    }
}
/**FMC NAND Command Sequencer Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc_csqicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FMC:FMC_CSQICR)*/
pub struct FMC_CSQICRrs;
impl crate::RegisterSpec for FMC_CSQICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fmc_csqicr::W`](W) writer structure
impl crate::Writable for FMC_CSQICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FMC_CSQICR to value 0
impl crate::Resettable for FMC_CSQICRrs {
    const RESET_VALUE: u32 = 0;
}
