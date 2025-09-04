///Register `APB1RSTR` reader
pub type R = crate::R<APB1RSTRrs>;
///Register `APB1RSTR` writer
pub type W = crate::W<APB1RSTRrs>;
/**Timer 2 reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2RST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<TIM2RST> for bool {
    #[inline(always)]
    fn from(variant: TIM2RST) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2RST` reader - Timer 2 reset
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
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM2RST::Reset
    }
}
///Field `TIM2RST` writer - Timer 2 reset
pub type TIM2RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM2RST>;
impl<'a, REG> TIM2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2RST::Reset)
    }
}
///Field `TIM3RST` reader - Timer 3 reset
pub use TIM2RST_R as TIM3RST_R;
///Field `TIM4RST` reader - Timer 14 reset
pub use TIM2RST_R as TIM4RST_R;
///Field `TIM5RST` reader - Timer 5 reset
pub use TIM2RST_R as TIM5RST_R;
///Field `TIM6RST` reader - Timer 6 reset
pub use TIM2RST_R as TIM6RST_R;
///Field `TIM7RST` reader - Timer 7 reset
pub use TIM2RST_R as TIM7RST_R;
///Field `TIM12RST` reader - Timer 12 reset
pub use TIM2RST_R as TIM12RST_R;
///Field `TIM13RST` reader - Timer 13 reset
pub use TIM2RST_R as TIM13RST_R;
///Field `TIM14RST` reader - Timer 14 reset
pub use TIM2RST_R as TIM14RST_R;
///Field `TIM18RST` reader - Timer 18 reset
pub use TIM2RST_R as TIM18RST_R;
///Field `WWDGRST` reader - Window watchdog reset
pub use TIM2RST_R as WWDGRST_R;
///Field `SPI2RST` reader - SPI2 reset
pub use TIM2RST_R as SPI2RST_R;
///Field `SPI3RST` reader - SPI3 reset
pub use TIM2RST_R as SPI3RST_R;
///Field `USART2RST` reader - USART 2 reset
pub use TIM2RST_R as USART2RST_R;
///Field `USART3RST` reader - USART3 reset
pub use TIM2RST_R as USART3RST_R;
///Field `I2C1RST` reader - I2C1 reset
pub use TIM2RST_R as I2C1RST_R;
///Field `I2C2RST` reader - I2C2 reset
pub use TIM2RST_R as I2C2RST_R;
///Field `USBRST` reader - USB reset
pub use TIM2RST_R as USBRST_R;
///Field `CANRST` reader - CAN reset
pub use TIM2RST_R as CANRST_R;
///Field `DAC2RST` reader - DAC3 reset
pub use TIM2RST_R as DAC2RST_R;
///Field `PWRRST` reader - Power interface reset
pub use TIM2RST_R as PWRRST_R;
///Field `DAC1RST` reader - DAC interface reset
pub use TIM2RST_R as DAC1RST_R;
///Field `CECRST` reader - HDMI CEC reset
pub use TIM2RST_R as CECRST_R;
///Field `TIM3RST` writer - Timer 3 reset
pub use TIM2RST_W as TIM3RST_W;
///Field `TIM4RST` writer - Timer 14 reset
pub use TIM2RST_W as TIM4RST_W;
///Field `TIM5RST` writer - Timer 5 reset
pub use TIM2RST_W as TIM5RST_W;
///Field `TIM6RST` writer - Timer 6 reset
pub use TIM2RST_W as TIM6RST_W;
///Field `TIM7RST` writer - Timer 7 reset
pub use TIM2RST_W as TIM7RST_W;
///Field `TIM12RST` writer - Timer 12 reset
pub use TIM2RST_W as TIM12RST_W;
///Field `TIM13RST` writer - Timer 13 reset
pub use TIM2RST_W as TIM13RST_W;
///Field `TIM14RST` writer - Timer 14 reset
pub use TIM2RST_W as TIM14RST_W;
///Field `TIM18RST` writer - Timer 18 reset
pub use TIM2RST_W as TIM18RST_W;
///Field `WWDGRST` writer - Window watchdog reset
pub use TIM2RST_W as WWDGRST_W;
///Field `SPI2RST` writer - SPI2 reset
pub use TIM2RST_W as SPI2RST_W;
///Field `SPI3RST` writer - SPI3 reset
pub use TIM2RST_W as SPI3RST_W;
///Field `USART2RST` writer - USART 2 reset
pub use TIM2RST_W as USART2RST_W;
///Field `USART3RST` writer - USART3 reset
pub use TIM2RST_W as USART3RST_W;
///Field `I2C1RST` writer - I2C1 reset
pub use TIM2RST_W as I2C1RST_W;
///Field `I2C2RST` writer - I2C2 reset
pub use TIM2RST_W as I2C2RST_W;
///Field `USBRST` writer - USB reset
pub use TIM2RST_W as USBRST_W;
///Field `CANRST` writer - CAN reset
pub use TIM2RST_W as CANRST_W;
///Field `DAC2RST` writer - DAC3 reset
pub use TIM2RST_W as DAC2RST_W;
///Field `PWRRST` writer - Power interface reset
pub use TIM2RST_W as PWRRST_W;
///Field `DAC1RST` writer - DAC interface reset
pub use TIM2RST_W as DAC1RST_W;
///Field `CECRST` writer - HDMI CEC reset
pub use TIM2RST_W as CECRST_W;
impl R {
    ///Bit 0 - Timer 2 reset
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timer 3 reset
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer 14 reset
    #[inline(always)]
    pub fn tim4rst(&self) -> TIM4RST_R {
        TIM4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timer 5 reset
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timer 6 reset
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Timer 7 reset
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Timer 12 reset
    #[inline(always)]
    pub fn tim12rst(&self) -> TIM12RST_R {
        TIM12RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Timer 13 reset
    #[inline(always)]
    pub fn tim13rst(&self) -> TIM13RST_R {
        TIM13RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Timer 14 reset
    #[inline(always)]
    pub fn tim14rst(&self) -> TIM14RST_R {
        TIM14RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Timer 18 reset
    #[inline(always)]
    pub fn tim18rst(&self) -> TIM18RST_R {
        TIM18RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Window watchdog reset
    #[inline(always)]
    pub fn wwdgrst(&self) -> WWDGRST_R {
        WWDGRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2 reset
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI3 reset
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - USART 2 reset
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 reset
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
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
    ///Bit 23 - USB reset
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - CAN reset
    #[inline(always)]
    pub fn canrst(&self) -> CANRST_R {
        CANRST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - DAC3 reset
    #[inline(always)]
    pub fn dac2rst(&self) -> DAC2RST_R {
        DAC2RST_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - Power interface reset
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC interface reset
    #[inline(always)]
    pub fn dac1rst(&self) -> DAC1RST_R {
        DAC1RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - HDMI CEC reset
    #[inline(always)]
    pub fn cecrst(&self) -> CECRST_R {
        CECRST_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1RSTR")
            .field("tim2rst", &self.tim2rst())
            .field("tim3rst", &self.tim3rst())
            .field("tim4rst", &self.tim4rst())
            .field("tim5rst", &self.tim5rst())
            .field("tim6rst", &self.tim6rst())
            .field("tim7rst", &self.tim7rst())
            .field("tim12rst", &self.tim12rst())
            .field("tim13rst", &self.tim13rst())
            .field("tim14rst", &self.tim14rst())
            .field("tim18rst", &self.tim18rst())
            .field("wwdgrst", &self.wwdgrst())
            .field("spi2rst", &self.spi2rst())
            .field("spi3rst", &self.spi3rst())
            .field("usart2rst", &self.usart2rst())
            .field("usart3rst", &self.usart3rst())
            .field("i2c1rst", &self.i2c1rst())
            .field("i2c2rst", &self.i2c2rst())
            .field("usbrst", &self.usbrst())
            .field("canrst", &self.canrst())
            .field("dac2rst", &self.dac2rst())
            .field("pwrrst", &self.pwrrst())
            .field("dac1rst", &self.dac1rst())
            .field("cecrst", &self.cecrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - Timer 2 reset
    #[inline(always)]
    pub fn tim2rst(&mut self) -> TIM2RST_W<APB1RSTRrs> {
        TIM2RST_W::new(self, 0)
    }
    ///Bit 1 - Timer 3 reset
    #[inline(always)]
    pub fn tim3rst(&mut self) -> TIM3RST_W<APB1RSTRrs> {
        TIM3RST_W::new(self, 1)
    }
    ///Bit 2 - Timer 14 reset
    #[inline(always)]
    pub fn tim4rst(&mut self) -> TIM4RST_W<APB1RSTRrs> {
        TIM4RST_W::new(self, 2)
    }
    ///Bit 3 - Timer 5 reset
    #[inline(always)]
    pub fn tim5rst(&mut self) -> TIM5RST_W<APB1RSTRrs> {
        TIM5RST_W::new(self, 3)
    }
    ///Bit 4 - Timer 6 reset
    #[inline(always)]
    pub fn tim6rst(&mut self) -> TIM6RST_W<APB1RSTRrs> {
        TIM6RST_W::new(self, 4)
    }
    ///Bit 5 - Timer 7 reset
    #[inline(always)]
    pub fn tim7rst(&mut self) -> TIM7RST_W<APB1RSTRrs> {
        TIM7RST_W::new(self, 5)
    }
    ///Bit 6 - Timer 12 reset
    #[inline(always)]
    pub fn tim12rst(&mut self) -> TIM12RST_W<APB1RSTRrs> {
        TIM12RST_W::new(self, 6)
    }
    ///Bit 7 - Timer 13 reset
    #[inline(always)]
    pub fn tim13rst(&mut self) -> TIM13RST_W<APB1RSTRrs> {
        TIM13RST_W::new(self, 7)
    }
    ///Bit 8 - Timer 14 reset
    #[inline(always)]
    pub fn tim14rst(&mut self) -> TIM14RST_W<APB1RSTRrs> {
        TIM14RST_W::new(self, 8)
    }
    ///Bit 9 - Timer 18 reset
    #[inline(always)]
    pub fn tim18rst(&mut self) -> TIM18RST_W<APB1RSTRrs> {
        TIM18RST_W::new(self, 9)
    }
    ///Bit 11 - Window watchdog reset
    #[inline(always)]
    pub fn wwdgrst(&mut self) -> WWDGRST_W<APB1RSTRrs> {
        WWDGRST_W::new(self, 11)
    }
    ///Bit 14 - SPI2 reset
    #[inline(always)]
    pub fn spi2rst(&mut self) -> SPI2RST_W<APB1RSTRrs> {
        SPI2RST_W::new(self, 14)
    }
    ///Bit 15 - SPI3 reset
    #[inline(always)]
    pub fn spi3rst(&mut self) -> SPI3RST_W<APB1RSTRrs> {
        SPI3RST_W::new(self, 15)
    }
    ///Bit 17 - USART 2 reset
    #[inline(always)]
    pub fn usart2rst(&mut self) -> USART2RST_W<APB1RSTRrs> {
        USART2RST_W::new(self, 17)
    }
    ///Bit 18 - USART3 reset
    #[inline(always)]
    pub fn usart3rst(&mut self) -> USART3RST_W<APB1RSTRrs> {
        USART3RST_W::new(self, 18)
    }
    ///Bit 21 - I2C1 reset
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<APB1RSTRrs> {
        I2C1RST_W::new(self, 21)
    }
    ///Bit 22 - I2C2 reset
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<APB1RSTRrs> {
        I2C2RST_W::new(self, 22)
    }
    ///Bit 23 - USB reset
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W<APB1RSTRrs> {
        USBRST_W::new(self, 23)
    }
    ///Bit 25 - CAN reset
    #[inline(always)]
    pub fn canrst(&mut self) -> CANRST_W<APB1RSTRrs> {
        CANRST_W::new(self, 25)
    }
    ///Bit 26 - DAC3 reset
    #[inline(always)]
    pub fn dac2rst(&mut self) -> DAC2RST_W<APB1RSTRrs> {
        DAC2RST_W::new(self, 26)
    }
    ///Bit 28 - Power interface reset
    #[inline(always)]
    pub fn pwrrst(&mut self) -> PWRRST_W<APB1RSTRrs> {
        PWRRST_W::new(self, 28)
    }
    ///Bit 29 - DAC interface reset
    #[inline(always)]
    pub fn dac1rst(&mut self) -> DAC1RST_W<APB1RSTRrs> {
        DAC1RST_W::new(self, 29)
    }
    ///Bit 30 - HDMI CEC reset
    #[inline(always)]
    pub fn cecrst(&mut self) -> CECRST_W<APB1RSTRrs> {
        CECRST_W::new(self, 30)
    }
}
/**APB1 peripheral reset register (RCC_APB1RSTR)

You can [`read`](crate::Reg::read) this register and get [`apb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#RCC:APB1RSTR)*/
pub struct APB1RSTRrs;
impl crate::RegisterSpec for APB1RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1rstr::R`](R) reader structure
impl crate::Readable for APB1RSTRrs {}
///`write(|w| ..)` method takes [`apb1rstr::W`](W) writer structure
impl crate::Writable for APB1RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1RSTR to value 0
impl crate::Resettable for APB1RSTRrs {}
