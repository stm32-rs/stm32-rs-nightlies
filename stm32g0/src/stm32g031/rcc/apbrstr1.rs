///Register `APBRSTR1` reader
pub type R = crate::R<APBRSTR1rs>;
///Register `APBRSTR1` writer
pub type W = crate::W<APBRSTR1rs>;
/**TIM2 timer reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2RST {
    ///1: Reset peripheral
    Reset = 1,
}
impl From<TIM2RST> for bool {
    #[inline(always)]
    fn from(variant: TIM2RST) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2RST` reader - TIM2 timer reset
pub type TIM2RST_R = crate::BitReader<TIM2RST>;
impl TIM2RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TIM2RST> {
        match self.bits {
            true => Some(TIM2RST::Reset),
            _ => None,
        }
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM2RST::Reset
    }
}
///Field `TIM2RST` writer - TIM2 timer reset
pub type TIM2RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM2RST>;
impl<'a, REG> TIM2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2RST::Reset)
    }
}
///Field `TIM3RST` reader - TIM3 timer reset
pub use TIM2RST_R as TIM3RST_R;
///Field `SPI2RST` reader - SPI2 reset
pub use TIM2RST_R as SPI2RST_R;
///Field `USART2RST` reader - USART2 reset
pub use TIM2RST_R as USART2RST_R;
///Field `LPUART1RST` reader - LPUART1 reset
pub use TIM2RST_R as LPUART1RST_R;
///Field `I2C1RST` reader - I2C1 reset
pub use TIM2RST_R as I2C1RST_R;
///Field `I2C2RST` reader - I2C2 reset
pub use TIM2RST_R as I2C2RST_R;
///Field `DBGRST` reader - Debug support reset
pub use TIM2RST_R as DBGRST_R;
///Field `PWRRST` reader - Power interface reset
pub use TIM2RST_R as PWRRST_R;
///Field `LPTIM2RST` reader - Low Power Timer 2 reset
pub use TIM2RST_R as LPTIM2RST_R;
///Field `LPTIM1RST` reader - Low Power Timer 1 reset
pub use TIM2RST_R as LPTIM1RST_R;
///Field `TIM3RST` writer - TIM3 timer reset
pub use TIM2RST_W as TIM3RST_W;
///Field `SPI2RST` writer - SPI2 reset
pub use TIM2RST_W as SPI2RST_W;
///Field `USART2RST` writer - USART2 reset
pub use TIM2RST_W as USART2RST_W;
///Field `LPUART1RST` writer - LPUART1 reset
pub use TIM2RST_W as LPUART1RST_W;
///Field `I2C1RST` writer - I2C1 reset
pub use TIM2RST_W as I2C1RST_W;
///Field `I2C2RST` writer - I2C2 reset
pub use TIM2RST_W as I2C2RST_W;
///Field `DBGRST` writer - Debug support reset
pub use TIM2RST_W as DBGRST_W;
///Field `PWRRST` writer - Power interface reset
pub use TIM2RST_W as PWRRST_W;
///Field `LPTIM2RST` writer - Low Power Timer 2 reset
pub use TIM2RST_W as LPTIM2RST_W;
///Field `LPTIM1RST` writer - Low Power Timer 1 reset
pub use TIM2RST_W as LPTIM1RST_W;
impl R {
    ///Bit 0 - TIM2 timer reset
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 timer reset
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 14 - SPI2 reset
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - USART2 reset
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - LPUART1 reset
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 reset
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 reset
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 27 - Debug support reset
    #[inline(always)]
    pub fn dbgrst(&self) -> DBGRST_R {
        DBGRST_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Power interface reset
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - Low Power Timer 2 reset
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low Power Timer 1 reset
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APBRSTR1")
            .field("tim2rst", &self.tim2rst())
            .field("tim3rst", &self.tim3rst())
            .field("spi2rst", &self.spi2rst())
            .field("usart2rst", &self.usart2rst())
            .field("lpuart1rst", &self.lpuart1rst())
            .field("i2c1rst", &self.i2c1rst())
            .field("i2c2rst", &self.i2c2rst())
            .field("dbgrst", &self.dbgrst())
            .field("pwrrst", &self.pwrrst())
            .field("lptim2rst", &self.lptim2rst())
            .field("lptim1rst", &self.lptim1rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 timer reset
    #[inline(always)]
    pub fn tim2rst(&mut self) -> TIM2RST_W<'_, APBRSTR1rs> {
        TIM2RST_W::new(self, 0)
    }
    ///Bit 1 - TIM3 timer reset
    #[inline(always)]
    pub fn tim3rst(&mut self) -> TIM3RST_W<'_, APBRSTR1rs> {
        TIM3RST_W::new(self, 1)
    }
    ///Bit 14 - SPI2 reset
    #[inline(always)]
    pub fn spi2rst(&mut self) -> SPI2RST_W<'_, APBRSTR1rs> {
        SPI2RST_W::new(self, 14)
    }
    ///Bit 17 - USART2 reset
    #[inline(always)]
    pub fn usart2rst(&mut self) -> USART2RST_W<'_, APBRSTR1rs> {
        USART2RST_W::new(self, 17)
    }
    ///Bit 20 - LPUART1 reset
    #[inline(always)]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<'_, APBRSTR1rs> {
        LPUART1RST_W::new(self, 20)
    }
    ///Bit 21 - I2C1 reset
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<'_, APBRSTR1rs> {
        I2C1RST_W::new(self, 21)
    }
    ///Bit 22 - I2C2 reset
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<'_, APBRSTR1rs> {
        I2C2RST_W::new(self, 22)
    }
    ///Bit 27 - Debug support reset
    #[inline(always)]
    pub fn dbgrst(&mut self) -> DBGRST_W<'_, APBRSTR1rs> {
        DBGRST_W::new(self, 27)
    }
    ///Bit 28 - Power interface reset
    #[inline(always)]
    pub fn pwrrst(&mut self) -> PWRRST_W<'_, APBRSTR1rs> {
        PWRRST_W::new(self, 28)
    }
    ///Bit 30 - Low Power Timer 2 reset
    #[inline(always)]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W<'_, APBRSTR1rs> {
        LPTIM2RST_W::new(self, 30)
    }
    ///Bit 31 - Low Power Timer 1 reset
    #[inline(always)]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<'_, APBRSTR1rs> {
        LPTIM1RST_W::new(self, 31)
    }
}
/**APB peripheral reset register 1

You can [`read`](crate::Reg::read) this register and get [`apbrstr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrstr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#RCC:APBRSTR1)*/
pub struct APBRSTR1rs;
impl crate::RegisterSpec for APBRSTR1rs {
    type Ux = u32;
}
///`read()` method returns [`apbrstr1::R`](R) reader structure
impl crate::Readable for APBRSTR1rs {}
///`write(|w| ..)` method takes [`apbrstr1::W`](W) writer structure
impl crate::Writable for APBRSTR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APBRSTR1 to value 0
impl crate::Resettable for APBRSTR1rs {}
