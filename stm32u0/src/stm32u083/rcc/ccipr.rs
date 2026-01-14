///Register `CCIPR` reader
pub type R = crate::R<CCIPRrs>;
///Register `CCIPR` writer
pub type W = crate::W<CCIPRrs>;
///Field `USART1SEL` reader - USART1 clock source selection This bitfield is controlled by software to select USART1 clock source as follows:
pub type USART1SEL_R = crate::FieldReader;
///Field `USART1SEL` writer - USART1 clock source selection This bitfield is controlled by software to select USART1 clock source as follows:
pub type USART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `USART2SEL` reader - USART2 clock source selection This bitfield is controlled by software to select USART2 clock source as follows:
pub type USART2SEL_R = crate::FieldReader;
///Field `USART2SEL` writer - USART2 clock source selection This bitfield is controlled by software to select USART2 clock source as follows:
pub type USART2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPUART3SEL` reader - LPUART3 clock source selection<sup>(1)</sup> This bitfield is controlled by software to select LPUART3 clock source as follows:
pub type LPUART3SEL_R = crate::FieldReader;
///Field `LPUART3SEL` writer - LPUART3 clock source selection<sup>(1)</sup> This bitfield is controlled by software to select LPUART3 clock source as follows:
pub type LPUART3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPUART2SEL` reader - LPUART2 clock source selection This bitfield is controlled by software to select LPUART2 clock source as follows:
pub type LPUART2SEL_R = crate::FieldReader;
///Field `LPUART2SEL` writer - LPUART2 clock source selection This bitfield is controlled by software to select LPUART2 clock source as follows:
pub type LPUART2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPUART1SEL` reader - LPUART1 clock source selection This bitfield is controlled by software to select LPUART1 clock source as follows:
pub type LPUART1SEL_R = crate::FieldReader;
///Field `LPUART1SEL` writer - LPUART1 clock source selection This bitfield is controlled by software to select LPUART1 clock source as follows:
pub type LPUART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `I2C1SEL` reader - I2C1 clock source selection This bitfield is controlled by software to select I2C1 clock source as follows:
pub type I2C1SEL_R = crate::FieldReader;
///Field `I2C1SEL` writer - I2C1 clock source selection This bitfield is controlled by software to select I2C1 clock source as follows:
pub type I2C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `I2C3SEL` reader - I2C3 clock source selection This bitfield is controlled by software to select I2C3 clock source as follows:
pub type I2C3SEL_R = crate::FieldReader;
///Field `I2C3SEL` writer - I2C3 clock source selection This bitfield is controlled by software to select I2C3 clock source as follows:
pub type I2C3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPTIM1SEL` reader - LPTIM1 clock source selection This bitfield is controlled by software to select LPTIM1 clock source as follows:
pub type LPTIM1SEL_R = crate::FieldReader;
///Field `LPTIM1SEL` writer - LPTIM1 clock source selection This bitfield is controlled by software to select LPTIM1 clock source as follows:
pub type LPTIM1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPTIM2SEL` reader - LPTIM2 clock source selection This bitfield is controlled by software to select LPTIM2 clock source as follows:
pub type LPTIM2SEL_R = crate::FieldReader;
///Field `LPTIM2SEL` writer - LPTIM2 clock source selection This bitfield is controlled by software to select LPTIM2 clock source as follows:
pub type LPTIM2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LPTIM3SEL` reader - LPTIM3 clock source selection This bitfield is controlled by software to select LPTIM3 clock source as follows:
pub type LPTIM3SEL_R = crate::FieldReader;
///Field `LPTIM3SEL` writer - LPTIM3 clock source selection This bitfield is controlled by software to select LPTIM3 clock source as follows:
pub type LPTIM3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TIM1SEL` reader - TIM1 clock source selection This bit is set and cleared by software. It selects TIM1 clock source as follows:
pub type TIM1SEL_R = crate::BitReader;
///Field `TIM1SEL` writer - TIM1 clock source selection This bit is set and cleared by software. It selects TIM1 clock source as follows:
pub type TIM1SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM15SEL` reader - TIM15 clock source selection This bit is set and cleared by software. It selects TIM15 clock source as follows:
pub type TIM15SEL_R = crate::BitReader;
///Field `TIM15SEL` writer - TIM15 clock source selection This bit is set and cleared by software. It selects TIM15 clock source as follows:
pub type TIM15SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK48SEL` reader - 481MHz clock source selection This bitfield is controlled by software to select the 481MHz clock source used by the USB FS and the RNG:
pub type CLK48SEL_R = crate::FieldReader;
///Field `CLK48SEL` writer - 481MHz clock source selection This bitfield is controlled by software to select the 481MHz clock source used by the USB FS and the RNG:
pub type CLK48SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ADCSEL` reader - ADCs clock source selection This bitfield is controlled by software to select the clock source for ADC:
pub type ADCSEL_R = crate::FieldReader;
///Field `ADCSEL` writer - ADCs clock source selection This bitfield is controlled by software to select the clock source for ADC:
pub type ADCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - USART1 clock source selection This bitfield is controlled by software to select USART1 clock source as follows:
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - USART2 clock source selection This bitfield is controlled by software to select USART2 clock source as follows:
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 6:7 - LPUART3 clock source selection<sup>(1)</sup> This bitfield is controlled by software to select LPUART3 clock source as follows:
    #[inline(always)]
    pub fn lpuart3sel(&self) -> LPUART3SEL_R {
        LPUART3SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - LPUART2 clock source selection This bitfield is controlled by software to select LPUART2 clock source as follows:
    #[inline(always)]
    pub fn lpuart2sel(&self) -> LPUART2SEL_R {
        LPUART2SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - LPUART1 clock source selection This bitfield is controlled by software to select LPUART1 clock source as follows:
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - I2C1 clock source selection This bitfield is controlled by software to select I2C1 clock source as follows:
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 16:17 - I2C3 clock source selection This bitfield is controlled by software to select I2C3 clock source as follows:
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - LPTIM1 clock source selection This bitfield is controlled by software to select LPTIM1 clock source as follows:
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - LPTIM2 clock source selection This bitfield is controlled by software to select LPTIM2 clock source as follows:
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - LPTIM3 clock source selection This bitfield is controlled by software to select LPTIM3 clock source as follows:
    #[inline(always)]
    pub fn lptim3sel(&self) -> LPTIM3SEL_R {
        LPTIM3SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 24 - TIM1 clock source selection This bit is set and cleared by software. It selects TIM1 clock source as follows:
    #[inline(always)]
    pub fn tim1sel(&self) -> TIM1SEL_R {
        TIM1SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - TIM15 clock source selection This bit is set and cleared by software. It selects TIM15 clock source as follows:
    #[inline(always)]
    pub fn tim15sel(&self) -> TIM15SEL_R {
        TIM15SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 26:27 - 481MHz clock source selection This bitfield is controlled by software to select the 481MHz clock source used by the USB FS and the RNG:
    #[inline(always)]
    pub fn clk48sel(&self) -> CLK48SEL_R {
        CLK48SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - ADCs clock source selection This bitfield is controlled by software to select the clock source for ADC:
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR")
            .field("usart1sel", &self.usart1sel())
            .field("usart2sel", &self.usart2sel())
            .field("lpuart3sel", &self.lpuart3sel())
            .field("lpuart2sel", &self.lpuart2sel())
            .field("lpuart1sel", &self.lpuart1sel())
            .field("i2c1sel", &self.i2c1sel())
            .field("i2c3sel", &self.i2c3sel())
            .field("lptim1sel", &self.lptim1sel())
            .field("lptim2sel", &self.lptim2sel())
            .field("lptim3sel", &self.lptim3sel())
            .field("tim1sel", &self.tim1sel())
            .field("tim15sel", &self.tim15sel())
            .field("clk48sel", &self.clk48sel())
            .field("adcsel", &self.adcsel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - USART1 clock source selection This bitfield is controlled by software to select USART1 clock source as follows:
    #[inline(always)]
    pub fn usart1sel(&mut self) -> USART1SEL_W<'_, CCIPRrs> {
        USART1SEL_W::new(self, 0)
    }
    ///Bits 2:3 - USART2 clock source selection This bitfield is controlled by software to select USART2 clock source as follows:
    #[inline(always)]
    pub fn usart2sel(&mut self) -> USART2SEL_W<'_, CCIPRrs> {
        USART2SEL_W::new(self, 2)
    }
    ///Bits 6:7 - LPUART3 clock source selection<sup>(1)</sup> This bitfield is controlled by software to select LPUART3 clock source as follows:
    #[inline(always)]
    pub fn lpuart3sel(&mut self) -> LPUART3SEL_W<'_, CCIPRrs> {
        LPUART3SEL_W::new(self, 6)
    }
    ///Bits 8:9 - LPUART2 clock source selection This bitfield is controlled by software to select LPUART2 clock source as follows:
    #[inline(always)]
    pub fn lpuart2sel(&mut self) -> LPUART2SEL_W<'_, CCIPRrs> {
        LPUART2SEL_W::new(self, 8)
    }
    ///Bits 10:11 - LPUART1 clock source selection This bitfield is controlled by software to select LPUART1 clock source as follows:
    #[inline(always)]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<'_, CCIPRrs> {
        LPUART1SEL_W::new(self, 10)
    }
    ///Bits 12:13 - I2C1 clock source selection This bitfield is controlled by software to select I2C1 clock source as follows:
    #[inline(always)]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<'_, CCIPRrs> {
        I2C1SEL_W::new(self, 12)
    }
    ///Bits 16:17 - I2C3 clock source selection This bitfield is controlled by software to select I2C3 clock source as follows:
    #[inline(always)]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W<'_, CCIPRrs> {
        I2C3SEL_W::new(self, 16)
    }
    ///Bits 18:19 - LPTIM1 clock source selection This bitfield is controlled by software to select LPTIM1 clock source as follows:
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<'_, CCIPRrs> {
        LPTIM1SEL_W::new(self, 18)
    }
    ///Bits 20:21 - LPTIM2 clock source selection This bitfield is controlled by software to select LPTIM2 clock source as follows:
    #[inline(always)]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<'_, CCIPRrs> {
        LPTIM2SEL_W::new(self, 20)
    }
    ///Bits 22:23 - LPTIM3 clock source selection This bitfield is controlled by software to select LPTIM3 clock source as follows:
    #[inline(always)]
    pub fn lptim3sel(&mut self) -> LPTIM3SEL_W<'_, CCIPRrs> {
        LPTIM3SEL_W::new(self, 22)
    }
    ///Bit 24 - TIM1 clock source selection This bit is set and cleared by software. It selects TIM1 clock source as follows:
    #[inline(always)]
    pub fn tim1sel(&mut self) -> TIM1SEL_W<'_, CCIPRrs> {
        TIM1SEL_W::new(self, 24)
    }
    ///Bit 25 - TIM15 clock source selection This bit is set and cleared by software. It selects TIM15 clock source as follows:
    #[inline(always)]
    pub fn tim15sel(&mut self) -> TIM15SEL_W<'_, CCIPRrs> {
        TIM15SEL_W::new(self, 25)
    }
    ///Bits 26:27 - 481MHz clock source selection This bitfield is controlled by software to select the 481MHz clock source used by the USB FS and the RNG:
    #[inline(always)]
    pub fn clk48sel(&mut self) -> CLK48SEL_W<'_, CCIPRrs> {
        CLK48SEL_W::new(self, 26)
    }
    ///Bits 28:29 - ADCs clock source selection This bitfield is controlled by software to select the clock source for ADC:
    #[inline(always)]
    pub fn adcsel(&mut self) -> ADCSEL_W<'_, CCIPRrs> {
        ADCSEL_W::new(self, 28)
    }
}
/**Peripherals independent clock configuration register

You can [`read`](crate::Reg::read) this register and get [`ccipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:CCIPR)*/
pub struct CCIPRrs;
impl crate::RegisterSpec for CCIPRrs {
    type Ux = u32;
}
///`read()` method returns [`ccipr::R`](R) reader structure
impl crate::Readable for CCIPRrs {}
///`write(|w| ..)` method takes [`ccipr::W`](W) writer structure
impl crate::Writable for CCIPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR to value 0
impl crate::Resettable for CCIPRrs {}
