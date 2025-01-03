///Register `RGCFR` writer
pub type W = crate::W<RGCFRrs>;
///Field `COF0` writer - Clear trigger Overrun Flag 0
pub type COF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COF1` writer - Clear trigger Overrun Flag 1
pub type COF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COF2` writer - Clear trigger Overrun Flag 2
pub type COF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COF3` writer - Clear trigger Overrun Flag 3
pub type COF3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<RGCFRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear trigger Overrun Flag 0
    #[inline(always)]
    pub fn cof0(&mut self) -> COF0_W<RGCFRrs> {
        COF0_W::new(self, 0)
    }
    ///Bit 1 - Clear trigger Overrun Flag 1
    #[inline(always)]
    pub fn cof1(&mut self) -> COF1_W<RGCFRrs> {
        COF1_W::new(self, 1)
    }
    ///Bit 2 - Clear trigger Overrun Flag 2
    #[inline(always)]
    pub fn cof2(&mut self) -> COF2_W<RGCFRrs> {
        COF2_W::new(self, 2)
    }
    ///Bit 3 - Clear trigger Overrun Flag 3
    #[inline(always)]
    pub fn cof3(&mut self) -> COF3_W<RGCFRrs> {
        COF3_W::new(self, 3)
    }
}
/**DMA Request Generator Clear Flag Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#DMAMUX1:RGCFR)*/
pub struct RGCFRrs;
impl crate::RegisterSpec for RGCFRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`rgcfr::W`](W) writer structure
impl crate::Writable for RGCFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RGCFR to value 0
impl crate::Resettable for RGCFRrs {
    const RESET_VALUE: u32 = 0;
}
