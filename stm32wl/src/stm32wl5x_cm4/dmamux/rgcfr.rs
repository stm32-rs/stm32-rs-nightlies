///Register `RGCFR` writer
pub type W = crate::W<RGCFRrs>;
/**COF0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COF0 {
    ///1: Clear overrun flag
    Clear = 1,
}
impl From<COF0> for bool {
    #[inline(always)]
    fn from(variant: COF0) -> Self {
        variant as u8 != 0
    }
}
///Field `COF0` writer - COF0
pub type COF0_W<'a, REG> = crate::BitWriter<'a, REG, COF0>;
impl<'a, REG> COF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear overrun flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(COF0::Clear)
    }
}
///Field `COF1` writer - COF1
pub use COF0_W as COF1_W;
///Field `COF2` writer - COF2
pub use COF0_W as COF2_W;
///Field `COF3` writer - Clear trigger overrun event flag
pub use COF0_W as COF3_W;
impl core::fmt::Debug for crate::generic::Reg<RGCFRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - COF0
    #[inline(always)]
    pub fn cof0(&mut self) -> COF0_W<RGCFRrs> {
        COF0_W::new(self, 0)
    }
    ///Bit 1 - COF1
    #[inline(always)]
    pub fn cof1(&mut self) -> COF1_W<RGCFRrs> {
        COF1_W::new(self, 1)
    }
    ///Bit 2 - COF2
    #[inline(always)]
    pub fn cof2(&mut self) -> COF2_W<RGCFRrs> {
        COF2_W::new(self, 2)
    }
    ///Bit 3 - Clear trigger overrun event flag
    #[inline(always)]
    pub fn cof3(&mut self) -> COF3_W<RGCFRrs> {
        COF3_W::new(self, 3)
    }
}
/**request generator interrupt clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#DMAMUX:RGCFR)*/
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
