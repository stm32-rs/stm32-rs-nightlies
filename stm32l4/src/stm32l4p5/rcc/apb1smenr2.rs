///Register `APB1SMENR2` reader
pub type R = crate::R<APB1SMENR2rs>;
///Register `APB1SMENR2` writer
pub type W = crate::W<APB1SMENR2rs>;
/**Low power UART 1 clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1SMEN {
    ///0: LPUART1 clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: LPUART1 clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<LPUART1SMEN> for bool {
    #[inline(always)]
    fn from(variant: LPUART1SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `LPUART1SMEN` reader - Low power UART 1 clocks enable during Sleep and Stop modes
pub type LPUART1SMEN_R = crate::BitReader<LPUART1SMEN>;
impl LPUART1SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPUART1SMEN {
        match self.bits {
            false => LPUART1SMEN::Disabled,
            true => LPUART1SMEN::Enabled,
        }
    }
    ///LPUART1 clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPUART1SMEN::Disabled
    }
    ///LPUART1 clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPUART1SMEN::Enabled
    }
}
///Field `LPUART1SMEN` writer - Low power UART 1 clocks enable during Sleep and Stop modes
pub type LPUART1SMEN_W<'a, REG> = crate::BitWriter<'a, REG, LPUART1SMEN>;
impl<'a, REG> LPUART1SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LPUART1 clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SMEN::Disabled)
    }
    ///LPUART1 clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SMEN::Enabled)
    }
}
/**I2C4 clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C4SMEN {
    ///0: I2C4 clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: I2C4 clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<I2C4SMEN> for bool {
    #[inline(always)]
    fn from(variant: I2C4SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C4SMEN` reader - I2C4 clocks enable during Sleep and Stop modes
pub type I2C4SMEN_R = crate::BitReader<I2C4SMEN>;
impl I2C4SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C4SMEN {
        match self.bits {
            false => I2C4SMEN::Disabled,
            true => I2C4SMEN::Enabled,
        }
    }
    ///I2C4 clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C4SMEN::Disabled
    }
    ///I2C4 clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C4SMEN::Enabled
    }
}
///Field `I2C4SMEN` writer - I2C4 clocks enable during Sleep and Stop modes
pub type I2C4SMEN_W<'a, REG> = crate::BitWriter<'a, REG, I2C4SMEN>;
impl<'a, REG> I2C4SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///I2C4 clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C4SMEN::Disabled)
    }
    ///I2C4 clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2C4SMEN::Enabled)
    }
}
/**LPTIM2SMEN

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM2SMEN {
    ///0: LPTIM2 clocks disabled by the clock gating during Sleep and Stop modes
    Disabled = 0,
    ///1: LPTIM2 clocks enabled by the clock gating(1) during Sleep and Stop modes
    Enabled = 1,
}
impl From<LPTIM2SMEN> for bool {
    #[inline(always)]
    fn from(variant: LPTIM2SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `LPTIM2SMEN` reader - LPTIM2SMEN
pub type LPTIM2SMEN_R = crate::BitReader<LPTIM2SMEN>;
impl LPTIM2SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM2SMEN {
        match self.bits {
            false => LPTIM2SMEN::Disabled,
            true => LPTIM2SMEN::Enabled,
        }
    }
    ///LPTIM2 clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPTIM2SMEN::Disabled
    }
    ///LPTIM2 clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPTIM2SMEN::Enabled
    }
}
///Field `LPTIM2SMEN` writer - LPTIM2SMEN
pub type LPTIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG, LPTIM2SMEN>;
impl<'a, REG> LPTIM2SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LPTIM2 clocks disabled by the clock gating during Sleep and Stop modes
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SMEN::Disabled)
    }
    ///LPTIM2 clocks enabled by the clock gating(1) during Sleep and Stop modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SMEN::Enabled)
    }
}
impl R {
    ///Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I2C4 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c4smen(&self) -> I2C4SMEN_R {
        I2C4SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - LPTIM2SMEN
    #[inline(always)]
    pub fn lptim2smen(&self) -> LPTIM2SMEN_R {
        LPTIM2SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1SMENR2")
            .field("lpuart1smen", &self.lpuart1smen())
            .field("i2c4smen", &self.i2c4smen())
            .field("lptim2smen", &self.lptim2smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W<'_, APB1SMENR2rs> {
        LPUART1SMEN_W::new(self, 0)
    }
    ///Bit 1 - I2C4 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c4smen(&mut self) -> I2C4SMEN_W<'_, APB1SMENR2rs> {
        I2C4SMEN_W::new(self, 1)
    }
    ///Bit 5 - LPTIM2SMEN
    #[inline(always)]
    pub fn lptim2smen(&mut self) -> LPTIM2SMEN_W<'_, APB1SMENR2rs> {
        LPTIM2SMEN_W::new(self, 5)
    }
}
/**APB1 peripheral clocks enable in Sleep and Stop modes register 2

You can [`read`](crate::Reg::read) this register and get [`apb1smenr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RCC:APB1SMENR2)*/
pub struct APB1SMENR2rs;
impl crate::RegisterSpec for APB1SMENR2rs {
    type Ux = u32;
}
///`read()` method returns [`apb1smenr2::R`](R) reader structure
impl crate::Readable for APB1SMENR2rs {}
///`write(|w| ..)` method takes [`apb1smenr2::W`](W) writer structure
impl crate::Writable for APB1SMENR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1SMENR2 to value 0x23
impl crate::Resettable for APB1SMENR2rs {
    const RESET_VALUE: u32 = 0x23;
}
