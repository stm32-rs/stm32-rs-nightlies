#[doc = "Register `APB3RSTR` reader"]
pub type R = crate::R<APB3RSTRrs>;
#[doc = "Register `APB3RSTR` writer"]
pub type W = crate::W<APB3RSTRrs>;
#[doc = "SBS block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBSRST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<SBSRST> for bool {
    #[inline(always)]
    fn from(variant: SBSRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBSRST` reader - SBS block reset Set and reset by software."]
pub type SBSRST_R = crate::BitReader<SBSRST>;
impl SBSRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SBSRST> {
        match self.bits {
            true => Some(SBSRST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SBSRST::Reset
    }
}
#[doc = "Field `SBSRST` writer - SBS block reset Set and reset by software."]
pub type SBSRST_W<'a, REG> = crate::BitWriter<'a, REG, SBSRST>;
impl<'a, REG> SBSRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SBSRST::Reset)
    }
}
#[doc = "Field `SPI5RST` reader - SPI5 block reset Set and reset by software."]
pub use SBSRST_R as SPI5RST_R;
#[doc = "Field `LPUART1RST` reader - LPUART1 block reset Set and reset by software."]
pub use SBSRST_R as LPUART1RST_R;
#[doc = "Field `I2C3RST` reader - I2C3 block reset Set and reset by software."]
pub use SBSRST_R as I2C3RST_R;
#[doc = "Field `I2C4RST` reader - I2C4 block reset Set and reset by software."]
pub use SBSRST_R as I2C4RST_R;
#[doc = "Field `LPTIM1RST` reader - LPTIM1 block reset Set and reset by software."]
pub use SBSRST_R as LPTIM1RST_R;
#[doc = "Field `LPTIM3RST` reader - LPTIM3 block reset Set and reset by software."]
pub use SBSRST_R as LPTIM3RST_R;
#[doc = "Field `LPTIM4RST` reader - LPTIM4 block reset Set and reset by software."]
pub use SBSRST_R as LPTIM4RST_R;
#[doc = "Field `LPTIM5RST` reader - LPTIM5 block reset Set and reset by software."]
pub use SBSRST_R as LPTIM5RST_R;
#[doc = "Field `LPTIM6RST` reader - LPTIM6 block reset Set and reset by software."]
pub use SBSRST_R as LPTIM6RST_R;
#[doc = "Field `VREFRST` reader - VREF block reset Set and reset by software."]
pub use SBSRST_R as VREFRST_R;
#[doc = "Field `SPI5RST` writer - SPI5 block reset Set and reset by software."]
pub use SBSRST_W as SPI5RST_W;
#[doc = "Field `LPUART1RST` writer - LPUART1 block reset Set and reset by software."]
pub use SBSRST_W as LPUART1RST_W;
#[doc = "Field `I2C3RST` writer - I2C3 block reset Set and reset by software."]
pub use SBSRST_W as I2C3RST_W;
#[doc = "Field `I2C4RST` writer - I2C4 block reset Set and reset by software."]
pub use SBSRST_W as I2C4RST_W;
#[doc = "Field `LPTIM1RST` writer - LPTIM1 block reset Set and reset by software."]
pub use SBSRST_W as LPTIM1RST_W;
#[doc = "Field `LPTIM3RST` writer - LPTIM3 block reset Set and reset by software."]
pub use SBSRST_W as LPTIM3RST_W;
#[doc = "Field `LPTIM4RST` writer - LPTIM4 block reset Set and reset by software."]
pub use SBSRST_W as LPTIM4RST_W;
#[doc = "Field `LPTIM5RST` writer - LPTIM5 block reset Set and reset by software."]
pub use SBSRST_W as LPTIM5RST_W;
#[doc = "Field `LPTIM6RST` writer - LPTIM6 block reset Set and reset by software."]
pub use SBSRST_W as LPTIM6RST_W;
#[doc = "Field `VREFRST` writer - VREF block reset Set and reset by software."]
pub use SBSRST_W as VREFRST_W;
impl R {
    #[doc = "Bit 1 - SBS block reset Set and reset by software."]
    #[inline(always)]
    pub fn sbsrst(&self) -> SBSRST_R {
        SBSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI5 block reset Set and reset by software."]
    #[inline(always)]
    pub fn spi5rst(&self) -> SPI5RST_R {
        SPI5RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LPUART1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C3 block reset Set and reset by software."]
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C4 block reset Set and reset by software."]
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPTIM3 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lptim3rst(&self) -> LPTIM3RST_R {
        LPTIM3RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LPTIM4 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lptim4rst(&self) -> LPTIM4RST_R {
        LPTIM4RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LPTIM5 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lptim5rst(&self) -> LPTIM5RST_R {
        LPTIM5RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LPTIM6 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lptim6rst(&self) -> LPTIM6RST_R {
        LPTIM6RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - VREF block reset Set and reset by software."]
    #[inline(always)]
    pub fn vrefrst(&self) -> VREFRST_R {
        VREFRST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SBS block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sbsrst(&mut self) -> SBSRST_W<APB3RSTRrs> {
        SBSRST_W::new(self, 1)
    }
    #[doc = "Bit 5 - SPI5 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi5rst(&mut self) -> SPI5RST_W<APB3RSTRrs> {
        SPI5RST_W::new(self, 5)
    }
    #[doc = "Bit 6 - LPUART1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<APB3RSTRrs> {
        LPUART1RST_W::new(self, 6)
    }
    #[doc = "Bit 7 - I2C3 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<APB3RSTRrs> {
        I2C3RST_W::new(self, 7)
    }
    #[doc = "Bit 8 - I2C4 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c4rst(&mut self) -> I2C4RST_W<APB3RSTRrs> {
        I2C4RST_W::new(self, 8)
    }
    #[doc = "Bit 11 - LPTIM1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<APB3RSTRrs> {
        LPTIM1RST_W::new(self, 11)
    }
    #[doc = "Bit 12 - LPTIM3 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim3rst(&mut self) -> LPTIM3RST_W<APB3RSTRrs> {
        LPTIM3RST_W::new(self, 12)
    }
    #[doc = "Bit 13 - LPTIM4 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim4rst(&mut self) -> LPTIM4RST_W<APB3RSTRrs> {
        LPTIM4RST_W::new(self, 13)
    }
    #[doc = "Bit 14 - LPTIM5 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim5rst(&mut self) -> LPTIM5RST_W<APB3RSTRrs> {
        LPTIM5RST_W::new(self, 14)
    }
    #[doc = "Bit 15 - LPTIM6 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim6rst(&mut self) -> LPTIM6RST_W<APB3RSTRrs> {
        LPTIM6RST_W::new(self, 15)
    }
    #[doc = "Bit 20 - VREF block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn vrefrst(&mut self) -> VREFRST_W<APB3RSTRrs> {
        VREFRST_W::new(self, 20)
    }
}
#[doc = "RCC APB4 peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB3RSTRrs;
impl crate::RegisterSpec for APB3RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb3rstr::R`](R) reader structure"]
impl crate::Readable for APB3RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`apb3rstr::W`](W) writer structure"]
impl crate::Writable for APB3RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB3RSTR to value 0"]
impl crate::Resettable for APB3RSTRrs {
    const RESET_VALUE: u32 = 0;
}
