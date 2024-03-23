#[doc = "Register `APB1SMENR1` reader"]
pub type R = crate::R<APB1SMENR1rs>;
#[doc = "Register `APB1SMENR1` writer"]
pub type W = crate::W<APB1SMENR1rs>;
#[doc = "Field `TIM2SMEN` reader - TIM2 timer clocks enable during Sleep and Stop modes"]
pub type TIM2SMEN_R = crate::BitReader;
#[doc = "Field `TIM2SMEN` writer - TIM2 timer clocks enable during Sleep and Stop modes"]
pub type TIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3SMEN` reader - TIM3 timer clocks enable during Sleep and Stop modes"]
pub type TIM3SMEN_R = crate::BitReader;
#[doc = "Field `TIM3SMEN` writer - TIM3 timer clocks enable during Sleep and Stop modes"]
pub type TIM3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM4SMEN` reader - TIM4 timer clocks enable during Sleep and Stop modes"]
pub type TIM4SMEN_R = crate::BitReader;
#[doc = "Field `TIM4SMEN` writer - TIM4 timer clocks enable during Sleep and Stop modes"]
pub type TIM4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM5SMEN` reader - TIM5 timer clocks enable during Sleep and Stop modes"]
pub type TIM5SMEN_R = crate::BitReader;
#[doc = "Field `TIM5SMEN` writer - TIM5 timer clocks enable during Sleep and Stop modes"]
pub type TIM5SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6SMEN` reader - TIM6 timer clocks enable during Sleep and Stop modes"]
pub type TIM6SMEN_R = crate::BitReader;
#[doc = "Field `TIM6SMEN` writer - TIM6 timer clocks enable during Sleep and Stop modes"]
pub type TIM6SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM7SMEN` reader - TIM7 timer clocks enable during Sleep and Stop modes"]
pub type TIM7SMEN_R = crate::BitReader;
#[doc = "Field `TIM7SMEN` writer - TIM7 timer clocks enable during Sleep and Stop modes"]
pub type TIM7SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDSMEN` reader - LCD clocks enable during Sleep and Stop modes"]
pub type LCDSMEN_R = crate::BitReader;
#[doc = "Field `LCDSMEN` writer - LCD clocks enable during Sleep and Stop modes"]
pub type LCDSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep and Stop modes"]
pub type RTCAPBSMEN_R = crate::BitReader;
#[doc = "Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep and Stop modes"]
pub type RTCAPBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGSMEN` reader - Window watchdog clocks enable during Sleep and Stop modes"]
pub type WWDGSMEN_R = crate::BitReader;
#[doc = "Field `WWDGSMEN` writer - Window watchdog clocks enable during Sleep and Stop modes"]
pub type WWDGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2SMEN` reader - SPI2 clocks enable during Sleep and Stop modes"]
pub type SPI2SMEN_R = crate::BitReader;
#[doc = "Field `SPI2SMEN` writer - SPI2 clocks enable during Sleep and Stop modes"]
pub type SPI2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP3SMEN` reader - SPI3 clocks enable during Sleep and Stop modes"]
pub type SP3SMEN_R = crate::BitReader;
#[doc = "Field `SP3SMEN` writer - SPI3 clocks enable during Sleep and Stop modes"]
pub type SP3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2SMEN` reader - USART2 clocks enable during Sleep and Stop modes"]
pub type USART2SMEN_R = crate::BitReader;
#[doc = "Field `USART2SMEN` writer - USART2 clocks enable during Sleep and Stop modes"]
pub type USART2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3SMEN` reader - USART3 clocks enable during Sleep and Stop modes"]
pub type USART3SMEN_R = crate::BitReader;
#[doc = "Field `USART3SMEN` writer - USART3 clocks enable during Sleep and Stop modes"]
pub type USART3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART4SMEN` reader - UART4 clocks enable during Sleep and Stop modes"]
pub type UART4SMEN_R = crate::BitReader;
#[doc = "Field `UART4SMEN` writer - UART4 clocks enable during Sleep and Stop modes"]
pub type UART4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5SMEN` reader - UART5 clocks enable during Sleep and Stop modes"]
pub type UART5SMEN_R = crate::BitReader;
#[doc = "Field `UART5SMEN` writer - UART5 clocks enable during Sleep and Stop modes"]
pub type UART5SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1SMEN` reader - I2C1 clocks enable during Sleep and Stop modes"]
pub type I2C1SMEN_R = crate::BitReader;
#[doc = "Field `I2C1SMEN` writer - I2C1 clocks enable during Sleep and Stop modes"]
pub type I2C1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2SMEN` reader - I2C2 clocks enable during Sleep and Stop modes"]
pub type I2C2SMEN_R = crate::BitReader;
#[doc = "Field `I2C2SMEN` writer - I2C2 clocks enable during Sleep and Stop modes"]
pub type I2C2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3SMEN` reader - I2C3 clocks enable during Sleep and Stop modes"]
pub type I2C3SMEN_R = crate::BitReader;
#[doc = "Field `I2C3SMEN` writer - I2C3 clocks enable during Sleep and Stop modes"]
pub type I2C3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1SMEN` reader - CAN1 clocks enable during Sleep and Stop modes"]
pub type CAN1SMEN_R = crate::BitReader;
#[doc = "Field `CAN1SMEN` writer - CAN1 clocks enable during Sleep and Stop modes"]
pub type CAN1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSMEN` reader - Power interface clocks enable during Sleep and Stop modes"]
pub type PWRSMEN_R = crate::BitReader;
#[doc = "Field `PWRSMEN` writer - Power interface clocks enable during Sleep and Stop modes"]
pub type PWRSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC1SMEN` reader - DAC1 interface clocks enable during Sleep and Stop modes"]
pub type DAC1SMEN_R = crate::BitReader;
#[doc = "Field `DAC1SMEN` writer - DAC1 interface clocks enable during Sleep and Stop modes"]
pub type DAC1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAMPSMEN` reader - OPAMP interface clocks enable during Sleep and Stop modes"]
pub type OPAMPSMEN_R = crate::BitReader;
#[doc = "Field `OPAMPSMEN` writer - OPAMP interface clocks enable during Sleep and Stop modes"]
pub type OPAMPSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1SMEN` reader - Low power timer 1 clocks enable during Sleep and Stop modes"]
pub type LPTIM1SMEN_R = crate::BitReader;
#[doc = "Field `LPTIM1SMEN` writer - Low power timer 1 clocks enable during Sleep and Stop modes"]
pub type LPTIM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim4smen(&self) -> TIM4SMEN_R {
        TIM4SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim5smen(&self) -> TIM5SMEN_R {
        TIM5SMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim6smen(&self) -> TIM6SMEN_R {
        TIM6SMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tim7smen(&self) -> TIM7SMEN_R {
        TIM7SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn lcdsmen(&self) -> LCDSMEN_R {
        LCDSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sp3smen(&self) -> SP3SMEN_R {
        SP3SMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn usart3smen(&self) -> USART3SMEN_R {
        USART3SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn uart4smen(&self) -> UART4SMEN_R {
        UART4SMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn uart5smen(&self) -> UART5SMEN_R {
        UART5SMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn can1smen(&self) -> CAN1SMEN_R {
        CAN1SMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dac1smen(&self) -> DAC1SMEN_R {
        DAC1SMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OPAMP interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn opampsmen(&self) -> OPAMPSMEN_R {
        OPAMPSMEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low power timer 1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<APB1SMENR1rs> {
        TIM2SMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W<APB1SMENR1rs> {
        TIM3SMEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim4smen(&mut self) -> TIM4SMEN_W<APB1SMENR1rs> {
        TIM4SMEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim5smen(&mut self) -> TIM5SMEN_W<APB1SMENR1rs> {
        TIM5SMEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - TIM6 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim6smen(&mut self) -> TIM6SMEN_W<APB1SMENR1rs> {
        TIM6SMEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 timer clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn tim7smen(&mut self) -> TIM7SMEN_W<APB1SMENR1rs> {
        TIM7SMEN_W::new(self, 5)
    }
    #[doc = "Bit 9 - LCD clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn lcdsmen(&mut self) -> LCDSMEN_W<APB1SMENR1rs> {
        LCDSMEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<APB1SMENR1rs> {
        RTCAPBSMEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Window watchdog clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<APB1SMENR1rs> {
        WWDGSMEN_W::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W<APB1SMENR1rs> {
        SPI2SMEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn sp3smen(&mut self) -> SP3SMEN_W<APB1SMENR1rs> {
        SP3SMEN_W::new(self, 15)
    }
    #[doc = "Bit 17 - USART2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<APB1SMENR1rs> {
        USART2SMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn usart3smen(&mut self) -> USART3SMEN_W<APB1SMENR1rs> {
        USART3SMEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - UART4 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn uart4smen(&mut self) -> UART4SMEN_W<APB1SMENR1rs> {
        UART4SMEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - UART5 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn uart5smen(&mut self) -> UART5SMEN_W<APB1SMENR1rs> {
        UART5SMEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<APB1SMENR1rs> {
        I2C1SMEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W<APB1SMENR1rs> {
        I2C2SMEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W<APB1SMENR1rs> {
        I2C3SMEN_W::new(self, 23)
    }
    #[doc = "Bit 25 - CAN1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn can1smen(&mut self) -> CAN1SMEN_W<APB1SMENR1rs> {
        CAN1SMEN_W::new(self, 25)
    }
    #[doc = "Bit 28 - Power interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<APB1SMENR1rs> {
        PWRSMEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC1 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn dac1smen(&mut self) -> DAC1SMEN_W<APB1SMENR1rs> {
        DAC1SMEN_W::new(self, 29)
    }
    #[doc = "Bit 30 - OPAMP interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn opampsmen(&mut self) -> OPAMPSMEN_W<APB1SMENR1rs> {
        OPAMPSMEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Low power timer 1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W<APB1SMENR1rs> {
        LPTIM1SMEN_W::new(self, 31)
    }
}
#[doc = "APB1SMENR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1smenr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1smenr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1SMENR1rs;
impl crate::RegisterSpec for APB1SMENR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1smenr1::R`](R) reader structure"]
impl crate::Readable for APB1SMENR1rs {}
#[doc = "`write(|w| ..)` method takes [`apb1smenr1::W`](W) writer structure"]
impl crate::Writable for APB1SMENR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1SMENR1 to value 0xf2fe_ca3f"]
impl crate::Resettable for APB1SMENR1rs {
    const RESET_VALUE: u32 = 0xf2fe_ca3f;
}
