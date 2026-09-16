///Register `DMAMUX_RGCFR` writer
pub type W = crate::W<DMAMUX_RGCFRrs>;
///Field `COF0` writer - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register.
pub type COF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COF1` writer - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register.
pub type COF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COF2` writer - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register.
pub type COF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COF3` writer - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register.
pub type COF3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<DMAMUX_RGCFRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register.
    #[inline(always)]
    pub fn cof0(&mut self) -> COF0_W<'_, DMAMUX_RGCFRrs> {
        COF0_W::new(self, 0)
    }
    ///Bit 1 - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register.
    #[inline(always)]
    pub fn cof1(&mut self) -> COF1_W<'_, DMAMUX_RGCFRrs> {
        COF1_W::new(self, 1)
    }
    ///Bit 2 - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register.
    #[inline(always)]
    pub fn cof2(&mut self) -> COF2_W<'_, DMAMUX_RGCFRrs> {
        COF2_W::new(self, 2)
    }
    ///Bit 3 - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register.
    #[inline(always)]
    pub fn cof3(&mut self) -> COF3_W<'_, DMAMUX_RGCFRrs> {
        COF3_W::new(self, 3)
    }
}
/**DMAMUX request generator interrupt clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamux_rgcfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#DMAMUX:DMAMUX_RGCFR)*/
pub struct DMAMUX_RGCFRrs;
impl crate::RegisterSpec for DMAMUX_RGCFRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`dmamux_rgcfr::W`](W) writer structure
impl crate::Writable for DMAMUX_RGCFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAMUX_RGCFR to value 0
impl crate::Resettable for DMAMUX_RGCFRrs {}
