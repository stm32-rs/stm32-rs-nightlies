#[doc = "Register `APB1RSTR2` reader"]
pub type R = crate::R<APB1RSTR2rs>;
#[doc = "Register `APB1RSTR2` writer"]
pub type W = crate::W<APB1RSTR2rs>;
#[doc = "Low-power UART 1 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1RST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset LPUART1"]
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
    pub const fn variant(&self) -> LPUART1RST {
        match self.bits {
            false => LPUART1RST::NoEffect,
            true => LPUART1RST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == LPUART1RST::NoEffect
    }
    #[doc = "Reset LPUART1"]
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
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1RST::NoEffect)
    }
    #[doc = "Reset LPUART1"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1RST::Reset)
    }
}
#[doc = "I2C4 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C4RST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset I2C4"]
    Reset = 1,
}
impl From<I2C4RST> for bool {
    #[inline(always)]
    fn from(variant: I2C4RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C4RST` reader - I2C4 reset"]
pub type I2C4RST_R = crate::BitReader<I2C4RST>;
impl I2C4RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C4RST {
        match self.bits {
            false => I2C4RST::NoEffect,
            true => I2C4RST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == I2C4RST::NoEffect
    }
    #[doc = "Reset I2C4"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == I2C4RST::Reset
    }
}
#[doc = "Field `I2C4RST` writer - I2C4 reset"]
pub type I2C4RST_W<'a, REG> = crate::BitWriter<'a, REG, I2C4RST>;
impl<'a, REG> I2C4RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(I2C4RST::NoEffect)
    }
    #[doc = "Reset I2C4"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(I2C4RST::Reset)
    }
}
#[doc = "Low-power timer 2 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM2RST {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Reset LPTIM2"]
    Reset = 1,
}
impl From<LPTIM2RST> for bool {
    #[inline(always)]
    fn from(variant: LPTIM2RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM2RST` reader - Low-power timer 2 reset"]
pub type LPTIM2RST_R = crate::BitReader<LPTIM2RST>;
impl LPTIM2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM2RST {
        match self.bits {
            false => LPTIM2RST::NoEffect,
            true => LPTIM2RST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == LPTIM2RST::NoEffect
    }
    #[doc = "Reset LPTIM2"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LPTIM2RST::Reset
    }
}
#[doc = "Field `LPTIM2RST` writer - Low-power timer 2 reset"]
pub type LPTIM2RST_W<'a, REG> = crate::BitWriter<'a, REG, LPTIM2RST>;
impl<'a, REG> LPTIM2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2RST::NoEffect)
    }
    #[doc = "Reset LPTIM2"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2RST::Reset)
    }
}
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
    #[doc = "Bit 5 - Low-power timer 2 reset"]
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 5) & 1) != 0)
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
    #[doc = "Bit 5 - Low-power timer 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W<APB1RSTR2rs> {
        LPTIM2RST_W::new(self, 5)
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
