///Register `APB5RSTR` reader
pub type R = crate::R<APB5RSTRrs>;
///Register `APB5RSTR` writer
pub type W = crate::W<APB5RSTRrs>;
/**LTDC block reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCRST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<LTDCRST> for bool {
    #[inline(always)]
    fn from(variant: LTDCRST) -> Self {
        variant as u8 != 0
    }
}
///Field `LTDCRST` reader - LTDC block reset
pub type LTDCRST_R = crate::BitReader<LTDCRST>;
impl LTDCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LTDCRST> {
        match self.bits {
            true => Some(LTDCRST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LTDCRST::Reset
    }
}
///Field `LTDCRST` writer - LTDC block reset
pub type LTDCRST_W<'a, REG> = crate::BitWriter<'a, REG, LTDCRST>;
impl<'a, REG> LTDCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCRST::Reset)
    }
}
///Field `DCMIPPRST` reader - DCMIPP block reset
pub use LTDCRST_R as DCMIPPRST_R;
///Field `GFXTIMRST` reader - GFXTIM block reset
pub use LTDCRST_R as GFXTIMRST_R;
///Field `DCMIPPRST` writer - DCMIPP block reset
pub use LTDCRST_W as DCMIPPRST_W;
///Field `GFXTIMRST` writer - GFXTIM block reset
pub use LTDCRST_W as GFXTIMRST_W;
impl R {
    ///Bit 1 - LTDC block reset
    #[inline(always)]
    pub fn ltdcrst(&self) -> LTDCRST_R {
        LTDCRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DCMIPP block reset
    #[inline(always)]
    pub fn dcmipprst(&self) -> DCMIPPRST_R {
        DCMIPPRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - GFXTIM block reset
    #[inline(always)]
    pub fn gfxtimrst(&self) -> GFXTIMRST_R {
        GFXTIMRST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB5RSTR")
            .field("ltdcrst", &self.ltdcrst())
            .field("dcmipprst", &self.dcmipprst())
            .field("gfxtimrst", &self.gfxtimrst())
            .finish()
    }
}
impl W {
    ///Bit 1 - LTDC block reset
    #[inline(always)]
    pub fn ltdcrst(&mut self) -> LTDCRST_W<'_, APB5RSTRrs> {
        LTDCRST_W::new(self, 1)
    }
    ///Bit 2 - DCMIPP block reset
    #[inline(always)]
    pub fn dcmipprst(&mut self) -> DCMIPPRST_W<'_, APB5RSTRrs> {
        DCMIPPRST_W::new(self, 2)
    }
    ///Bit 4 - GFXTIM block reset
    #[inline(always)]
    pub fn gfxtimrst(&mut self) -> GFXTIMRST_W<'_, APB5RSTRrs> {
        GFXTIMRST_W::new(self, 4)
    }
}
/**RCC APB5 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb5rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RCC:APB5RSTR)*/
pub struct APB5RSTRrs;
impl crate::RegisterSpec for APB5RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb5rstr::R`](R) reader structure
impl crate::Readable for APB5RSTRrs {}
///`write(|w| ..)` method takes [`apb5rstr::W`](W) writer structure
impl crate::Writable for APB5RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB5RSTR to value 0
impl crate::Resettable for APB5RSTRrs {}
