#[doc = "Register `CCIPR` reader"]
pub type R = crate::R<CCIPRrs>;
#[doc = "Register `CCIPR` writer"]
pub type W = crate::W<CCIPRrs>;
#[doc = "Field `USART1SEL` reader - USART1 clock source selection"]
pub type USART1SEL_R = crate::FieldReader;
#[doc = "Field `USART1SEL` writer - USART1 clock source selection"]
pub type USART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPUART1SEL` reader - LPUART1 clock source selection"]
pub type LPUART1SEL_R = crate::FieldReader;
#[doc = "Field `LPUART1SEL` writer - LPUART1 clock source selection"]
pub type LPUART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2C1SEL` reader - I2C1 clock source selection"]
pub type I2C1SEL_R = crate::FieldReader;
#[doc = "Field `I2C1SEL` writer - I2C1 clock source selection"]
pub type I2C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2C3SEL` reader - I2C3 clock source selection"]
pub type I2C3SEL_R = crate::FieldReader;
#[doc = "Field `I2C3SEL` writer - I2C3 clock source selection"]
pub type I2C3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPTIM1SEL` reader - Low power timer 1 clock source selection"]
pub type LPTIM1SEL_R = crate::FieldReader;
#[doc = "Field `LPTIM1SEL` writer - Low power timer 1 clock source selection"]
pub type LPTIM1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPTIM2SEL` reader - Low power timer 2 clock source selection"]
pub type LPTIM2SEL_R = crate::FieldReader;
#[doc = "Field `LPTIM2SEL` writer - Low power timer 2 clock source selection"]
pub type LPTIM2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAI1SEL` reader - SAI1 clock source selection"]
pub type SAI1SEL_R = crate::FieldReader;
#[doc = "Field `SAI1SEL` writer - SAI1 clock source selection"]
pub type SAI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLK48SEL` reader - 48 MHz clock source selection"]
pub type CLK48SEL_R = crate::FieldReader;
#[doc = "Field `CLK48SEL` writer - 48 MHz clock source selection"]
pub type CLK48SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADCSEL` reader - ADCs clock source selection"]
pub type ADCSEL_R = crate::FieldReader;
#[doc = "Field `ADCSEL` writer - ADCs clock source selection"]
pub type ADCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RNGSEL` reader - RNG clock source selection"]
pub type RNGSEL_R = crate::FieldReader;
#[doc = "Field `RNGSEL` writer - RNG clock source selection"]
pub type RNGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
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
    #[doc = "Bits 16:17 - I2C3 clock source selection"]
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Low power timer 1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Low power timer 2 clock source selection"]
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - SAI1 clock source selection"]
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - 48 MHz clock source selection"]
    #[inline(always)]
    pub fn clk48sel(&self) -> CLK48SEL_R {
        CLK48SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - ADCs clock source selection"]
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - RNG clock source selection"]
    #[inline(always)]
    pub fn rngsel(&self) -> RNGSEL_R {
        RNGSEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> USART1SEL_W<CCIPRrs> {
        USART1SEL_W::new(self, 0)
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
    #[doc = "Bits 16:17 - I2C3 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W<CCIPRrs> {
        I2C3SEL_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Low power timer 1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<CCIPRrs> {
        LPTIM1SEL_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Low power timer 2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<CCIPRrs> {
        LPTIM2SEL_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - SAI1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<CCIPRrs> {
        SAI1SEL_W::new(self, 22)
    }
    #[doc = "Bits 26:27 - 48 MHz clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn clk48sel(&mut self) -> CLK48SEL_W<CCIPRrs> {
        CLK48SEL_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - ADCs clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn adcsel(&mut self) -> ADCSEL_W<CCIPRrs> {
        ADCSEL_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - RNG clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn rngsel(&mut self) -> RNGSEL_W<CCIPRrs> {
        RNGSEL_W::new(self, 30)
    }
}
#[doc = "CCIPR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
