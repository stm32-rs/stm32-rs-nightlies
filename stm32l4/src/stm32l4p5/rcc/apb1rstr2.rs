///Register `APB1RSTR2` reader
pub type R = crate::R<APB1RSTR2rs>;
///Register `APB1RSTR2` writer
pub type W = crate::W<APB1RSTR2rs>;
/**Low-power UART 1 reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1RST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset LPUART1
    Reset = 1,
}
impl From<LPUART1RST> for bool {
    #[inline(always)]
    fn from(variant: LPUART1RST) -> Self {
        variant as u8 != 0
    }
}
///Field `LPUART1RST` reader - Low-power UART 1 reset
pub type LPUART1RST_R = crate::BitReader<LPUART1RST>;
impl LPUART1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPUART1RST {
        match self.bits {
            false => LPUART1RST::NoEffect,
            true => LPUART1RST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == LPUART1RST::NoEffect
    }
    ///Reset LPUART1
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LPUART1RST::Reset
    }
}
///Field `LPUART1RST` writer - Low-power UART 1 reset
pub type LPUART1RST_W<'a, REG> = crate::BitWriter<'a, REG, LPUART1RST>;
impl<'a, REG> LPUART1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1RST::NoEffect)
    }
    ///Reset LPUART1
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1RST::Reset)
    }
}
/**I2C4 reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C4RST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset I2C4
    Reset = 1,
}
impl From<I2C4RST> for bool {
    #[inline(always)]
    fn from(variant: I2C4RST) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C4RST` reader - I2C4 reset
pub type I2C4RST_R = crate::BitReader<I2C4RST>;
impl I2C4RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C4RST {
        match self.bits {
            false => I2C4RST::NoEffect,
            true => I2C4RST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == I2C4RST::NoEffect
    }
    ///Reset I2C4
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == I2C4RST::Reset
    }
}
///Field `I2C4RST` writer - I2C4 reset
pub type I2C4RST_W<'a, REG> = crate::BitWriter<'a, REG, I2C4RST>;
impl<'a, REG> I2C4RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(I2C4RST::NoEffect)
    }
    ///Reset I2C4
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(I2C4RST::Reset)
    }
}
/**Low-power timer 2 reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM2RST {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset LPTIM2
    Reset = 1,
}
impl From<LPTIM2RST> for bool {
    #[inline(always)]
    fn from(variant: LPTIM2RST) -> Self {
        variant as u8 != 0
    }
}
///Field `LPTIM2RST` reader - Low-power timer 2 reset
pub type LPTIM2RST_R = crate::BitReader<LPTIM2RST>;
impl LPTIM2RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM2RST {
        match self.bits {
            false => LPTIM2RST::NoEffect,
            true => LPTIM2RST::Reset,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == LPTIM2RST::NoEffect
    }
    ///Reset LPTIM2
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LPTIM2RST::Reset
    }
}
///Field `LPTIM2RST` writer - Low-power timer 2 reset
pub type LPTIM2RST_W<'a, REG> = crate::BitWriter<'a, REG, LPTIM2RST>;
impl<'a, REG> LPTIM2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2RST::NoEffect)
    }
    ///Reset LPTIM2
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2RST::Reset)
    }
}
impl R {
    ///Bit 0 - Low-power UART 1 reset
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I2C4 reset
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - Low-power timer 2 reset
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1RSTR2")
            .field("lpuart1rst", &self.lpuart1rst())
            .field("i2c4rst", &self.i2c4rst())
            .field("lptim2rst", &self.lptim2rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - Low-power UART 1 reset
    #[inline(always)]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<'_, APB1RSTR2rs> {
        LPUART1RST_W::new(self, 0)
    }
    ///Bit 1 - I2C4 reset
    #[inline(always)]
    pub fn i2c4rst(&mut self) -> I2C4RST_W<'_, APB1RSTR2rs> {
        I2C4RST_W::new(self, 1)
    }
    ///Bit 5 - Low-power timer 2 reset
    #[inline(always)]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W<'_, APB1RSTR2rs> {
        LPTIM2RST_W::new(self, 5)
    }
}
/**APB1 peripheral reset register 2

You can [`read`](crate::Reg::read) this register and get [`apb1rstr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#RCC:APB1RSTR2)*/
pub struct APB1RSTR2rs;
impl crate::RegisterSpec for APB1RSTR2rs {
    type Ux = u32;
}
///`read()` method returns [`apb1rstr2::R`](R) reader structure
impl crate::Readable for APB1RSTR2rs {}
///`write(|w| ..)` method takes [`apb1rstr2::W`](W) writer structure
impl crate::Writable for APB1RSTR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1RSTR2 to value 0
impl crate::Resettable for APB1RSTR2rs {}
