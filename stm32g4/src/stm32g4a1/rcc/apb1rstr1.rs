#[doc = "Register `APB1RSTR1` reader"]
pub type R = crate::R<APB1RSTR1rs>;
#[doc = "Register `APB1RSTR1` writer"]
pub type W = crate::W<APB1RSTR1rs>;
#[doc = "TIM2 timer reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2RST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<TIM2RST> for bool {
    #[inline(always)]
    fn from(variant: TIM2RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2RST` reader - TIM2 timer reset"]
pub type TIM2RST_R = crate::BitReader<TIM2RST>;
impl TIM2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TIM2RST> {
        match self.bits {
            true => Some(TIM2RST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM2RST::Reset
    }
}
#[doc = "Field `TIM2RST` writer - TIM2 timer reset"]
pub type TIM2RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM2RST>;
impl<'a, REG> TIM2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2RST::Reset)
    }
}
#[doc = "Field `TIM3RST` reader - TIM3 timer reset"]
pub use TIM2RST_R as TIM3RST_R;
#[doc = "Field `TIM4RST` reader - TIM3 timer reset"]
pub use TIM2RST_R as TIM4RST_R;
#[doc = "Field `TIM5RST` reader - TIM5 timer reset"]
pub use TIM2RST_R as TIM5RST_R;
#[doc = "Field `TIM6RST` reader - TIM6 timer reset"]
pub use TIM2RST_R as TIM6RST_R;
#[doc = "Field `TIM7RST` reader - TIM7 timer reset"]
pub use TIM2RST_R as TIM7RST_R;
#[doc = "Field `CRSRST` reader - Clock recovery system reset"]
pub use TIM2RST_R as CRSRST_R;
#[doc = "Field `SPI2RST` reader - SPI2 reset"]
pub use TIM2RST_R as SPI2RST_R;
#[doc = "Field `SPI3RST` reader - SPI3 reset"]
pub use TIM2RST_R as SPI3RST_R;
#[doc = "Field `USART2RST` reader - USART2 reset"]
pub use TIM2RST_R as USART2RST_R;
#[doc = "Field `USART3RST` reader - USART3 reset"]
pub use TIM2RST_R as USART3RST_R;
#[doc = "Field `UART4RST` reader - UART4 reset"]
pub use TIM2RST_R as UART4RST_R;
#[doc = "Field `UART5RST` reader - UART5 reset"]
pub use TIM2RST_R as UART5RST_R;
#[doc = "Field `I2C1RST` reader - I2C1 reset"]
pub use TIM2RST_R as I2C1RST_R;
#[doc = "Field `I2C2RST` reader - I2C2 reset"]
pub use TIM2RST_R as I2C2RST_R;
#[doc = "Field `USBRST` reader - USBD reset"]
pub use TIM2RST_R as USBRST_R;
#[doc = "Field `FDCANRST` reader - FDCAN reset"]
pub use TIM2RST_R as FDCANRST_R;
#[doc = "Field `PWRRST` reader - Power interface reset"]
pub use TIM2RST_R as PWRRST_R;
#[doc = "Field `I2C3RST` reader - I2C3 interface reset"]
pub use TIM2RST_R as I2C3RST_R;
#[doc = "Field `LPTIM1RST` reader - Low Power Timer 1 reset"]
pub use TIM2RST_R as LPTIM1RST_R;
#[doc = "Field `TIM3RST` writer - TIM3 timer reset"]
pub use TIM2RST_W as TIM3RST_W;
#[doc = "Field `TIM4RST` writer - TIM3 timer reset"]
pub use TIM2RST_W as TIM4RST_W;
#[doc = "Field `TIM5RST` writer - TIM5 timer reset"]
pub use TIM2RST_W as TIM5RST_W;
#[doc = "Field `TIM6RST` writer - TIM6 timer reset"]
pub use TIM2RST_W as TIM6RST_W;
#[doc = "Field `TIM7RST` writer - TIM7 timer reset"]
pub use TIM2RST_W as TIM7RST_W;
#[doc = "Field `CRSRST` writer - Clock recovery system reset"]
pub use TIM2RST_W as CRSRST_W;
#[doc = "Field `SPI2RST` writer - SPI2 reset"]
pub use TIM2RST_W as SPI2RST_W;
#[doc = "Field `SPI3RST` writer - SPI3 reset"]
pub use TIM2RST_W as SPI3RST_W;
#[doc = "Field `USART2RST` writer - USART2 reset"]
pub use TIM2RST_W as USART2RST_W;
#[doc = "Field `USART3RST` writer - USART3 reset"]
pub use TIM2RST_W as USART3RST_W;
#[doc = "Field `UART4RST` writer - UART4 reset"]
pub use TIM2RST_W as UART4RST_W;
#[doc = "Field `UART5RST` writer - UART5 reset"]
pub use TIM2RST_W as UART5RST_W;
#[doc = "Field `I2C1RST` writer - I2C1 reset"]
pub use TIM2RST_W as I2C1RST_W;
#[doc = "Field `I2C2RST` writer - I2C2 reset"]
pub use TIM2RST_W as I2C2RST_W;
#[doc = "Field `USBRST` writer - USBD reset"]
pub use TIM2RST_W as USBRST_W;
#[doc = "Field `FDCANRST` writer - FDCAN reset"]
pub use TIM2RST_W as FDCANRST_W;
#[doc = "Field `PWRRST` writer - Power interface reset"]
pub use TIM2RST_W as PWRRST_W;
#[doc = "Field `I2C3RST` writer - I2C3 interface reset"]
pub use TIM2RST_W as I2C3RST_W;
#[doc = "Field `LPTIM1RST` writer - Low Power Timer 1 reset"]
pub use TIM2RST_W as LPTIM1RST_W;
impl R {
    #[doc = "Bit 0 - TIM2 timer reset"]
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer reset"]
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM3 timer reset"]
    #[inline(always)]
    pub fn tim4rst(&self) -> TIM4RST_R {
        TIM4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 timer reset"]
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer reset"]
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer reset"]
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Clock recovery system reset"]
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 reset"]
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 reset"]
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 reset"]
    #[inline(always)]
    pub fn uart4rst(&self) -> UART4RST_R {
        UART4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 reset"]
    #[inline(always)]
    pub fn uart5rst(&self) -> UART5RST_R {
        UART5RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USBD reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - FDCAN reset"]
    #[inline(always)]
    pub fn fdcanrst(&self) -> FDCANRST_R {
        FDCANRST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - I2C3 interface reset"]
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low Power Timer 1 reset"]
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim2rst(&mut self) -> TIM2RST_W<APB1RSTR1rs> {
        TIM2RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim3rst(&mut self) -> TIM3RST_W<APB1RSTR1rs> {
        TIM3RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM3 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim4rst(&mut self) -> TIM4RST_W<APB1RSTR1rs> {
        TIM4RST_W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim5rst(&mut self) -> TIM5RST_W<APB1RSTR1rs> {
        TIM5RST_W::new(self, 3)
    }
    #[doc = "Bit 4 - TIM6 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim6rst(&mut self) -> TIM6RST_W<APB1RSTR1rs> {
        TIM6RST_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 timer reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim7rst(&mut self) -> TIM7RST_W<APB1RSTR1rs> {
        TIM7RST_W::new(self, 5)
    }
    #[doc = "Bit 8 - Clock recovery system reset"]
    #[inline(always)]
    #[must_use]
    pub fn crsrst(&mut self) -> CRSRST_W<APB1RSTR1rs> {
        CRSRST_W::new(self, 8)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi2rst(&mut self) -> SPI2RST_W<APB1RSTR1rs> {
        SPI2RST_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi3rst(&mut self) -> SPI3RST_W<APB1RSTR1rs> {
        SPI3RST_W::new(self, 15)
    }
    #[doc = "Bit 17 - USART2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart2rst(&mut self) -> USART2RST_W<APB1RSTR1rs> {
        USART2RST_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart3rst(&mut self) -> USART3RST_W<APB1RSTR1rs> {
        USART3RST_W::new(self, 18)
    }
    #[doc = "Bit 19 - UART4 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart4rst(&mut self) -> UART4RST_W<APB1RSTR1rs> {
        UART4RST_W::new(self, 19)
    }
    #[doc = "Bit 20 - UART5 reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart5rst(&mut self) -> UART5RST_W<APB1RSTR1rs> {
        UART5RST_W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<APB1RSTR1rs> {
        I2C1RST_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<APB1RSTR1rs> {
        I2C2RST_W::new(self, 22)
    }
    #[doc = "Bit 23 - USBD reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> USBRST_W<APB1RSTR1rs> {
        USBRST_W::new(self, 23)
    }
    #[doc = "Bit 25 - FDCAN reset"]
    #[inline(always)]
    #[must_use]
    pub fn fdcanrst(&mut self) -> FDCANRST_W<APB1RSTR1rs> {
        FDCANRST_W::new(self, 25)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn pwrrst(&mut self) -> PWRRST_W<APB1RSTR1rs> {
        PWRRST_W::new(self, 28)
    }
    #[doc = "Bit 30 - I2C3 interface reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<APB1RSTR1rs> {
        I2C3RST_W::new(self, 30)
    }
    #[doc = "Bit 31 - Low Power Timer 1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<APB1RSTR1rs> {
        LPTIM1RST_W::new(self, 31)
    }
}
#[doc = "APB1 peripheral reset register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rstr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rstr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1RSTR1rs;
impl crate::RegisterSpec for APB1RSTR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1rstr1::R`](R) reader structure"]
impl crate::Readable for APB1RSTR1rs {}
#[doc = "`write(|w| ..)` method takes [`apb1rstr1::W`](W) writer structure"]
impl crate::Writable for APB1RSTR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1RSTR1 to value 0"]
impl crate::Resettable for APB1RSTR1rs {
    const RESET_VALUE: u32 = 0;
}
