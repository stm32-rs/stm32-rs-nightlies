#[doc = "Register `CCIPR` reader"]
pub type R = crate::R<CCIPRrs>;
#[doc = "Register `CCIPR` writer"]
pub type W = crate::W<CCIPRrs>;
#[doc = "Field `USART1SEL` reader - USART1 clock source selection"]
pub type USART1SEL_R = crate::FieldReader;
#[doc = "Field `USART1SEL` writer - USART1 clock source selection"]
pub type USART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USART2SEL` reader - USART2 clock source selection"]
pub type USART2SEL_R = crate::FieldReader;
#[doc = "Field `USART2SEL` writer - USART2 clock source selection"]
pub type USART2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USART3SEL` reader - USART3 clock source selection"]
pub type USART3SEL_R = crate::FieldReader;
#[doc = "Field `USART3SEL` writer - USART3 clock source selection"]
pub type USART3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CECSEL` reader - HDMI CEC clock source selection"]
pub type CECSEL_R = crate::BitReader;
#[doc = "Field `CECSEL` writer - HDMI CEC clock source selection"]
pub type CECSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUART2SEL` reader - LPUART2 clock source selection"]
pub type LPUART2SEL_R = crate::FieldReader;
#[doc = "Field `LPUART2SEL` writer - LPUART2 clock source selection"]
pub type LPUART2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPUART1SEL` reader - LPUART1 clock source selection"]
pub type LPUART1SEL_R = crate::FieldReader;
#[doc = "Field `LPUART1SEL` writer - LPUART1 clock source selection"]
pub type LPUART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2C1SEL` reader - I2C1 clock source selection"]
pub type I2C1SEL_R = crate::FieldReader;
#[doc = "Field `I2C1SEL` writer - I2C1 clock source selection"]
pub type I2C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2S2SEL` reader - I2S1 clock source selection"]
pub type I2S2SEL_R = crate::FieldReader;
#[doc = "Field `I2S2SEL` writer - I2S1 clock source selection"]
pub type I2S2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPTIM1SEL` reader - LPTIM1 clock source selection"]
pub type LPTIM1SEL_R = crate::FieldReader;
#[doc = "Field `LPTIM1SEL` writer - LPTIM1 clock source selection"]
pub type LPTIM1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPTIM2SEL` reader - LPTIM2 clock source selection"]
pub type LPTIM2SEL_R = crate::FieldReader;
#[doc = "Field `LPTIM2SEL` writer - LPTIM2 clock source selection"]
pub type LPTIM2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIM1SEL` reader - TIM1 clock source selection"]
pub type TIM1SEL_R = crate::BitReader;
#[doc = "Field `TIM1SEL` writer - TIM1 clock source selection"]
pub type TIM1SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15SEL` reader - TIM15 clock source selection"]
pub type TIM15SEL_R = crate::BitReader;
#[doc = "Field `TIM15SEL` writer - TIM15 clock source selection"]
pub type TIM15SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGSEL` reader - RNG clock source selection"]
pub type RNGSEL_R = crate::FieldReader;
#[doc = "Field `RNGSEL` writer - RNG clock source selection"]
pub type RNGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RNGDIV` reader - Division factor of RNG clock divider"]
pub type RNGDIV_R = crate::FieldReader;
#[doc = "Field `RNGDIV` writer - Division factor of RNG clock divider"]
pub type RNGDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADCSEL` reader - ADCs clock source selection"]
pub type ADCSEL_R = crate::FieldReader;
#[doc = "Field `ADCSEL` writer - ADCs clock source selection"]
pub type ADCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - USART3 clock source selection"]
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - HDMI CEC clock source selection"]
    #[inline(always)]
    pub fn cecsel(&self) -> CECSEL_R {
        CECSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - LPUART2 clock source selection"]
    #[inline(always)]
    pub fn lpuart2sel(&self) -> LPUART2SEL_R {
        LPUART2SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection"]
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - I2S1 clock source selection"]
    #[inline(always)]
    pub fn i2s2sel(&self) -> I2S2SEL_R {
        I2S2SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 18:19 - LPTIM1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - LPTIM2 clock source selection"]
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - TIM1 clock source selection"]
    #[inline(always)]
    pub fn tim1sel(&self) -> TIM1SEL_R {
        TIM1SEL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - TIM15 clock source selection"]
    #[inline(always)]
    pub fn tim15sel(&self) -> TIM15SEL_R {
        TIM15SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 26:27 - RNG clock source selection"]
    #[inline(always)]
    pub fn rngsel(&self) -> RNGSEL_R {
        RNGSEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Division factor of RNG clock divider"]
    #[inline(always)]
    pub fn rngdiv(&self) -> RNGDIV_R {
        RNGDIV_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - ADCs clock source selection"]
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> USART1SEL_W<CCIPRrs> {
        USART1SEL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart2sel(&mut self) -> USART2SEL_W<CCIPRrs> {
        USART2SEL_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - USART3 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart3sel(&mut self) -> USART3SEL_W<CCIPRrs> {
        USART3SEL_W::new(self, 4)
    }
    #[doc = "Bit 6 - HDMI CEC clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn cecsel(&mut self) -> CECSEL_W<CCIPRrs> {
        CECSEL_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - LPUART2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart2sel(&mut self) -> LPUART2SEL_W<CCIPRrs> {
        LPUART2SEL_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<CCIPRrs> {
        LPUART1SEL_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<CCIPRrs> {
        I2C1SEL_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - I2S1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2s2sel(&mut self) -> I2S2SEL_W<CCIPRrs> {
        I2S2SEL_W::new(self, 14)
    }
    #[doc = "Bits 18:19 - LPTIM1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<CCIPRrs> {
        LPTIM1SEL_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - LPTIM2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<CCIPRrs> {
        LPTIM2SEL_W::new(self, 20)
    }
    #[doc = "Bit 22 - TIM1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn tim1sel(&mut self) -> TIM1SEL_W<CCIPRrs> {
        TIM1SEL_W::new(self, 22)
    }
    #[doc = "Bit 24 - TIM15 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn tim15sel(&mut self) -> TIM15SEL_W<CCIPRrs> {
        TIM15SEL_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - RNG clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn rngsel(&mut self) -> RNGSEL_W<CCIPRrs> {
        RNGSEL_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Division factor of RNG clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn rngdiv(&mut self) -> RNGDIV_W<CCIPRrs> {
        RNGDIV_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - ADCs clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn adcsel(&mut self) -> ADCSEL_W<CCIPRrs> {
        ADCSEL_W::new(self, 30)
    }
}
#[doc = "Peripherals independent clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCIPRrs;
impl crate::RegisterSpec for CCIPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr::R`](R) reader structure"]
impl crate::Readable for CCIPRrs {}
#[doc = "`write(|w| ..)` method takes [`ccipr::W`](W) writer structure"]
impl crate::Writable for CCIPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCIPR to value 0"]
impl crate::Resettable for CCIPRrs {
    const RESET_VALUE: u32 = 0;
}
