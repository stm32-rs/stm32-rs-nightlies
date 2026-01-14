///Register `APB1SMENR2` reader
pub type R = crate::R<APB1SMENR2rs>;
///Register `APB1SMENR2` writer
pub type W = crate::W<APB1SMENR2rs>;
///Field `LPUART1SMEN` reader - Low power UART 1 clocks enable during Sleep and Stop modes
pub type LPUART1SMEN_R = crate::BitReader;
///Field `LPUART1SMEN` writer - Low power UART 1 clocks enable during Sleep and Stop modes
pub type LPUART1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4SMEN` reader - I2C4 clocks enable during Sleep and Stop modes
pub type I2C4SMEN_R = crate::BitReader;
///Field `I2C4SMEN` writer - I2C4 clocks enable during Sleep and Stop modes
pub type I2C4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2SMEN` reader - LPTIM2SMEN
pub type LPTIM2SMEN_R = crate::BitReader;
///Field `LPTIM2SMEN` writer - LPTIM2SMEN
pub type LPTIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3SMEN` reader - LPTIM3SMEN
pub type LPTIM3SMEN_R = crate::BitReader;
///Field `LPTIM3SMEN` writer - LPTIM3SMEN
pub type LPTIM3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDCAN1SMEN` reader - FDCAN1SMEN
pub type FDCAN1SMEN_R = crate::BitReader;
///Field `FDCAN1SMEN` writer - FDCAN1SMEN
pub type FDCAN1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBFSSMEN` reader - USBFSSMEN
pub type USBFSSMEN_R = crate::BitReader;
///Field `USBFSSMEN` writer - USBFSSMEN
pub type USBFSSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UCPD1SMEN` reader - UCPD1SMEN
pub type UCPD1SMEN_R = crate::BitReader;
///Field `UCPD1SMEN` writer - UCPD1SMEN
pub type UCPD1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I2C4 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c4smen(&self) -> I2C4SMEN_R {
        I2C4SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - LPTIM2SMEN
    #[inline(always)]
    pub fn lptim2smen(&self) -> LPTIM2SMEN_R {
        LPTIM2SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LPTIM3SMEN
    #[inline(always)]
    pub fn lptim3smen(&self) -> LPTIM3SMEN_R {
        LPTIM3SMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - FDCAN1SMEN
    #[inline(always)]
    pub fn fdcan1smen(&self) -> FDCAN1SMEN_R {
        FDCAN1SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 21 - USBFSSMEN
    #[inline(always)]
    pub fn usbfssmen(&self) -> USBFSSMEN_R {
        USBFSSMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - UCPD1SMEN
    #[inline(always)]
    pub fn ucpd1smen(&self) -> UCPD1SMEN_R {
        UCPD1SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1SMENR2")
            .field("lpuart1smen", &self.lpuart1smen())
            .field("i2c4smen", &self.i2c4smen())
            .field("lptim2smen", &self.lptim2smen())
            .field("lptim3smen", &self.lptim3smen())
            .field("fdcan1smen", &self.fdcan1smen())
            .field("usbfssmen", &self.usbfssmen())
            .field("ucpd1smen", &self.ucpd1smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W<'_, APB1SMENR2rs> {
        LPUART1SMEN_W::new(self, 0)
    }
    ///Bit 1 - I2C4 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn i2c4smen(&mut self) -> I2C4SMEN_W<'_, APB1SMENR2rs> {
        I2C4SMEN_W::new(self, 1)
    }
    ///Bit 5 - LPTIM2SMEN
    #[inline(always)]
    pub fn lptim2smen(&mut self) -> LPTIM2SMEN_W<'_, APB1SMENR2rs> {
        LPTIM2SMEN_W::new(self, 5)
    }
    ///Bit 6 - LPTIM3SMEN
    #[inline(always)]
    pub fn lptim3smen(&mut self) -> LPTIM3SMEN_W<'_, APB1SMENR2rs> {
        LPTIM3SMEN_W::new(self, 6)
    }
    ///Bit 9 - FDCAN1SMEN
    #[inline(always)]
    pub fn fdcan1smen(&mut self) -> FDCAN1SMEN_W<'_, APB1SMENR2rs> {
        FDCAN1SMEN_W::new(self, 9)
    }
    ///Bit 21 - USBFSSMEN
    #[inline(always)]
    pub fn usbfssmen(&mut self) -> USBFSSMEN_W<'_, APB1SMENR2rs> {
        USBFSSMEN_W::new(self, 21)
    }
    ///Bit 23 - UCPD1SMEN
    #[inline(always)]
    pub fn ucpd1smen(&mut self) -> UCPD1SMEN_W<'_, APB1SMENR2rs> {
        UCPD1SMEN_W::new(self, 23)
    }
}
/**APB1 peripheral clocks enable in Sleep and Stop modes register 2

You can [`read`](crate::Reg::read) this register and get [`apb1smenr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:APB1SMENR2)*/
pub struct APB1SMENR2rs;
impl crate::RegisterSpec for APB1SMENR2rs {
    type Ux = u32;
}
///`read()` method returns [`apb1smenr2::R`](R) reader structure
impl crate::Readable for APB1SMENR2rs {}
///`write(|w| ..)` method takes [`apb1smenr2::W`](W) writer structure
impl crate::Writable for APB1SMENR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1SMENR2 to value 0x00a0_0223
impl crate::Resettable for APB1SMENR2rs {
    const RESET_VALUE: u32 = 0x00a0_0223;
}
