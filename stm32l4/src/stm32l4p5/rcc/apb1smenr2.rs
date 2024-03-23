#[doc = "Register `APB1SMENR2` reader"]
pub type R = crate::R<APB1SMENR2rs>;
#[doc = "Register `APB1SMENR2` writer"]
pub type W = crate::W<APB1SMENR2rs>;
#[doc = "Low power UART 1 clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1SMEN {
    #[doc = "0: LPUART1 clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: LPUART1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<LPUART1SMEN> for bool {
    #[inline(always)]
    fn from(variant: LPUART1SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART1SMEN` reader - Low power UART 1 clocks enable during Sleep and Stop modes"]
pub type LPUART1SMEN_R = crate::BitReader<LPUART1SMEN>;
impl LPUART1SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPUART1SMEN {
        match self.bits {
            false => LPUART1SMEN::Disabled,
            true => LPUART1SMEN::Enabled,
        }
    }
    #[doc = "LPUART1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPUART1SMEN::Disabled
    }
    #[doc = "LPUART1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPUART1SMEN::Enabled
    }
}
#[doc = "Field `LPUART1SMEN` writer - Low power UART 1 clocks enable during Sleep and Stop modes"]
pub type LPUART1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, LPUART1SMEN>;
impl<'a, REG> LPUART1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPUART1 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SMEN::Disabled)
    }
    #[doc = "LPUART1 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SMEN::Enabled)
    }
}
#[doc = "I2C4 clocks enable during Sleep and Stop modes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C4SMEN {
    #[doc = "0: I2C4 clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: I2C4 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<I2C4SMEN> for bool {
    #[inline(always)]
    fn from(variant: I2C4SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C4SMEN` reader - I2C4 clocks enable during Sleep and Stop modes"]
pub type I2C4SMEN_R = crate::BitReader<I2C4SMEN>;
impl I2C4SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C4SMEN {
        match self.bits {
            false => I2C4SMEN::Disabled,
            true => I2C4SMEN::Enabled,
        }
    }
    #[doc = "I2C4 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C4SMEN::Disabled
    }
    #[doc = "I2C4 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C4SMEN::Enabled
    }
}
#[doc = "Field `I2C4SMEN` writer - I2C4 clocks enable during Sleep and Stop modes"]
pub type I2C4SMEN_W<'a, REG> = crate::BitWriter<'a, REG, I2C4SMEN>;
impl<'a, REG> I2C4SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C4 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C4SMEN::Disabled)
    }
    #[doc = "I2C4 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C4SMEN::Enabled)
    }
}
#[doc = "LPTIM2SMEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM2SMEN {
    #[doc = "0: LPTIM2 clocks disabled by the clock gating during Sleep and Stop modes"]
    Disabled = 0,
    #[doc = "1: LPTIM2 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    Enabled = 1,
}
impl From<LPTIM2SMEN> for bool {
    #[inline(always)]
    fn from(variant: LPTIM2SMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM2SMEN` reader - LPTIM2SMEN"]
pub type LPTIM2SMEN_R = crate::BitReader<LPTIM2SMEN>;
impl LPTIM2SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM2SMEN {
        match self.bits {
            false => LPTIM2SMEN::Disabled,
            true => LPTIM2SMEN::Enabled,
        }
    }
    #[doc = "LPTIM2 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPTIM2SMEN::Disabled
    }
    #[doc = "LPTIM2 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPTIM2SMEN::Enabled
    }
}
#[doc = "Field `LPTIM2SMEN` writer - LPTIM2SMEN"]
pub type LPTIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG, LPTIM2SMEN>;
impl<'a, REG> LPTIM2SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPTIM2 clocks disabled by the clock gating during Sleep and Stop modes"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SMEN::Disabled)
    }
    #[doc = "LPTIM2 clocks enabled by the clock gating(1) during Sleep and Stop modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SMEN::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C4 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c4smen(&self) -> I2C4SMEN_R {
        I2C4SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - LPTIM2SMEN"]
    #[inline(always)]
    pub fn lptim2smen(&self) -> LPTIM2SMEN_R {
        LPTIM2SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W<APB1SMENR2rs> {
        LPUART1SMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C4 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4smen(&mut self) -> I2C4SMEN_W<APB1SMENR2rs> {
        I2C4SMEN_W::new(self, 1)
    }
    #[doc = "Bit 5 - LPTIM2SMEN"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2smen(&mut self) -> LPTIM2SMEN_W<APB1SMENR2rs> {
        LPTIM2SMEN_W::new(self, 5)
    }
}
#[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1smenr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1smenr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1SMENR2rs;
impl crate::RegisterSpec for APB1SMENR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1smenr2::R`](R) reader structure"]
impl crate::Readable for APB1SMENR2rs {}
#[doc = "`write(|w| ..)` method takes [`apb1smenr2::W`](W) writer structure"]
impl crate::Writable for APB1SMENR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1SMENR2 to value 0x23"]
impl crate::Resettable for APB1SMENR2rs {
    const RESET_VALUE: u32 = 0x23;
}
