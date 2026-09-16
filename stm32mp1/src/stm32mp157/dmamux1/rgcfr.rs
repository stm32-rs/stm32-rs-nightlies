///Register `RGCFR` writer
pub type W = crate::W<RGCFRrs>;
///Field `COF0` writer - COF0
pub type COF0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COF1` writer - COF1
pub type COF1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COF2` writer - COF2
pub type COF2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COF3` writer - COF3
pub type COF3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COF4` writer - COF4
pub type COF4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COF5` writer - COF5
pub type COF5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COF6` writer - COF6
pub type COF6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COF7` writer - COF7
pub type COF7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<RGCFRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - COF0
    #[inline(always)]
    pub fn cof0(&mut self) -> COF0_W<'_, RGCFRrs> {
        COF0_W::new(self, 0)
    }
    ///Bit 1 - COF1
    #[inline(always)]
    pub fn cof1(&mut self) -> COF1_W<'_, RGCFRrs> {
        COF1_W::new(self, 1)
    }
    ///Bit 2 - COF2
    #[inline(always)]
    pub fn cof2(&mut self) -> COF2_W<'_, RGCFRrs> {
        COF2_W::new(self, 2)
    }
    ///Bit 3 - COF3
    #[inline(always)]
    pub fn cof3(&mut self) -> COF3_W<'_, RGCFRrs> {
        COF3_W::new(self, 3)
    }
    ///Bit 4 - COF4
    #[inline(always)]
    pub fn cof4(&mut self) -> COF4_W<'_, RGCFRrs> {
        COF4_W::new(self, 4)
    }
    ///Bit 5 - COF5
    #[inline(always)]
    pub fn cof5(&mut self) -> COF5_W<'_, RGCFRrs> {
        COF5_W::new(self, 5)
    }
    ///Bit 6 - COF6
    #[inline(always)]
    pub fn cof6(&mut self) -> COF6_W<'_, RGCFRrs> {
        COF6_W::new(self, 6)
    }
    ///Bit 7 - COF7
    #[inline(always)]
    pub fn cof7(&mut self) -> COF7_W<'_, RGCFRrs> {
        COF7_W::new(self, 7)
    }
}
/**DMAMUX request generator interrupt clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DMAMUX1:RGCFR)*/
pub struct RGCFRrs;
impl crate::RegisterSpec for RGCFRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`rgcfr::W`](W) writer structure
impl crate::Writable for RGCFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RGCFR to value 0
impl crate::Resettable for RGCFRrs {}
