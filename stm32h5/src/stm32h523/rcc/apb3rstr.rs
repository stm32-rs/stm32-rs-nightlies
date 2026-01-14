///Register `APB3RSTR` reader
pub type R = crate::R<APB3RSTRrs>;
///Register `APB3RSTR` writer
pub type W = crate::W<APB3RSTRrs>;
/**LPUART1 block reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1RST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<LPUART1RST> for bool {
    #[inline(always)]
    fn from(variant: LPUART1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `LPUART1RST` reader - LPUART1 block reset
pub type LPUART1RST_R = crate::BitReader<LPUART1RST>;
impl LPUART1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPUART1RST> {
        match self.bits {
            true => Some(LPUART1RST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LPUART1RST::Reset
    }
}
///Field `LPUART1RST` writer - LPUART1 block reset
pub type LPUART1RST_W<'a, REG> = crate::BitWriter<'a, REG, LPUART1RST>;
impl<'a, REG> LPUART1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1RST::Reset)
    }
}
///Field `I2C3RST` reader - I2C3 block reset
pub use LPUART1RST_R as I2C3RST_R;
///Field `I3C2RST` reader - I3C2 block reset
pub use LPUART1RST_R as I3C2RST_R;
///Field `LPTIM1RST` reader - LPTIM1 block reset
pub use LPUART1RST_R as LPTIM1RST_R;
///Field `LPTIM3RST` reader - LPTIM3 block reset
pub use LPUART1RST_R as LPTIM3RST_R;
///Field `LPTIM4RST` reader - LPTIM4 block reset
pub use LPUART1RST_R as LPTIM4RST_R;
///Field `LPTIM5RST` reader - LPTIM5 block reset
pub use LPUART1RST_R as LPTIM5RST_R;
///Field `LPTIM6RST` reader - LPTIM6 block reset
pub use LPUART1RST_R as LPTIM6RST_R;
///Field `VREFRST` reader - VREFBUF block reset
pub use LPUART1RST_R as VREFRST_R;
///Field `I2C3RST` writer - I2C3 block reset
pub use LPUART1RST_W as I2C3RST_W;
///Field `I3C2RST` writer - I3C2 block reset
pub use LPUART1RST_W as I3C2RST_W;
///Field `LPTIM1RST` writer - LPTIM1 block reset
pub use LPUART1RST_W as LPTIM1RST_W;
///Field `LPTIM3RST` writer - LPTIM3 block reset
pub use LPUART1RST_W as LPTIM3RST_W;
///Field `LPTIM4RST` writer - LPTIM4 block reset
pub use LPUART1RST_W as LPTIM4RST_W;
///Field `LPTIM5RST` writer - LPTIM5 block reset
pub use LPUART1RST_W as LPTIM5RST_W;
///Field `LPTIM6RST` writer - LPTIM6 block reset
pub use LPUART1RST_W as LPTIM6RST_W;
///Field `VREFRST` writer - VREFBUF block reset
pub use LPUART1RST_W as VREFRST_W;
impl R {
    ///Bit 6 - LPUART1 block reset
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C3 block reset
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - I3C2 block reset
    #[inline(always)]
    pub fn i3c2rst(&self) -> I3C2RST_R {
        I3C2RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - LPTIM1 block reset
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM3 block reset
    #[inline(always)]
    pub fn lptim3rst(&self) -> LPTIM3RST_R {
        LPTIM3RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - LPTIM4 block reset
    #[inline(always)]
    pub fn lptim4rst(&self) -> LPTIM4RST_R {
        LPTIM4RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - LPTIM5 block reset
    #[inline(always)]
    pub fn lptim5rst(&self) -> LPTIM5RST_R {
        LPTIM5RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - LPTIM6 block reset
    #[inline(always)]
    pub fn lptim6rst(&self) -> LPTIM6RST_R {
        LPTIM6RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 20 - VREFBUF block reset
    #[inline(always)]
    pub fn vrefrst(&self) -> VREFRST_R {
        VREFRST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB3RSTR")
            .field("lpuart1rst", &self.lpuart1rst())
            .field("i2c3rst", &self.i2c3rst())
            .field("i3c2rst", &self.i3c2rst())
            .field("lptim1rst", &self.lptim1rst())
            .field("lptim3rst", &self.lptim3rst())
            .field("lptim4rst", &self.lptim4rst())
            .field("lptim5rst", &self.lptim5rst())
            .field("lptim6rst", &self.lptim6rst())
            .field("vrefrst", &self.vrefrst())
            .finish()
    }
}
impl W {
    ///Bit 6 - LPUART1 block reset
    #[inline(always)]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<'_, APB3RSTRrs> {
        LPUART1RST_W::new(self, 6)
    }
    ///Bit 7 - I2C3 block reset
    #[inline(always)]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<'_, APB3RSTRrs> {
        I2C3RST_W::new(self, 7)
    }
    ///Bit 9 - I3C2 block reset
    #[inline(always)]
    pub fn i3c2rst(&mut self) -> I3C2RST_W<'_, APB3RSTRrs> {
        I3C2RST_W::new(self, 9)
    }
    ///Bit 11 - LPTIM1 block reset
    #[inline(always)]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<'_, APB3RSTRrs> {
        LPTIM1RST_W::new(self, 11)
    }
    ///Bit 12 - LPTIM3 block reset
    #[inline(always)]
    pub fn lptim3rst(&mut self) -> LPTIM3RST_W<'_, APB3RSTRrs> {
        LPTIM3RST_W::new(self, 12)
    }
    ///Bit 13 - LPTIM4 block reset
    #[inline(always)]
    pub fn lptim4rst(&mut self) -> LPTIM4RST_W<'_, APB3RSTRrs> {
        LPTIM4RST_W::new(self, 13)
    }
    ///Bit 14 - LPTIM5 block reset
    #[inline(always)]
    pub fn lptim5rst(&mut self) -> LPTIM5RST_W<'_, APB3RSTRrs> {
        LPTIM5RST_W::new(self, 14)
    }
    ///Bit 15 - LPTIM6 block reset
    #[inline(always)]
    pub fn lptim6rst(&mut self) -> LPTIM6RST_W<'_, APB3RSTRrs> {
        LPTIM6RST_W::new(self, 15)
    }
    ///Bit 20 - VREFBUF block reset
    #[inline(always)]
    pub fn vrefrst(&mut self) -> VREFRST_W<'_, APB3RSTRrs> {
        VREFRST_W::new(self, 20)
    }
}
/**RCC APB3 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb3rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#RCC:APB3RSTR)*/
pub struct APB3RSTRrs;
impl crate::RegisterSpec for APB3RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb3rstr::R`](R) reader structure
impl crate::Readable for APB3RSTRrs {}
///`write(|w| ..)` method takes [`apb3rstr::W`](W) writer structure
impl crate::Writable for APB3RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3RSTR to value 0
impl crate::Resettable for APB3RSTRrs {}
