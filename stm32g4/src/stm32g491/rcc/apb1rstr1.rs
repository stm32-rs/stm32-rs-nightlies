///Register `APB1RSTR1` reader
pub type R = crate::R<APB1RSTR1rs>;
///Register `APB1RSTR1` writer
pub type W = crate::W<APB1RSTR1rs>;
/**TIM2 timer reset

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
    ///Reset the selected module
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
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2RST::Reset)
    }
}
///Field `TIM3RST` reader - TIM3 timer reset
pub use TIM2RST_R as TIM3RST_R;
///Field `TIM4RST` reader - TIM3 timer reset
pub use TIM2RST_R as TIM4RST_R;
///Field `TIM5RST` reader - TIM5 timer reset
pub use TIM2RST_R as TIM5RST_R;
///Field `TIM6RST` reader - TIM6 timer reset
pub use TIM2RST_R as TIM6RST_R;
///Field `TIM7RST` reader - TIM7 timer reset
pub use TIM2RST_R as TIM7RST_R;
///Field `CRSRST` reader - CRS reset
pub use TIM2RST_R as CRSRST_R;
///Field `SPI2RST` reader - SPI2 reset
pub use TIM2RST_R as SPI2RST_R;
///Field `SPI3RST` reader - SPI3 reset
pub use TIM2RST_R as SPI3RST_R;
///Field `USART2RST` reader - USART2 reset
pub use TIM2RST_R as USART2RST_R;
///Field `USART3RST` reader - USART3 reset
pub use TIM2RST_R as USART3RST_R;
///Field `UART4RST` reader - UART4 reset
pub use TIM2RST_R as UART4RST_R;
///Field `UART5RST` reader - UART5 reset
pub use TIM2RST_R as UART5RST_R;
///Field `I2C1RST` reader - I2C1 reset
pub use TIM2RST_R as I2C1RST_R;
///Field `I2C2RST` reader - I2C2 reset
pub use TIM2RST_R as I2C2RST_R;
///Field `USBRST` reader - USB device reset
pub use TIM2RST_R as USBRST_R;
///Field `FDCANRST` reader - FDCAN reset
pub use TIM2RST_R as FDCANRST_R;
///Field `PWRRST` reader - Power interface reset
pub use TIM2RST_R as PWRRST_R;
///Field `I2C3RST` reader - I2C3 reset
pub use TIM2RST_R as I2C3RST_R;
///Field `LPTIM1RST` reader - Low Power Timer 1 reset
pub use TIM2RST_R as LPTIM1RST_R;
///Field `TIM3RST` writer - TIM3 timer reset
pub use TIM2RST_W as TIM3RST_W;
///Field `TIM4RST` writer - TIM3 timer reset
pub use TIM2RST_W as TIM4RST_W;
///Field `TIM5RST` writer - TIM5 timer reset
pub use TIM2RST_W as TIM5RST_W;
///Field `TIM6RST` writer - TIM6 timer reset
pub use TIM2RST_W as TIM6RST_W;
///Field `TIM7RST` writer - TIM7 timer reset
pub use TIM2RST_W as TIM7RST_W;
///Field `CRSRST` writer - CRS reset
pub use TIM2RST_W as CRSRST_W;
///Field `SPI2RST` writer - SPI2 reset
pub use TIM2RST_W as SPI2RST_W;
///Field `SPI3RST` writer - SPI3 reset
pub use TIM2RST_W as SPI3RST_W;
///Field `USART2RST` writer - USART2 reset
pub use TIM2RST_W as USART2RST_W;
///Field `USART3RST` writer - USART3 reset
pub use TIM2RST_W as USART3RST_W;
///Field `UART4RST` writer - UART4 reset
pub use TIM2RST_W as UART4RST_W;
///Field `UART5RST` writer - UART5 reset
pub use TIM2RST_W as UART5RST_W;
///Field `I2C1RST` writer - I2C1 reset
pub use TIM2RST_W as I2C1RST_W;
///Field `I2C2RST` writer - I2C2 reset
pub use TIM2RST_W as I2C2RST_W;
///Field `USBRST` writer - USB device reset
pub use TIM2RST_W as USBRST_W;
///Field `FDCANRST` writer - FDCAN reset
pub use TIM2RST_W as FDCANRST_W;
///Field `PWRRST` writer - Power interface reset
pub use TIM2RST_W as PWRRST_W;
///Field `I2C3RST` writer - I2C3 reset
pub use TIM2RST_W as I2C3RST_W;
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
    ///Bit 2 - TIM3 timer reset
    #[inline(always)]
    pub fn tim4rst(&self) -> TIM4RST_R {
        TIM4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5 timer reset
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6 timer reset
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 timer reset
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - CRS reset
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 8) & 1) != 0)
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
    ///Bit 17 - USART2 reset
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 reset
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART4 reset
    #[inline(always)]
    pub fn uart4rst(&self) -> UART4RST_R {
        UART4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - UART5 reset
    #[inline(always)]
    pub fn uart5rst(&self) -> UART5RST_R {
        UART5RST_R::new(((self.bits >> 20) & 1) != 0)
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
    ///Bit 23 - USB device reset
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - FDCAN reset
    #[inline(always)]
    pub fn fdcanrst(&self) -> FDCANRST_R {
        FDCANRST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 28 - Power interface reset
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - I2C3 reset
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low Power Timer 1 reset
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1RSTR1")
            .field("tim2rst", &self.tim2rst())
            .field("tim3rst", &self.tim3rst())
            .field("tim4rst", &self.tim4rst())
            .field("tim5rst", &self.tim5rst())
            .field("tim6rst", &self.tim6rst())
            .field("tim7rst", &self.tim7rst())
            .field("crsrst", &self.crsrst())
            .field("spi2rst", &self.spi2rst())
            .field("spi3rst", &self.spi3rst())
            .field("usart2rst", &self.usart2rst())
            .field("usart3rst", &self.usart3rst())
            .field("uart4rst", &self.uart4rst())
            .field("uart5rst", &self.uart5rst())
            .field("i2c1rst", &self.i2c1rst())
            .field("i2c2rst", &self.i2c2rst())
            .field("usbrst", &self.usbrst())
            .field("fdcanrst", &self.fdcanrst())
            .field("pwrrst", &self.pwrrst())
            .field("i2c3rst", &self.i2c3rst())
            .field("lptim1rst", &self.lptim1rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 timer reset
    #[inline(always)]
    pub fn tim2rst(&mut self) -> TIM2RST_W<'_, APB1RSTR1rs> {
        TIM2RST_W::new(self, 0)
    }
    ///Bit 1 - TIM3 timer reset
    #[inline(always)]
    pub fn tim3rst(&mut self) -> TIM3RST_W<'_, APB1RSTR1rs> {
        TIM3RST_W::new(self, 1)
    }
    ///Bit 2 - TIM3 timer reset
    #[inline(always)]
    pub fn tim4rst(&mut self) -> TIM4RST_W<'_, APB1RSTR1rs> {
        TIM4RST_W::new(self, 2)
    }
    ///Bit 3 - TIM5 timer reset
    #[inline(always)]
    pub fn tim5rst(&mut self) -> TIM5RST_W<'_, APB1RSTR1rs> {
        TIM5RST_W::new(self, 3)
    }
    ///Bit 4 - TIM6 timer reset
    #[inline(always)]
    pub fn tim6rst(&mut self) -> TIM6RST_W<'_, APB1RSTR1rs> {
        TIM6RST_W::new(self, 4)
    }
    ///Bit 5 - TIM7 timer reset
    #[inline(always)]
    pub fn tim7rst(&mut self) -> TIM7RST_W<'_, APB1RSTR1rs> {
        TIM7RST_W::new(self, 5)
    }
    ///Bit 8 - CRS reset
    #[inline(always)]
    pub fn crsrst(&mut self) -> CRSRST_W<'_, APB1RSTR1rs> {
        CRSRST_W::new(self, 8)
    }
    ///Bit 14 - SPI2 reset
    #[inline(always)]
    pub fn spi2rst(&mut self) -> SPI2RST_W<'_, APB1RSTR1rs> {
        SPI2RST_W::new(self, 14)
    }
    ///Bit 15 - SPI3 reset
    #[inline(always)]
    pub fn spi3rst(&mut self) -> SPI3RST_W<'_, APB1RSTR1rs> {
        SPI3RST_W::new(self, 15)
    }
    ///Bit 17 - USART2 reset
    #[inline(always)]
    pub fn usart2rst(&mut self) -> USART2RST_W<'_, APB1RSTR1rs> {
        USART2RST_W::new(self, 17)
    }
    ///Bit 18 - USART3 reset
    #[inline(always)]
    pub fn usart3rst(&mut self) -> USART3RST_W<'_, APB1RSTR1rs> {
        USART3RST_W::new(self, 18)
    }
    ///Bit 19 - UART4 reset
    #[inline(always)]
    pub fn uart4rst(&mut self) -> UART4RST_W<'_, APB1RSTR1rs> {
        UART4RST_W::new(self, 19)
    }
    ///Bit 20 - UART5 reset
    #[inline(always)]
    pub fn uart5rst(&mut self) -> UART5RST_W<'_, APB1RSTR1rs> {
        UART5RST_W::new(self, 20)
    }
    ///Bit 21 - I2C1 reset
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<'_, APB1RSTR1rs> {
        I2C1RST_W::new(self, 21)
    }
    ///Bit 22 - I2C2 reset
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<'_, APB1RSTR1rs> {
        I2C2RST_W::new(self, 22)
    }
    ///Bit 23 - USB device reset
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W<'_, APB1RSTR1rs> {
        USBRST_W::new(self, 23)
    }
    ///Bit 25 - FDCAN reset
    #[inline(always)]
    pub fn fdcanrst(&mut self) -> FDCANRST_W<'_, APB1RSTR1rs> {
        FDCANRST_W::new(self, 25)
    }
    ///Bit 28 - Power interface reset
    #[inline(always)]
    pub fn pwrrst(&mut self) -> PWRRST_W<'_, APB1RSTR1rs> {
        PWRRST_W::new(self, 28)
    }
    ///Bit 30 - I2C3 reset
    #[inline(always)]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<'_, APB1RSTR1rs> {
        I2C3RST_W::new(self, 30)
    }
    ///Bit 31 - Low Power Timer 1 reset
    #[inline(always)]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<'_, APB1RSTR1rs> {
        LPTIM1RST_W::new(self, 31)
    }
}
/**APB1 peripheral reset register 1

You can [`read`](crate::Reg::read) this register and get [`apb1rstr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G491.html#RCC:APB1RSTR1)*/
pub struct APB1RSTR1rs;
impl crate::RegisterSpec for APB1RSTR1rs {
    type Ux = u32;
}
///`read()` method returns [`apb1rstr1::R`](R) reader structure
impl crate::Readable for APB1RSTR1rs {}
///`write(|w| ..)` method takes [`apb1rstr1::W`](W) writer structure
impl crate::Writable for APB1RSTR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1RSTR1 to value 0
impl crate::Resettable for APB1RSTR1rs {}
