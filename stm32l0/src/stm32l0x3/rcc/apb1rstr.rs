///Register `APB1RSTR` reader
pub type R = crate::R<APB1RSTRrs>;
///Register `APB1RSTR` writer
pub type W = crate::W<APB1RSTRrs>;
/**Timer2 reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2RSTW {
    ///1: Reset the module
    Reset = 1,
}
impl From<TIM2RSTW> for bool {
    #[inline(always)]
    fn from(variant: TIM2RSTW) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2RST` reader - Timer2 reset
pub type TIM2RST_R = crate::BitReader<TIM2RSTW>;
impl TIM2RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TIM2RSTW> {
        match self.bits {
            true => Some(TIM2RSTW::Reset),
            _ => None,
        }
    }
    ///Reset the module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM2RSTW::Reset
    }
}
///Field `TIM2RST` writer - Timer2 reset
pub type TIM2RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM2RSTW>;
impl<'a, REG> TIM2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2RSTW::Reset)
    }
}
///Field `TIM3RST` reader - Timer3 reset
pub use TIM2RST_R as TIM3RST_R;
///Field `TIM6RST` reader - Timer 6 reset
pub use TIM2RST_R as TIM6RST_R;
///Field `TIM7RST` reader - Timer 7 reset
pub use TIM2RST_R as TIM7RST_R;
///Field `WWDRST` reader - Window watchdog reset
pub use TIM2RST_R as WWDRST_R;
///Field `SPI2RST` reader - SPI2 reset
pub use TIM2RST_R as SPI2RST_R;
///Field `LPUART12RST` reader - UART2 reset
pub use TIM2RST_R as LPUART12RST_R;
///Field `LPUART1RST` reader - LPUART1 reset
pub use TIM2RST_R as LPUART1RST_R;
///Field `USART4RST` reader - USART4 reset
pub use TIM2RST_R as USART4RST_R;
///Field `USART5RST` reader - USART5 reset
pub use TIM2RST_R as USART5RST_R;
///Field `I2C1RST` reader - I2C1 reset
pub use TIM2RST_R as I2C1RST_R;
///Field `I2C2RST` reader - I2C2 reset
pub use TIM2RST_R as I2C2RST_R;
///Field `USBRST` reader - USB reset
pub use TIM2RST_R as USBRST_R;
///Field `CRSRST` reader - Clock recovery system reset
pub use TIM2RST_R as CRSRST_R;
///Field `PWRRST` reader - Power interface reset
pub use TIM2RST_R as PWRRST_R;
///Field `DACRST` reader - DAC interface reset
pub use TIM2RST_R as DACRST_R;
///Field `I2C3RST` reader - I2C3 reset
pub use TIM2RST_R as I2C3RST_R;
///Field `LPTIM1RST` reader - Low power timer reset
pub use TIM2RST_R as LPTIM1RST_R;
///Field `TIM3RST` writer - Timer3 reset
pub use TIM2RST_W as TIM3RST_W;
///Field `TIM6RST` writer - Timer 6 reset
pub use TIM2RST_W as TIM6RST_W;
///Field `TIM7RST` writer - Timer 7 reset
pub use TIM2RST_W as TIM7RST_W;
///Field `WWDRST` writer - Window watchdog reset
pub use TIM2RST_W as WWDRST_W;
///Field `SPI2RST` writer - SPI2 reset
pub use TIM2RST_W as SPI2RST_W;
///Field `LPUART12RST` writer - UART2 reset
pub use TIM2RST_W as LPUART12RST_W;
///Field `LPUART1RST` writer - LPUART1 reset
pub use TIM2RST_W as LPUART1RST_W;
///Field `USART4RST` writer - USART4 reset
pub use TIM2RST_W as USART4RST_W;
///Field `USART5RST` writer - USART5 reset
pub use TIM2RST_W as USART5RST_W;
///Field `I2C1RST` writer - I2C1 reset
pub use TIM2RST_W as I2C1RST_W;
///Field `I2C2RST` writer - I2C2 reset
pub use TIM2RST_W as I2C2RST_W;
///Field `USBRST` writer - USB reset
pub use TIM2RST_W as USBRST_W;
///Field `CRSRST` writer - Clock recovery system reset
pub use TIM2RST_W as CRSRST_W;
///Field `PWRRST` writer - Power interface reset
pub use TIM2RST_W as PWRRST_W;
///Field `DACRST` writer - DAC interface reset
pub use TIM2RST_W as DACRST_W;
///Field `I2C3RST` writer - I2C3 reset
pub use TIM2RST_W as I2C3RST_W;
///Field `LPTIM1RST` writer - Low power timer reset
pub use TIM2RST_W as LPTIM1RST_W;
impl R {
    ///Bit 0 - Timer2 reset
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timer3 reset
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
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
    ///Bit 11 - Window watchdog reset
    #[inline(always)]
    pub fn wwdrst(&self) -> WWDRST_R {
        WWDRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2 reset
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - UART2 reset
    #[inline(always)]
    pub fn lpuart12rst(&self) -> LPUART12RST_R {
        LPUART12RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - LPUART1 reset
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - USART4 reset
    #[inline(always)]
    pub fn usart4rst(&self) -> USART4RST_R {
        USART4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - USART5 reset
    #[inline(always)]
    pub fn usart5rst(&self) -> USART5RST_R {
        USART5RST_R::new(((self.bits >> 20) & 1) != 0)
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
    ///Bit 27 - Clock recovery system reset
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Power interface reset
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC interface reset
    #[inline(always)]
    pub fn dacrst(&self) -> DACRST_R {
        DACRST_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - I2C3 reset
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low power timer reset
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1RSTR")
            .field("tim2rst", &self.tim2rst())
            .field("lptim1rst", &self.lptim1rst())
            .field("dacrst", &self.dacrst())
            .field("pwrrst", &self.pwrrst())
            .field("crsrst", &self.crsrst())
            .field("usbrst", &self.usbrst())
            .field("i2c2rst", &self.i2c2rst())
            .field("i2c1rst", &self.i2c1rst())
            .field("lpuart1rst", &self.lpuart1rst())
            .field("lpuart12rst", &self.lpuart12rst())
            .field("spi2rst", &self.spi2rst())
            .field("wwdrst", &self.wwdrst())
            .field("tim6rst", &self.tim6rst())
            .field("tim3rst", &self.tim3rst())
            .field("tim7rst", &self.tim7rst())
            .field("usart4rst", &self.usart4rst())
            .field("usart5rst", &self.usart5rst())
            .field("i2c3rst", &self.i2c3rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - Timer2 reset
    #[inline(always)]
    pub fn tim2rst(&mut self) -> TIM2RST_W<'_, APB1RSTRrs> {
        TIM2RST_W::new(self, 0)
    }
    ///Bit 1 - Timer3 reset
    #[inline(always)]
    pub fn tim3rst(&mut self) -> TIM3RST_W<'_, APB1RSTRrs> {
        TIM3RST_W::new(self, 1)
    }
    ///Bit 4 - Timer 6 reset
    #[inline(always)]
    pub fn tim6rst(&mut self) -> TIM6RST_W<'_, APB1RSTRrs> {
        TIM6RST_W::new(self, 4)
    }
    ///Bit 5 - Timer 7 reset
    #[inline(always)]
    pub fn tim7rst(&mut self) -> TIM7RST_W<'_, APB1RSTRrs> {
        TIM7RST_W::new(self, 5)
    }
    ///Bit 11 - Window watchdog reset
    #[inline(always)]
    pub fn wwdrst(&mut self) -> WWDRST_W<'_, APB1RSTRrs> {
        WWDRST_W::new(self, 11)
    }
    ///Bit 14 - SPI2 reset
    #[inline(always)]
    pub fn spi2rst(&mut self) -> SPI2RST_W<'_, APB1RSTRrs> {
        SPI2RST_W::new(self, 14)
    }
    ///Bit 17 - UART2 reset
    #[inline(always)]
    pub fn lpuart12rst(&mut self) -> LPUART12RST_W<'_, APB1RSTRrs> {
        LPUART12RST_W::new(self, 17)
    }
    ///Bit 18 - LPUART1 reset
    #[inline(always)]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<'_, APB1RSTRrs> {
        LPUART1RST_W::new(self, 18)
    }
    ///Bit 19 - USART4 reset
    #[inline(always)]
    pub fn usart4rst(&mut self) -> USART4RST_W<'_, APB1RSTRrs> {
        USART4RST_W::new(self, 19)
    }
    ///Bit 20 - USART5 reset
    #[inline(always)]
    pub fn usart5rst(&mut self) -> USART5RST_W<'_, APB1RSTRrs> {
        USART5RST_W::new(self, 20)
    }
    ///Bit 21 - I2C1 reset
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<'_, APB1RSTRrs> {
        I2C1RST_W::new(self, 21)
    }
    ///Bit 22 - I2C2 reset
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<'_, APB1RSTRrs> {
        I2C2RST_W::new(self, 22)
    }
    ///Bit 23 - USB reset
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W<'_, APB1RSTRrs> {
        USBRST_W::new(self, 23)
    }
    ///Bit 27 - Clock recovery system reset
    #[inline(always)]
    pub fn crsrst(&mut self) -> CRSRST_W<'_, APB1RSTRrs> {
        CRSRST_W::new(self, 27)
    }
    ///Bit 28 - Power interface reset
    #[inline(always)]
    pub fn pwrrst(&mut self) -> PWRRST_W<'_, APB1RSTRrs> {
        PWRRST_W::new(self, 28)
    }
    ///Bit 29 - DAC interface reset
    #[inline(always)]
    pub fn dacrst(&mut self) -> DACRST_W<'_, APB1RSTRrs> {
        DACRST_W::new(self, 29)
    }
    ///Bit 30 - I2C3 reset
    #[inline(always)]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<'_, APB1RSTRrs> {
        I2C3RST_W::new(self, 30)
    }
    ///Bit 31 - Low power timer reset
    #[inline(always)]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<'_, APB1RSTRrs> {
        LPTIM1RST_W::new(self, 31)
    }
}
/**APB1 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x3.html#RCC:APB1RSTR)*/
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
