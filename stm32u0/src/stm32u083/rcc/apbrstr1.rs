///Register `APBRSTR1` reader
pub type R = crate::R<APBRSTR1rs>;
///Register `APBRSTR1` writer
pub type W = crate::W<APBRSTR1rs>;
///Field `TIM2RST` reader - TIM2 timer reset Set and cleared by software.
pub type TIM2RST_R = crate::BitReader;
///Field `TIM2RST` writer - TIM2 timer reset Set and cleared by software.
pub type TIM2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3RST` reader - TIM3 timer reset Set and cleared by software.
pub type TIM3RST_R = crate::BitReader;
///Field `TIM3RST` writer - TIM3 timer reset Set and cleared by software.
pub type TIM3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6RST` reader - TIM6 timer reset Set and cleared by software.
pub type TIM6RST_R = crate::BitReader;
///Field `TIM6RST` writer - TIM6 timer reset Set and cleared by software.
pub type TIM6RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7RST` reader - TIM7 timer reset Set and cleared by software.
pub type TIM7RST_R = crate::BitReader;
///Field `TIM7RST` writer - TIM7 timer reset Set and cleared by software.
pub type TIM7RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART2RST` reader - LPUART2 reset Set and cleared by software.
pub type LPUART2RST_R = crate::BitReader;
///Field `LPUART2RST` writer - LPUART2 reset Set and cleared by software.
pub type LPUART2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCDRST` reader - LCD reset<sup>(1)</sup> Set and cleared by software.
pub type LCDRST_R = crate::BitReader;
///Field `LCDRST` writer - LCD reset<sup>(1)</sup> Set and cleared by software.
pub type LCDRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART3RST` reader - LPUART3 reset<sup>(1)</sup> Set and cleared by software.
pub type LPUART3RST_R = crate::BitReader;
///Field `LPUART3RST` writer - LPUART3 reset<sup>(1)</sup> Set and cleared by software.
pub type LPUART3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBRST` reader - USB reset<sup>(1)</sup> Set and cleared by software.
pub type USBRST_R = crate::BitReader;
///Field `USBRST` writer - USB reset<sup>(1)</sup> Set and cleared by software.
pub type USBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2RST` reader - SPI2 reset Set and cleared by software.
pub type SPI2RST_R = crate::BitReader;
///Field `SPI2RST` writer - SPI2 reset Set and cleared by software.
pub type SPI2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3RST` reader - SPI3 reset<sup>(1)</sup> Set and cleared by software.
pub type SPI3RST_R = crate::BitReader;
///Field `SPI3RST` writer - SPI3 reset<sup>(1)</sup> Set and cleared by software.
pub type SPI3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRSRST` reader - CRS reset<sup>(1)</sup> Set and cleared by software.
pub type CRSRST_R = crate::BitReader;
///Field `CRSRST` writer - CRS reset<sup>(1)</sup> Set and cleared by software.
pub type CRSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2RST` reader - USART2 reset Set and cleared by software.
pub type USART2RST_R = crate::BitReader;
///Field `USART2RST` writer - USART2 reset Set and cleared by software.
pub type USART2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3RST` reader - USART3 reset Set and cleared by software.
pub type USART3RST_R = crate::BitReader;
///Field `USART3RST` writer - USART3 reset Set and cleared by software.
pub type USART3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART4RST` reader - USART4 reset Set and cleared by software.
pub type USART4RST_R = crate::BitReader;
///Field `USART4RST` writer - USART4 reset Set and cleared by software.
pub type USART4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1RST` reader - LPUART1 reset Set and cleared by software.
pub type LPUART1RST_R = crate::BitReader;
///Field `LPUART1RST` writer - LPUART1 reset Set and cleared by software.
pub type LPUART1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1RST` reader - I2C1 reset Set and cleared by software.
pub type I2C1RST_R = crate::BitReader;
///Field `I2C1RST` writer - I2C1 reset Set and cleared by software.
pub type I2C1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2RST` reader - I2C2 reset Set and cleared by software.
pub type I2C2RST_R = crate::BitReader;
///Field `I2C2RST` writer - I2C2 reset Set and cleared by software.
pub type I2C2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3RST` reader - I2C3 reset Set and cleared by software.
pub type I2C3RST_R = crate::BitReader;
///Field `I2C3RST` writer - I2C3 reset Set and cleared by software.
pub type I2C3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPAMPRST` reader - OPAMP reset Set and cleared by software.
pub type OPAMPRST_R = crate::BitReader;
///Field `OPAMPRST` writer - OPAMP reset Set and cleared by software.
pub type OPAMPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4RST` reader - I2C4 reset<sup>(1)</sup> Set and cleared by software.
pub type I2C4RST_R = crate::BitReader;
///Field `I2C4RST` writer - I2C4 reset<sup>(1)</sup> Set and cleared by software.
pub type I2C4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3RST` reader - LPTIM3 reset Set and cleared by software.
pub type LPTIM3RST_R = crate::BitReader;
///Field `LPTIM3RST` writer - LPTIM3 reset Set and cleared by software.
pub type LPTIM3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRRST` reader - Power interface reset Set and cleared by software.
pub type PWRRST_R = crate::BitReader;
///Field `PWRRST` writer - Power interface reset Set and cleared by software.
pub type PWRRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC1RST` reader - DAC1 interface reset Set and cleared by software.
pub type DAC1RST_R = crate::BitReader;
///Field `DAC1RST` writer - DAC1 interface reset Set and cleared by software.
pub type DAC1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2RST` reader - Low Power Timer 2 reset Set and cleared by software.
pub type LPTIM2RST_R = crate::BitReader;
///Field `LPTIM2RST` writer - Low Power Timer 2 reset Set and cleared by software.
pub type LPTIM2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1RST` reader - Low Power Timer 1 reset Set and cleared by software.
pub type LPTIM1RST_R = crate::BitReader;
///Field `LPTIM1RST` writer - Low Power Timer 1 reset Set and cleared by software.
pub type LPTIM1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM2 timer reset Set and cleared by software.
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 timer reset Set and cleared by software.
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - TIM6 timer reset Set and cleared by software.
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 timer reset Set and cleared by software.
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - LPUART2 reset Set and cleared by software.
    #[inline(always)]
    pub fn lpuart2rst(&self) -> LPUART2RST_R {
        LPUART2RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - LCD reset<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn lcdrst(&self) -> LCDRST_R {
        LCDRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - LPUART3 reset<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn lpuart3rst(&self) -> LPUART3RST_R {
        LPUART3RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - USB reset<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SPI2 reset Set and cleared by software.
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI3 reset<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - CRS reset<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - USART2 reset Set and cleared by software.
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 reset Set and cleared by software.
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - USART4 reset Set and cleared by software.
    #[inline(always)]
    pub fn usart4rst(&self) -> USART4RST_R {
        USART4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - LPUART1 reset Set and cleared by software.
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 reset Set and cleared by software.
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 reset Set and cleared by software.
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2C3 reset Set and cleared by software.
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - OPAMP reset Set and cleared by software.
    #[inline(always)]
    pub fn opamprst(&self) -> OPAMPRST_R {
        OPAMPRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - I2C4 reset<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - LPTIM3 reset Set and cleared by software.
    #[inline(always)]
    pub fn lptim3rst(&self) -> LPTIM3RST_R {
        LPTIM3RST_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - Power interface reset Set and cleared by software.
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC1 interface reset Set and cleared by software.
    #[inline(always)]
    pub fn dac1rst(&self) -> DAC1RST_R {
        DAC1RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Low Power Timer 2 reset Set and cleared by software.
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low Power Timer 1 reset Set and cleared by software.
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
            .field("tim6rst", &self.tim6rst())
            .field("tim7rst", &self.tim7rst())
            .field("lpuart2rst", &self.lpuart2rst())
            .field("lcdrst", &self.lcdrst())
            .field("lpuart3rst", &self.lpuart3rst())
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
            .field("opamprst", &self.opamprst())
            .field("i2c4rst", &self.i2c4rst())
            .field("lptim3rst", &self.lptim3rst())
            .field("pwrrst", &self.pwrrst())
            .field("dac1rst", &self.dac1rst())
            .field("lptim2rst", &self.lptim2rst())
            .field("lptim1rst", &self.lptim1rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 timer reset Set and cleared by software.
    #[inline(always)]
    pub fn tim2rst(&mut self) -> TIM2RST_W<APBRSTR1rs> {
        TIM2RST_W::new(self, 0)
    }
    ///Bit 1 - TIM3 timer reset Set and cleared by software.
    #[inline(always)]
    pub fn tim3rst(&mut self) -> TIM3RST_W<APBRSTR1rs> {
        TIM3RST_W::new(self, 1)
    }
    ///Bit 4 - TIM6 timer reset Set and cleared by software.
    #[inline(always)]
    pub fn tim6rst(&mut self) -> TIM6RST_W<APBRSTR1rs> {
        TIM6RST_W::new(self, 4)
    }
    ///Bit 5 - TIM7 timer reset Set and cleared by software.
    #[inline(always)]
    pub fn tim7rst(&mut self) -> TIM7RST_W<APBRSTR1rs> {
        TIM7RST_W::new(self, 5)
    }
    ///Bit 7 - LPUART2 reset Set and cleared by software.
    #[inline(always)]
    pub fn lpuart2rst(&mut self) -> LPUART2RST_W<APBRSTR1rs> {
        LPUART2RST_W::new(self, 7)
    }
    ///Bit 9 - LCD reset<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn lcdrst(&mut self) -> LCDRST_W<APBRSTR1rs> {
        LCDRST_W::new(self, 9)
    }
    ///Bit 12 - LPUART3 reset<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn lpuart3rst(&mut self) -> LPUART3RST_W<APBRSTR1rs> {
        LPUART3RST_W::new(self, 12)
    }
    ///Bit 13 - USB reset<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W<APBRSTR1rs> {
        USBRST_W::new(self, 13)
    }
    ///Bit 14 - SPI2 reset Set and cleared by software.
    #[inline(always)]
    pub fn spi2rst(&mut self) -> SPI2RST_W<APBRSTR1rs> {
        SPI2RST_W::new(self, 14)
    }
    ///Bit 15 - SPI3 reset<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn spi3rst(&mut self) -> SPI3RST_W<APBRSTR1rs> {
        SPI3RST_W::new(self, 15)
    }
    ///Bit 16 - CRS reset<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn crsrst(&mut self) -> CRSRST_W<APBRSTR1rs> {
        CRSRST_W::new(self, 16)
    }
    ///Bit 17 - USART2 reset Set and cleared by software.
    #[inline(always)]
    pub fn usart2rst(&mut self) -> USART2RST_W<APBRSTR1rs> {
        USART2RST_W::new(self, 17)
    }
    ///Bit 18 - USART3 reset Set and cleared by software.
    #[inline(always)]
    pub fn usart3rst(&mut self) -> USART3RST_W<APBRSTR1rs> {
        USART3RST_W::new(self, 18)
    }
    ///Bit 19 - USART4 reset Set and cleared by software.
    #[inline(always)]
    pub fn usart4rst(&mut self) -> USART4RST_W<APBRSTR1rs> {
        USART4RST_W::new(self, 19)
    }
    ///Bit 20 - LPUART1 reset Set and cleared by software.
    #[inline(always)]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<APBRSTR1rs> {
        LPUART1RST_W::new(self, 20)
    }
    ///Bit 21 - I2C1 reset Set and cleared by software.
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<APBRSTR1rs> {
        I2C1RST_W::new(self, 21)
    }
    ///Bit 22 - I2C2 reset Set and cleared by software.
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<APBRSTR1rs> {
        I2C2RST_W::new(self, 22)
    }
    ///Bit 23 - I2C3 reset Set and cleared by software.
    #[inline(always)]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<APBRSTR1rs> {
        I2C3RST_W::new(self, 23)
    }
    ///Bit 24 - OPAMP reset Set and cleared by software.
    #[inline(always)]
    pub fn opamprst(&mut self) -> OPAMPRST_W<APBRSTR1rs> {
        OPAMPRST_W::new(self, 24)
    }
    ///Bit 25 - I2C4 reset<sup>(1)</sup> Set and cleared by software.
    #[inline(always)]
    pub fn i2c4rst(&mut self) -> I2C4RST_W<APBRSTR1rs> {
        I2C4RST_W::new(self, 25)
    }
    ///Bit 26 - LPTIM3 reset Set and cleared by software.
    #[inline(always)]
    pub fn lptim3rst(&mut self) -> LPTIM3RST_W<APBRSTR1rs> {
        LPTIM3RST_W::new(self, 26)
    }
    ///Bit 28 - Power interface reset Set and cleared by software.
    #[inline(always)]
    pub fn pwrrst(&mut self) -> PWRRST_W<APBRSTR1rs> {
        PWRRST_W::new(self, 28)
    }
    ///Bit 29 - DAC1 interface reset Set and cleared by software.
    #[inline(always)]
    pub fn dac1rst(&mut self) -> DAC1RST_W<APBRSTR1rs> {
        DAC1RST_W::new(self, 29)
    }
    ///Bit 30 - Low Power Timer 2 reset Set and cleared by software.
    #[inline(always)]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W<APBRSTR1rs> {
        LPTIM2RST_W::new(self, 30)
    }
    ///Bit 31 - Low Power Timer 1 reset Set and cleared by software.
    #[inline(always)]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<APBRSTR1rs> {
        LPTIM1RST_W::new(self, 31)
    }
}
/**APB peripheral reset register 1

You can [`read`](crate::Reg::read) this register and get [`apbrstr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrstr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:APBRSTR1)*/
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
