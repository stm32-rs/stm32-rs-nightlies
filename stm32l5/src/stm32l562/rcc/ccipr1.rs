#[doc = "Register `CCIPR1` reader"]
pub type R = crate::R<CCIPR1rs>;
#[doc = "Register `CCIPR1` writer"]
pub type W = crate::W<CCIPR1rs>;
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
#[doc = "Field `UART4SEL` reader - UART4 clock source selection"]
pub type UART4SEL_R = crate::FieldReader;
#[doc = "Field `UART4SEL` writer - UART4 clock source selection"]
pub type UART4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART5SEL` reader - UART5 clock source selection"]
pub type UART5SEL_R = crate::FieldReader;
#[doc = "Field `UART5SEL` writer - UART5 clock source selection"]
pub type UART5SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPUART1SEL` reader - LPUART1 clock source selection"]
pub type LPUART1SEL_R = crate::FieldReader;
#[doc = "Field `LPUART1SEL` writer - LPUART1 clock source selection"]
pub type LPUART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2C1SEL` reader - I2C1 clock source selection"]
pub type I2C1SEL_R = crate::FieldReader;
#[doc = "Field `I2C1SEL` writer - I2C1 clock source selection"]
pub type I2C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2C2SEL` reader - I2C2 clock source selection"]
pub type I2C2SEL_R = crate::FieldReader;
#[doc = "Field `I2C2SEL` writer - I2C2 clock source selection"]
pub type I2C2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
#[doc = "Field `LPTIM3SEL` reader - Low-power timer 3 clock source selection"]
pub type LPTIM3SEL_R = crate::FieldReader;
#[doc = "Field `LPTIM3SEL` writer - Low-power timer 3 clock source selection"]
pub type LPTIM3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FDCANSEL` reader - FDCAN clock source selection"]
pub type FDCANSEL_R = crate::FieldReader;
#[doc = "Field `FDCANSEL` writer - FDCAN clock source selection"]
pub type FDCANSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLK48MSEL` reader - 48 MHz clock source selection"]
pub type CLK48MSEL_R = crate::FieldReader;
#[doc = "Field `CLK48MSEL` writer - 48 MHz clock source selection"]
pub type CLK48MSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    #[doc = "Bits 6:7 - UART4 clock source selection"]
    #[inline(always)]
    pub fn uart4sel(&self) -> UART4SEL_R {
        UART4SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - UART5 clock source selection"]
    #[inline(always)]
    pub fn uart5sel(&self) -> UART5SEL_R {
        UART5SEL_R::new(((self.bits >> 8) & 3) as u8)
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
    #[doc = "Bits 14:15 - I2C2 clock source selection"]
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 14) & 3) as u8)
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
    #[doc = "Bits 22:23 - Low-power timer 3 clock source selection"]
    #[inline(always)]
    pub fn lptim3sel(&self) -> LPTIM3SEL_R {
        LPTIM3SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - FDCAN clock source selection"]
    #[inline(always)]
    pub fn fdcansel(&self) -> FDCANSEL_R {
        FDCANSEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - 48 MHz clock source selection"]
    #[inline(always)]
    pub fn clk48msel(&self) -> CLK48MSEL_R {
        CLK48MSEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - ADCs clock source selection"]
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> USART1SEL_W<CCIPR1rs> {
        USART1SEL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart2sel(&mut self) -> USART2SEL_W<CCIPR1rs> {
        USART2SEL_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - USART3 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart3sel(&mut self) -> USART3SEL_W<CCIPR1rs> {
        USART3SEL_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - UART4 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn uart4sel(&mut self) -> UART4SEL_W<CCIPR1rs> {
        UART4SEL_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - UART5 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn uart5sel(&mut self) -> UART5SEL_W<CCIPR1rs> {
        UART5SEL_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<CCIPR1rs> {
        LPUART1SEL_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<CCIPR1rs> {
        I2C1SEL_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - I2C2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W<CCIPR1rs> {
        I2C2SEL_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W<CCIPR1rs> {
        I2C3SEL_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Low power timer 1 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<CCIPR1rs> {
        LPTIM1SEL_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Low power timer 2 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<CCIPR1rs> {
        LPTIM2SEL_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Low-power timer 3 clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3sel(&mut self) -> LPTIM3SEL_W<CCIPR1rs> {
        LPTIM3SEL_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - FDCAN clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn fdcansel(&mut self) -> FDCANSEL_W<CCIPR1rs> {
        FDCANSEL_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - 48 MHz clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn clk48msel(&mut self) -> CLK48MSEL_W<CCIPR1rs> {
        CLK48MSEL_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - ADCs clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn adcsel(&mut self) -> ADCSEL_W<CCIPR1rs> {
        ADCSEL_W::new(self, 28)
    }
}
#[doc = "CCIPR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccipr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccipr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCIPR1rs;
impl crate::RegisterSpec for CCIPR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr1::R`](R) reader structure"]
impl crate::Readable for CCIPR1rs {}
#[doc = "`write(|w| ..)` method takes [`ccipr1::W`](W) writer structure"]
impl crate::Writable for CCIPR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCIPR1 to value 0"]
impl crate::Resettable for CCIPR1rs {
    const RESET_VALUE: u32 = 0;
}
