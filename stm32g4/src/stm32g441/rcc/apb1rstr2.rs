#[doc = "Register `APB1RSTR2` reader"]
pub type R = crate::R<APB1RSTR2rs>;
#[doc = "Register `APB1RSTR2` writer"]
pub type W = crate::W<APB1RSTR2rs>;
#[doc = "Low-power UART 1 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1RST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<LPUART1RST> for bool {
    #[inline(always)]
    fn from(variant: LPUART1RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART1RST` reader - Low-power UART 1 reset"]
pub type LPUART1RST_R = crate::BitReader<LPUART1RST>;
impl LPUART1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPUART1RST> {
        match self.bits {
            true => Some(LPUART1RST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LPUART1RST::Reset
    }
}
#[doc = "Field `LPUART1RST` writer - Low-power UART 1 reset"]
pub type LPUART1RST_W<'a, REG> = crate::BitWriter<'a, REG, LPUART1RST>;
impl<'a, REG> LPUART1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1RST::Reset)
    }
}
#[doc = "Field `I2C4RST` reader - I2C4 reset"]
pub use LPUART1RST_R as I2C4RST_R;
#[doc = "Field `UCPD1RST` reader - UCPD1 reset"]
pub use LPUART1RST_R as UCPD1RST_R;
#[doc = "Field `I2C4RST` writer - I2C4 reset"]
pub use LPUART1RST_W as I2C4RST_W;
#[doc = "Field `UCPD1RST` writer - UCPD1 reset"]
pub use LPUART1RST_W as UCPD1RST_W;
impl R {
    #[doc = "Bit 0 - Low-power UART 1 reset"]
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C4 reset"]
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - UCPD1 reset"]
    #[inline(always)]
    pub fn ucpd1rst(&self) -> UCPD1RST_R {
        UCPD1RST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low-power UART 1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<APB1RSTR2rs> {
        LPUART1RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4rst(&mut self) -> I2C4RST_W<APB1RSTR2rs> {
        I2C4RST_W::new(self, 1)
    }
    #[doc = "Bit 8 - UCPD1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn ucpd1rst(&mut self) -> UCPD1RST_W<APB1RSTR2rs> {
        UCPD1RST_W::new(self, 8)
    }
}
#[doc = "APB1 peripheral reset register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rstr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rstr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1RSTR2rs;
impl crate::RegisterSpec for APB1RSTR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1rstr2::R`](R) reader structure"]
impl crate::Readable for APB1RSTR2rs {}
#[doc = "`write(|w| ..)` method takes [`apb1rstr2::W`](W) writer structure"]
impl crate::Writable for APB1RSTR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1RSTR2 to value 0"]
impl crate::Resettable for APB1RSTR2rs {
    const RESET_VALUE: u32 = 0;
}
