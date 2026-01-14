///Register `APB1SMENR1` reader
pub type R = crate::R<APB1SMENR1rs>;
///Register `APB1SMENR1` writer
pub type W = crate::W<APB1SMENR1rs>;
///Field `TIM2SMEN` reader - TIM2 timer clocks enable during Sleep and Stop modes
pub type TIM2SMEN_R = crate::BitReader;
///Field `TIM2SMEN` writer - TIM2 timer clocks enable during Sleep and Stop modes
pub type TIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3SMEN` reader - TIM3 timer clocks enable during Sleep and Stop modes
pub type TIM3SMEN_R = crate::BitReader;
///Field `TIM3SMEN` writer - TIM3 timer clocks enable during Sleep and Stop modes
pub type TIM3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM4SMEN` reader - TIM4 timer clocks enable during Sleep and Stop modes
pub type TIM4SMEN_R = crate::BitReader;
///Field `TIM4SMEN` writer - TIM4 timer clocks enable during Sleep and Stop modes
pub type TIM4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM5SMEN` reader - TIM5 timer clocks enable during Sleep and Stop modes
pub type TIM5SMEN_R = crate::BitReader;
///Field `TIM5SMEN` writer - TIM5 timer clocks enable during Sleep and Stop modes
pub type TIM5SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6SMEN` reader - TIM6 timer clocks enable during Sleep and Stop modes
pub type TIM6SMEN_R = crate::BitReader;
///Field `TIM6SMEN` writer - TIM6 timer clocks enable during Sleep and Stop modes
pub type TIM6SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7SMEN` reader - TIM7 timer clocks enable during Sleep and Stop modes
pub type TIM7SMEN_R = crate::BitReader;
///Field `TIM7SMEN` writer - TIM7 timer clocks enable during Sleep and Stop modes
pub type TIM7SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRSSMEN` reader - CRS timer clocks enable during Sleep and Stop modes
pub type CRSSMEN_R = crate::BitReader;
///Field `CRSSMEN` writer - CRS timer clocks enable during Sleep and Stop modes
pub type CRSSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep and Stop modes
pub type RTCAPBSMEN_R = crate::BitReader;
///Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep and Stop modes
pub type RTCAPBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGSMEN` reader - Window watchdog clocks enable during Sleep and Stop modes
pub type WWDGSMEN_R = crate::BitReader;
///Field `WWDGSMEN` writer - Window watchdog clocks enable during Sleep and Stop modes
pub type WWDGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2SMEN` reader - SPI2 clocks enable during Sleep and Stop modes
pub type SPI2SMEN_R = crate::BitReader;
///Field `SPI2SMEN` writer - SPI2 clocks enable during Sleep and Stop modes
pub type SPI2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3SMEN` reader - SPI3 clocks enable during Sleep and Stop modes
pub type SPI3SMEN_R = crate::BitReader;
///Field `SPI3SMEN` writer - SPI3 clocks enable during Sleep and Stop modes
pub type SPI3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2SMEN` reader - USART2 clocks enable during Sleep and Stop modes
pub type USART2SMEN_R = crate::BitReader;
///Field `USART2SMEN` writer - USART2 clocks enable during Sleep and Stop modes
pub type USART2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3SMEN` reader - USART3 clocks enable during Sleep and Stop modes
pub type USART3SMEN_R = crate::BitReader;
///Field `USART3SMEN` writer - USART3 clocks enable during Sleep and Stop modes
pub type USART3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART4SMEN` reader - UART4 clocks enable during Sleep and Stop modes
pub type UART4SMEN_R = crate::BitReader;
///Field `UART4SMEN` writer - UART4 clocks enable during Sleep and Stop modes
pub type UART4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART5SMEN` reader - UART5 clocks enable during Sleep and Stop modes
pub type UART5SMEN_R = crate::BitReader;
///Field `UART5SMEN` writer - UART5 clocks enable during Sleep and Stop modes
pub type UART5SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1SMEN` reader - I2C1 clocks enable during Sleep and Stop modes
pub type I2C1SMEN_R = crate::BitReader;
///Field `I2C1SMEN` writer - I2C1 clocks enable during Sleep and Stop modes
pub type I2C1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2SMEN` reader - I2C2 clocks enable during Sleep and Stop modes
pub type I2C2SMEN_R = crate::BitReader;
///Field `I2C2SMEN` writer - I2C2 clocks enable during Sleep and Stop modes
pub type I2C2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBSMEN` reader - USB device clocks enable during Sleep and Stop modes
pub type USBSMEN_R = crate::BitReader;
///Field `USBSMEN` writer - USB device clocks enable during Sleep and Stop modes
pub type USBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDCANSMEN` reader - FDCAN clocks enable during Sleep and Stop modes
pub type FDCANSMEN_R = crate::BitReader;
///Field `FDCANSMEN` writer - FDCAN clocks enable during Sleep and Stop modes
pub type FDCANSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRSMEN` reader - Power interface clocks enable during Sleep and Stop modes
pub type PWRSMEN_R = crate::BitReader;
///Field `PWRSMEN` writer - Power interface clocks enable during Sleep and Stop modes
pub type PWRSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3SMEN` reader - I2C3 clocks enable during Sleep and Stop modes
pub type I2C3SMEN_R = crate::BitReader;
///Field `I2C3SMEN` writer - I2C3 clocks enable during Sleep and Stop modes
pub type I2C3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1SMEN` reader - Low power timer 1 clocks enable during Sleep and Stop modes
pub type LPTIM1SMEN_R = crate::BitReader;
///Field `LPTIM1SMEN` writer - Low power timer 1 clocks enable during Sleep and Stop modes
pub type LPTIM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM2 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim4smen(&self) -> TIM4SMEN_R {
        TIM4SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim5smen(&self) -> TIM5SMEN_R {
        TIM5SMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim6smen(&self) -> TIM6SMEN_R {
        TIM6SMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim7smen(&self) -> TIM7SMEN_R {
        TIM7SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - CRS timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn crssmen(&self) -> CRSSMEN_R {
        CRSSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - RTC APB clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Window watchdog clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI3 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn spi3smen(&self) -> SPI3SMEN_R {
        SPI3SMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - USART2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn usart3smen(&self) -> USART3SMEN_R {
        USART3SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART4 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn uart4smen(&self) -> UART4SMEN_R {
        UART4SMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - UART5 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn uart5smen(&self) -> UART5SMEN_R {
        UART5SMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - USB device clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn usbsmen(&self) -> USBSMEN_R {
        USBSMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - FDCAN clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn fdcansmen(&self) -> FDCANSMEN_R {
        FDCANSMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 28 - Power interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - I2C3 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low power timer 1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1SMENR1")
            .field("tim2smen", &self.tim2smen())
            .field("tim3smen", &self.tim3smen())
            .field("tim4smen", &self.tim4smen())
            .field("tim5smen", &self.tim5smen())
            .field("tim6smen", &self.tim6smen())
            .field("tim7smen", &self.tim7smen())
            .field("crssmen", &self.crssmen())
            .field("rtcapbsmen", &self.rtcapbsmen())
            .field("wwdgsmen", &self.wwdgsmen())
            .field("spi2smen", &self.spi2smen())
            .field("spi3smen", &self.spi3smen())
            .field("usart2smen", &self.usart2smen())
            .field("usart3smen", &self.usart3smen())
            .field("uart4smen", &self.uart4smen())
            .field("uart5smen", &self.uart5smen())
            .field("i2c1smen", &self.i2c1smen())
            .field("i2c2smen", &self.i2c2smen())
            .field("usbsmen", &self.usbsmen())
            .field("fdcansmen", &self.fdcansmen())
            .field("pwrsmen", &self.pwrsmen())
            .field("i2c3smen", &self.i2c3smen())
            .field("lptim1smen", &self.lptim1smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<'_, APB1SMENR1rs> {
        TIM2SMEN_W::new(self, 0)
    }
    ///Bit 1 - TIM3 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W<'_, APB1SMENR1rs> {
        TIM3SMEN_W::new(self, 1)
    }
    ///Bit 2 - TIM4 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim4smen(&mut self) -> TIM4SMEN_W<'_, APB1SMENR1rs> {
        TIM4SMEN_W::new(self, 2)
    }
    ///Bit 3 - TIM5 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim5smen(&mut self) -> TIM5SMEN_W<'_, APB1SMENR1rs> {
        TIM5SMEN_W::new(self, 3)
    }
    ///Bit 4 - TIM6 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim6smen(&mut self) -> TIM6SMEN_W<'_, APB1SMENR1rs> {
        TIM6SMEN_W::new(self, 4)
    }
    ///Bit 5 - TIM7 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim7smen(&mut self) -> TIM7SMEN_W<'_, APB1SMENR1rs> {
        TIM7SMEN_W::new(self, 5)
    }
    ///Bit 8 - CRS timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn crssmen(&mut self) -> CRSSMEN_W<'_, APB1SMENR1rs> {
        CRSSMEN_W::new(self, 8)
    }
    ///Bit 10 - RTC APB clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<'_, APB1SMENR1rs> {
        RTCAPBSMEN_W::new(self, 10)
    }
    ///Bit 11 - Window watchdog clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<'_, APB1SMENR1rs> {
        WWDGSMEN_W::new(self, 11)
    }
    ///Bit 14 - SPI2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W<'_, APB1SMENR1rs> {
        SPI2SMEN_W::new(self, 14)
    }
    ///Bit 15 - SPI3 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn spi3smen(&mut self) -> SPI3SMEN_W<'_, APB1SMENR1rs> {
        SPI3SMEN_W::new(self, 15)
    }
    ///Bit 17 - USART2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<'_, APB1SMENR1rs> {
        USART2SMEN_W::new(self, 17)
    }
    ///Bit 18 - USART3 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn usart3smen(&mut self) -> USART3SMEN_W<'_, APB1SMENR1rs> {
        USART3SMEN_W::new(self, 18)
    }
    ///Bit 19 - UART4 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn uart4smen(&mut self) -> UART4SMEN_W<'_, APB1SMENR1rs> {
        UART4SMEN_W::new(self, 19)
    }
    ///Bit 20 - UART5 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn uart5smen(&mut self) -> UART5SMEN_W<'_, APB1SMENR1rs> {
        UART5SMEN_W::new(self, 20)
    }
    ///Bit 21 - I2C1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<'_, APB1SMENR1rs> {
        I2C1SMEN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W<'_, APB1SMENR1rs> {
        I2C2SMEN_W::new(self, 22)
    }
    ///Bit 23 - USB device clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn usbsmen(&mut self) -> USBSMEN_W<'_, APB1SMENR1rs> {
        USBSMEN_W::new(self, 23)
    }
    ///Bit 25 - FDCAN clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn fdcansmen(&mut self) -> FDCANSMEN_W<'_, APB1SMENR1rs> {
        FDCANSMEN_W::new(self, 25)
    }
    ///Bit 28 - Power interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<'_, APB1SMENR1rs> {
        PWRSMEN_W::new(self, 28)
    }
    ///Bit 30 - I2C3 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W<'_, APB1SMENR1rs> {
        I2C3SMEN_W::new(self, 30)
    }
    ///Bit 31 - Low power timer 1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W<'_, APB1SMENR1rs> {
        LPTIM1SMEN_W::new(self, 31)
    }
}
/**APB1 peripheral clocks enable in Sleep and Stop modes register 1

You can [`read`](crate::Reg::read) this register and get [`apb1smenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473.html#RCC:APB1SMENR1)*/
pub struct APB1SMENR1rs;
impl crate::RegisterSpec for APB1SMENR1rs {
    type Ux = u32;
}
///`read()` method returns [`apb1smenr1::R`](R) reader structure
impl crate::Readable for APB1SMENR1rs {}
///`write(|w| ..)` method takes [`apb1smenr1::W`](W) writer structure
impl crate::Writable for APB1SMENR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1SMENR1 to value 0xd2fe_cd3f
impl crate::Resettable for APB1SMENR1rs {
    const RESET_VALUE: u32 = 0xd2fe_cd3f;
}
