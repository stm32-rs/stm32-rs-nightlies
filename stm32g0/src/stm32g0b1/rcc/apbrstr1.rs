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
///Field `TIM4RST` reader - TIM4 timer reset
pub use TIM2RST_R as TIM4RST_R;
///Field `TIM6RST` reader - TIM6 timer reset
pub use TIM2RST_R as TIM6RST_R;
///Field `TIM7RST` reader - TIM7 timer reset
pub use TIM2RST_R as TIM7RST_R;
///Field `LPUART2RST` reader - LPUART2RST
pub use TIM2RST_R as LPUART2RST_R;
///Field `USART5RST` reader - USART5RST
pub use TIM2RST_R as USART5RST_R;
///Field `USART6RST` reader - USART6RST
pub use TIM2RST_R as USART6RST_R;
///Field `FDCANRST` reader - FDCANRST
pub use TIM2RST_R as FDCANRST_R;
///Field `USBRST` reader - USBRST
pub use TIM2RST_R as USBRST_R;
///Field `SPI2RST` reader - SPI2 reset
pub use TIM2RST_R as SPI2RST_R;
///Field `SPI3RST` reader - SPI3 reset
pub use TIM2RST_R as SPI3RST_R;
///Field `CRSRST` reader - CRSRST
pub use TIM2RST_R as CRSRST_R;
///Field `USART2RST` reader - USART2 reset
pub use TIM2RST_R as USART2RST_R;
///Field `USART3RST` reader - USART3 reset
pub use TIM2RST_R as USART3RST_R;
///Field `USART4RST` reader - USART4 reset
pub use TIM2RST_R as USART4RST_R;
///Field `LPUART1RST` reader - LPUART1 reset
pub use TIM2RST_R as LPUART1RST_R;
///Field `I2C1RST` reader - I2C1 reset
pub use TIM2RST_R as I2C1RST_R;
///Field `I2C2RST` reader - I2C2 reset
pub use TIM2RST_R as I2C2RST_R;
///Field `I2C3RST` reader - I2C3RST reset
pub use TIM2RST_R as I2C3RST_R;
///Field `CECRST` reader - HDMI CEC reset
pub use TIM2RST_R as CECRST_R;
///Field `UCPD1RST` reader - UCPD1 reset
pub use TIM2RST_R as UCPD1RST_R;
///Field `UCPD2RST` reader - UCPD2 reset
pub use TIM2RST_R as UCPD2RST_R;
///Field `DBGRST` reader - Debug support reset
pub use TIM2RST_R as DBGRST_R;
///Field `PWRRST` reader - Power interface reset
pub use TIM2RST_R as PWRRST_R;
///Field `DAC1RST` reader - DAC1 interface reset
pub use TIM2RST_R as DAC1RST_R;
///Field `LPTIM2RST` reader - Low Power Timer 2 reset
pub use TIM2RST_R as LPTIM2RST_R;
///Field `LPTIM1RST` reader - Low Power Timer 1 reset
pub use TIM2RST_R as LPTIM1RST_R;
///Field `TIM3RST` writer - TIM3 timer reset
pub use TIM2RST_W as TIM3RST_W;
///Field `TIM4RST` writer - TIM4 timer reset
pub use TIM2RST_W as TIM4RST_W;
///Field `TIM6RST` writer - TIM6 timer reset
pub use TIM2RST_W as TIM6RST_W;
///Field `TIM7RST` writer - TIM7 timer reset
pub use TIM2RST_W as TIM7RST_W;
///Field `LPUART2RST` writer - LPUART2RST
pub use TIM2RST_W as LPUART2RST_W;
///Field `USART5RST` writer - USART5RST
pub use TIM2RST_W as USART5RST_W;
///Field `USART6RST` writer - USART6RST
pub use TIM2RST_W as USART6RST_W;
///Field `FDCANRST` writer - FDCANRST
pub use TIM2RST_W as FDCANRST_W;
///Field `USBRST` writer - USBRST
pub use TIM2RST_W as USBRST_W;
///Field `SPI2RST` writer - SPI2 reset
pub use TIM2RST_W as SPI2RST_W;
///Field `SPI3RST` writer - SPI3 reset
pub use TIM2RST_W as SPI3RST_W;
///Field `CRSRST` writer - CRSRST
pub use TIM2RST_W as CRSRST_W;
///Field `USART2RST` writer - USART2 reset
pub use TIM2RST_W as USART2RST_W;
///Field `USART3RST` writer - USART3 reset
pub use TIM2RST_W as USART3RST_W;
///Field `USART4RST` writer - USART4 reset
pub use TIM2RST_W as USART4RST_W;
///Field `LPUART1RST` writer - LPUART1 reset
pub use TIM2RST_W as LPUART1RST_W;
///Field `I2C1RST` writer - I2C1 reset
pub use TIM2RST_W as I2C1RST_W;
///Field `I2C2RST` writer - I2C2 reset
pub use TIM2RST_W as I2C2RST_W;
///Field `I2C3RST` writer - I2C3RST reset
pub use TIM2RST_W as I2C3RST_W;
///Field `CECRST` writer - HDMI CEC reset
pub use TIM2RST_W as CECRST_W;
///Field `UCPD1RST` writer - UCPD1 reset
pub use TIM2RST_W as UCPD1RST_W;
///Field `UCPD2RST` writer - UCPD2 reset
pub use TIM2RST_W as UCPD2RST_W;
///Field `DBGRST` writer - Debug support reset
pub use TIM2RST_W as DBGRST_W;
///Field `PWRRST` writer - Power interface reset
pub use TIM2RST_W as PWRRST_W;
///Field `DAC1RST` writer - DAC1 interface reset
pub use TIM2RST_W as DAC1RST_W;
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
    ///Bit 2 - TIM4 timer reset
    #[inline(always)]
    pub fn tim4rst(&self) -> TIM4RST_R {
        TIM4RST_R::new(((self.bits >> 2) & 1) != 0)
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
    ///Bit 7 - LPUART2RST
    #[inline(always)]
    pub fn lpuart2rst(&self) -> LPUART2RST_R {
        LPUART2RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - USART5RST
    #[inline(always)]
    pub fn usart5rst(&self) -> USART5RST_R {
        USART5RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - USART6RST
    #[inline(always)]
    pub fn usart6rst(&self) -> USART6RST_R {
        USART6RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - FDCANRST
    #[inline(always)]
    pub fn fdcanrst(&self) -> FDCANRST_R {
        FDCANRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - USBRST
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 13) & 1) != 0)
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
    ///Bit 16 - CRSRST
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 16) & 1) != 0)
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
    ///Bit 19 - USART4 reset
    #[inline(always)]
    pub fn usart4rst(&self) -> USART4RST_R {
        USART4RST_R::new(((self.bits >> 19) & 1) != 0)
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
    ///Bit 23 - I2C3RST reset
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - HDMI CEC reset
    #[inline(always)]
    pub fn cecrst(&self) -> CECRST_R {
        CECRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - UCPD1 reset
    #[inline(always)]
    pub fn ucpd1rst(&self) -> UCPD1RST_R {
        UCPD1RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - UCPD2 reset
    #[inline(always)]
    pub fn ucpd2rst(&self) -> UCPD2RST_R {
        UCPD2RST_R::new(((self.bits >> 26) & 1) != 0)
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
    ///Bit 29 - DAC1 interface reset
    #[inline(always)]
    pub fn dac1rst(&self) -> DAC1RST_R {
        DAC1RST_R::new(((self.bits >> 29) & 1) != 0)
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
            .field("tim4rst", &self.tim4rst())
            .field("tim6rst", &self.tim6rst())
            .field("tim7rst", &self.tim7rst())
            .field("lpuart2rst", &self.lpuart2rst())
            .field("usart5rst", &self.usart5rst())
            .field("usart6rst", &self.usart6rst())
            .field("fdcanrst", &self.fdcanrst())
            .field("usbrst", &self.usbrst())
            .field("spi2rst", &self.spi2rst())
            .field("spi3rst", &self.spi3rst())
            .field("crsrst", &self.crsrst())
            .field("usart2rst", &self.usart2rst())
            .field("usart3rst", &self.usart3rst())
            .field("usart4rst", &self.usart4rst())
            .field("lpuart1rst", &self.lpuart1rst())
            .field("i2c1rst", &self.i2c1rst())
            .field("i2c2rst", &self.i2c2rst())
            .field("i2c3rst", &self.i2c3rst())
            .field("cecrst", &self.cecrst())
            .field("ucpd1rst", &self.ucpd1rst())
            .field("ucpd2rst", &self.ucpd2rst())
            .field("dbgrst", &self.dbgrst())
            .field("pwrrst", &self.pwrrst())
            .field("dac1rst", &self.dac1rst())
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
    ///Bit 2 - TIM4 timer reset
    #[inline(always)]
    pub fn tim4rst(&mut self) -> TIM4RST_W<'_, APBRSTR1rs> {
        TIM4RST_W::new(self, 2)
    }
    ///Bit 4 - TIM6 timer reset
    #[inline(always)]
    pub fn tim6rst(&mut self) -> TIM6RST_W<'_, APBRSTR1rs> {
        TIM6RST_W::new(self, 4)
    }
    ///Bit 5 - TIM7 timer reset
    #[inline(always)]
    pub fn tim7rst(&mut self) -> TIM7RST_W<'_, APBRSTR1rs> {
        TIM7RST_W::new(self, 5)
    }
    ///Bit 7 - LPUART2RST
    #[inline(always)]
    pub fn lpuart2rst(&mut self) -> LPUART2RST_W<'_, APBRSTR1rs> {
        LPUART2RST_W::new(self, 7)
    }
    ///Bit 8 - USART5RST
    #[inline(always)]
    pub fn usart5rst(&mut self) -> USART5RST_W<'_, APBRSTR1rs> {
        USART5RST_W::new(self, 8)
    }
    ///Bit 9 - USART6RST
    #[inline(always)]
    pub fn usart6rst(&mut self) -> USART6RST_W<'_, APBRSTR1rs> {
        USART6RST_W::new(self, 9)
    }
    ///Bit 12 - FDCANRST
    #[inline(always)]
    pub fn fdcanrst(&mut self) -> FDCANRST_W<'_, APBRSTR1rs> {
        FDCANRST_W::new(self, 12)
    }
    ///Bit 13 - USBRST
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W<'_, APBRSTR1rs> {
        USBRST_W::new(self, 13)
    }
    ///Bit 14 - SPI2 reset
    #[inline(always)]
    pub fn spi2rst(&mut self) -> SPI2RST_W<'_, APBRSTR1rs> {
        SPI2RST_W::new(self, 14)
    }
    ///Bit 15 - SPI3 reset
    #[inline(always)]
    pub fn spi3rst(&mut self) -> SPI3RST_W<'_, APBRSTR1rs> {
        SPI3RST_W::new(self, 15)
    }
    ///Bit 16 - CRSRST
    #[inline(always)]
    pub fn crsrst(&mut self) -> CRSRST_W<'_, APBRSTR1rs> {
        CRSRST_W::new(self, 16)
    }
    ///Bit 17 - USART2 reset
    #[inline(always)]
    pub fn usart2rst(&mut self) -> USART2RST_W<'_, APBRSTR1rs> {
        USART2RST_W::new(self, 17)
    }
    ///Bit 18 - USART3 reset
    #[inline(always)]
    pub fn usart3rst(&mut self) -> USART3RST_W<'_, APBRSTR1rs> {
        USART3RST_W::new(self, 18)
    }
    ///Bit 19 - USART4 reset
    #[inline(always)]
    pub fn usart4rst(&mut self) -> USART4RST_W<'_, APBRSTR1rs> {
        USART4RST_W::new(self, 19)
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
    ///Bit 23 - I2C3RST reset
    #[inline(always)]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<'_, APBRSTR1rs> {
        I2C3RST_W::new(self, 23)
    }
    ///Bit 24 - HDMI CEC reset
    #[inline(always)]
    pub fn cecrst(&mut self) -> CECRST_W<'_, APBRSTR1rs> {
        CECRST_W::new(self, 24)
    }
    ///Bit 25 - UCPD1 reset
    #[inline(always)]
    pub fn ucpd1rst(&mut self) -> UCPD1RST_W<'_, APBRSTR1rs> {
        UCPD1RST_W::new(self, 25)
    }
    ///Bit 26 - UCPD2 reset
    #[inline(always)]
    pub fn ucpd2rst(&mut self) -> UCPD2RST_W<'_, APBRSTR1rs> {
        UCPD2RST_W::new(self, 26)
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
    ///Bit 29 - DAC1 interface reset
    #[inline(always)]
    pub fn dac1rst(&mut self) -> DAC1RST_W<'_, APBRSTR1rs> {
        DAC1RST_W::new(self, 29)
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B1.html#RCC:APBRSTR1)*/
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
