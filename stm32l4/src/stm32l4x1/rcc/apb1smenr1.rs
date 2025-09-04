///Register `APB1SMENR1` reader
pub type R = crate::R<APB1SMENR1rs>;
///Register `APB1SMENR1` writer
pub type W = crate::W<APB1SMENR1rs>;
///Field `TIM2SMEN` reader - TIM2 timer clocks enable during Sleep and Stop modes
pub type TIM2SMEN_R = crate::BitReader;
///Field `TIM2SMEN` writer - TIM2 timer clocks enable during Sleep and Stop modes
pub type TIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6SMEN` reader - TIM6 timer clocks enable during Sleep and Stop modes
pub type TIM6SMEN_R = crate::BitReader;
///Field `TIM6SMEN` writer - TIM6 timer clocks enable during Sleep and Stop modes
pub type TIM6SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7SMEN` reader - TIM7 timer clocks enable during Sleep and Stop modes
pub type TIM7SMEN_R = crate::BitReader;
///Field `TIM7SMEN` writer - TIM7 timer clocks enable during Sleep and Stop modes
pub type TIM7SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCDSMEN` reader - LCD clocks enable during Sleep and Stop modes
pub type LCDSMEN_R = crate::BitReader;
///Field `LCDSMEN` writer - LCD clocks enable during Sleep and Stop modes
pub type LCDSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
///Field `SP3SMEN` reader - SPI3 clocks enable during Sleep and Stop modes
pub type SP3SMEN_R = crate::BitReader;
///Field `SP3SMEN` writer - SPI3 clocks enable during Sleep and Stop modes
pub type SP3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
///Field `I2C1SMEN` reader - I2C1 clocks enable during Sleep and Stop modes
pub type I2C1SMEN_R = crate::BitReader;
///Field `I2C1SMEN` writer - I2C1 clocks enable during Sleep and Stop modes
pub type I2C1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2SMEN` reader - I2C2 clocks enable during Sleep and Stop modes
pub type I2C2SMEN_R = crate::BitReader;
///Field `I2C2SMEN` writer - I2C2 clocks enable during Sleep and Stop modes
pub type I2C2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3SMEN` reader - I2C3 clocks enable during Sleep and Stop modes
pub type I2C3SMEN_R = crate::BitReader;
///Field `I2C3SMEN` writer - I2C3 clocks enable during Sleep and Stop modes
pub type I2C3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRSSMEN` reader - CRS clock enable during Sleep and Stop modes
pub type CRSSMEN_R = crate::BitReader;
///Field `CRSSMEN` writer - CRS clock enable during Sleep and Stop modes
pub type CRSSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAN1SMEN` reader - CAN1 clocks enable during Sleep and Stop modes
pub type CAN1SMEN_R = crate::BitReader;
///Field `CAN1SMEN` writer - CAN1 clocks enable during Sleep and Stop modes
pub type CAN1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBFSSMEN` reader - USB FS clock enable during Sleep and Stop modes
pub type USBFSSMEN_R = crate::BitReader;
///Field `USBFSSMEN` writer - USB FS clock enable during Sleep and Stop modes
pub type USBFSSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRSMEN` reader - Power interface clocks enable during Sleep and Stop modes
pub type PWRSMEN_R = crate::BitReader;
///Field `PWRSMEN` writer - Power interface clocks enable during Sleep and Stop modes
pub type PWRSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC1SMEN` reader - DAC1 interface clocks enable during Sleep and Stop modes
pub type DAC1SMEN_R = crate::BitReader;
///Field `DAC1SMEN` writer - DAC1 interface clocks enable during Sleep and Stop modes
pub type DAC1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPAMPSMEN` reader - OPAMP interface clocks enable during Sleep and Stop modes
pub type OPAMPSMEN_R = crate::BitReader;
///Field `OPAMPSMEN` writer - OPAMP interface clocks enable during Sleep and Stop modes
pub type OPAMPSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 9 - LCD clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn lcdsmen(&self) -> LCDSMEN_R {
        LCDSMEN_R::new(((self.bits >> 9) & 1) != 0)
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
    pub fn sp3smen(&self) -> SP3SMEN_R {
        SP3SMEN_R::new(((self.bits >> 15) & 1) != 0)
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
    ///Bit 23 - I2C3 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CRS clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn crssmen(&self) -> CRSSMEN_R {
        CRSSMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - CAN1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn can1smen(&self) -> CAN1SMEN_R {
        CAN1SMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - USB FS clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn usbfssmen(&self) -> USBFSSMEN_R {
        USBFSSMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - Power interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC1 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dac1smen(&self) -> DAC1SMEN_R {
        DAC1SMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - OPAMP interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn opampsmen(&self) -> OPAMPSMEN_R {
        OPAMPSMEN_R::new(((self.bits >> 30) & 1) != 0)
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
            .field("lptim1smen", &self.lptim1smen())
            .field("opampsmen", &self.opampsmen())
            .field("dac1smen", &self.dac1smen())
            .field("pwrsmen", &self.pwrsmen())
            .field("can1smen", &self.can1smen())
            .field("i2c3smen", &self.i2c3smen())
            .field("i2c1smen", &self.i2c1smen())
            .field("usart2smen", &self.usart2smen())
            .field("sp3smen", &self.sp3smen())
            .field("spi2smen", &self.spi2smen())
            .field("wwdgsmen", &self.wwdgsmen())
            .field("lcdsmen", &self.lcdsmen())
            .field("tim7smen", &self.tim7smen())
            .field("tim6smen", &self.tim6smen())
            .field("tim2smen", &self.tim2smen())
            .field("rtcapbsmen", &self.rtcapbsmen())
            .field("usbfssmen", &self.usbfssmen())
            .field("i2c2smen", &self.i2c2smen())
            .field("crssmen", &self.crssmen())
            .field("usart3smen", &self.usart3smen())
            .field("uart4smen", &self.uart4smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<APB1SMENR1rs> {
        TIM2SMEN_W::new(self, 0)
    }
    ///Bit 4 - TIM6 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim6smen(&mut self) -> TIM6SMEN_W<APB1SMENR1rs> {
        TIM6SMEN_W::new(self, 4)
    }
    ///Bit 5 - TIM7 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim7smen(&mut self) -> TIM7SMEN_W<APB1SMENR1rs> {
        TIM7SMEN_W::new(self, 5)
    }
    ///Bit 9 - LCD clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn lcdsmen(&mut self) -> LCDSMEN_W<APB1SMENR1rs> {
        LCDSMEN_W::new(self, 9)
    }
    ///Bit 10 - RTC APB clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<APB1SMENR1rs> {
        RTCAPBSMEN_W::new(self, 10)
    }
    ///Bit 11 - Window watchdog clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<APB1SMENR1rs> {
        WWDGSMEN_W::new(self, 11)
    }
    ///Bit 14 - SPI2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W<APB1SMENR1rs> {
        SPI2SMEN_W::new(self, 14)
    }
    ///Bit 15 - SPI3 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sp3smen(&mut self) -> SP3SMEN_W<APB1SMENR1rs> {
        SP3SMEN_W::new(self, 15)
    }
    ///Bit 17 - USART2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<APB1SMENR1rs> {
        USART2SMEN_W::new(self, 17)
    }
    ///Bit 18 - USART3 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn usart3smen(&mut self) -> USART3SMEN_W<APB1SMENR1rs> {
        USART3SMEN_W::new(self, 18)
    }
    ///Bit 19 - UART4 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn uart4smen(&mut self) -> UART4SMEN_W<APB1SMENR1rs> {
        UART4SMEN_W::new(self, 19)
    }
    ///Bit 21 - I2C1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<APB1SMENR1rs> {
        I2C1SMEN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W<APB1SMENR1rs> {
        I2C2SMEN_W::new(self, 22)
    }
    ///Bit 23 - I2C3 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W<APB1SMENR1rs> {
        I2C3SMEN_W::new(self, 23)
    }
    ///Bit 24 - CRS clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn crssmen(&mut self) -> CRSSMEN_W<APB1SMENR1rs> {
        CRSSMEN_W::new(self, 24)
    }
    ///Bit 25 - CAN1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn can1smen(&mut self) -> CAN1SMEN_W<APB1SMENR1rs> {
        CAN1SMEN_W::new(self, 25)
    }
    ///Bit 26 - USB FS clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn usbfssmen(&mut self) -> USBFSSMEN_W<APB1SMENR1rs> {
        USBFSSMEN_W::new(self, 26)
    }
    ///Bit 28 - Power interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<APB1SMENR1rs> {
        PWRSMEN_W::new(self, 28)
    }
    ///Bit 29 - DAC1 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dac1smen(&mut self) -> DAC1SMEN_W<APB1SMENR1rs> {
        DAC1SMEN_W::new(self, 29)
    }
    ///Bit 30 - OPAMP interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn opampsmen(&mut self) -> OPAMPSMEN_W<APB1SMENR1rs> {
        OPAMPSMEN_W::new(self, 30)
    }
    ///Bit 31 - Low power timer 1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W<APB1SMENR1rs> {
        LPTIM1SMEN_W::new(self, 31)
    }
}
/**APB1SMENR1

You can [`read`](crate::Reg::read) this register and get [`apb1smenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x1.html#RCC:APB1SMENR1)*/
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
///`reset()` method sets APB1SMENR1 to value 0xf2fe_ca3f
impl crate::Resettable for APB1SMENR1rs {
    const RESET_VALUE: u32 = 0xf2fe_ca3f;
}
