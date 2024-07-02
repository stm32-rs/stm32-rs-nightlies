///Register `CCIPR` reader
pub type R = crate::R<CCIPRrs>;
///Register `CCIPR` writer
pub type W = crate::W<CCIPRrs>;
///Field `USART1SEL` reader - USART1 clock source selection
pub type USART1SEL_R = crate::FieldReader;
///Field `USART1SEL` writer - USART1 clock source selection
pub type USART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPUART1SEL` reader - LPUART1 clock source selection
pub type LPUART1SEL_R = crate::FieldReader;
///Field `LPUART1SEL` writer - LPUART1 clock source selection
pub type LPUART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `I2C1SEL` reader - I2C1 clock source selection
pub type I2C1SEL_R = crate::FieldReader;
///Field `I2C1SEL` writer - I2C1 clock source selection
pub type I2C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `I2S2SEL` reader - I2S1 clock source selection
pub type I2S2SEL_R = crate::FieldReader;
///Field `I2S2SEL` writer - I2S1 clock source selection
pub type I2S2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPTIM1SEL` reader - LPTIM1 clock source selection
pub type LPTIM1SEL_R = crate::FieldReader;
///Field `LPTIM1SEL` writer - LPTIM1 clock source selection
pub type LPTIM1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPTIM2SEL` reader - LPTIM2 clock source selection
pub type LPTIM2SEL_R = crate::FieldReader;
///Field `LPTIM2SEL` writer - LPTIM2 clock source selection
pub type LPTIM2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TIM1SEL` reader - TIM1 clock source selection
pub type TIM1SEL_R = crate::BitReader;
///Field `TIM1SEL` writer - TIM1 clock source selection
pub type TIM1SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGSEL` reader - RNG clock source selection
pub type RNGSEL_R = crate::FieldReader;
///Field `RNGSEL` writer - RNG clock source selection
pub type RNGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RNGDIV` reader - Division factor of RNG clock divider
pub type RNGDIV_R = crate::FieldReader;
///Field `RNGDIV` writer - Division factor of RNG clock divider
pub type RNGDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ADCSEL` reader - ADCs clock source selection
pub type ADCSEL_R = crate::FieldReader;
///Field `ADCSEL` writer - ADCs clock source selection
pub type ADCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - USART1 clock source selection
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 10:11 - LPUART1 clock source selection
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - I2C1 clock source selection
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - I2S1 clock source selection
    #[inline(always)]
    pub fn i2s2sel(&self) -> I2S2SEL_R {
        I2S2SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 18:19 - LPTIM1 clock source selection
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - LPTIM2 clock source selection
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - TIM1 clock source selection
    #[inline(always)]
    pub fn tim1sel(&self) -> TIM1SEL_R {
        TIM1SEL_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 26:27 - RNG clock source selection
    #[inline(always)]
    pub fn rngsel(&self) -> RNGSEL_R {
        RNGSEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Division factor of RNG clock divider
    #[inline(always)]
    pub fn rngdiv(&self) -> RNGDIV_R {
        RNGDIV_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - ADCs clock source selection
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR")
            .field("usart1sel", &self.usart1sel())
            .field("lpuart1sel", &self.lpuart1sel())
            .field("i2c1sel", &self.i2c1sel())
            .field("i2s2sel", &self.i2s2sel())
            .field("lptim1sel", &self.lptim1sel())
            .field("lptim2sel", &self.lptim2sel())
            .field("tim1sel", &self.tim1sel())
            .field("rngsel", &self.rngsel())
            .field("rngdiv", &self.rngdiv())
            .field("adcsel", &self.adcsel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - USART1 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> USART1SEL_W<CCIPRrs> {
        USART1SEL_W::new(self, 0)
    }
    ///Bits 10:11 - LPUART1 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<CCIPRrs> {
        LPUART1SEL_W::new(self, 10)
    }
    ///Bits 12:13 - I2C1 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<CCIPRrs> {
        I2C1SEL_W::new(self, 12)
    }
    ///Bits 14:15 - I2S1 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn i2s2sel(&mut self) -> I2S2SEL_W<CCIPRrs> {
        I2S2SEL_W::new(self, 14)
    }
    ///Bits 18:19 - LPTIM1 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<CCIPRrs> {
        LPTIM1SEL_W::new(self, 18)
    }
    ///Bits 20:21 - LPTIM2 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<CCIPRrs> {
        LPTIM2SEL_W::new(self, 20)
    }
    ///Bit 22 - TIM1 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn tim1sel(&mut self) -> TIM1SEL_W<CCIPRrs> {
        TIM1SEL_W::new(self, 22)
    }
    ///Bits 26:27 - RNG clock source selection
    #[inline(always)]
    #[must_use]
    pub fn rngsel(&mut self) -> RNGSEL_W<CCIPRrs> {
        RNGSEL_W::new(self, 26)
    }
    ///Bits 28:29 - Division factor of RNG clock divider
    #[inline(always)]
    #[must_use]
    pub fn rngdiv(&mut self) -> RNGDIV_W<CCIPRrs> {
        RNGDIV_W::new(self, 28)
    }
    ///Bits 30:31 - ADCs clock source selection
    #[inline(always)]
    #[must_use]
    pub fn adcsel(&mut self) -> ADCSEL_W<CCIPRrs> {
        ADCSEL_W::new(self, 30)
    }
}
/**Peripherals independent clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G041.html#RCC:CCIPR)*/
pub struct CCIPRrs;
impl crate::RegisterSpec for CCIPRrs {
    type Ux = u32;
}
///`read()` method returns [`ccipr::R`](R) reader structure
impl crate::Readable for CCIPRrs {}
///`write(|w| ..)` method takes [`ccipr::W`](W) writer structure
impl crate::Writable for CCIPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCIPR to value 0
impl crate::Resettable for CCIPRrs {
    const RESET_VALUE: u32 = 0;
}
