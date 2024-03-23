#[doc = "Register `APB1SMENR1` reader"]
pub type R = crate::R<APB1SMENR1rs>;
#[doc = "Register `APB1SMENR1` writer"]
pub type W = crate::W<APB1SMENR1rs>;
#[doc = "Field `TIM2SMEN` reader - TIM2 timer clocks enable during CPU1 Sleep mode"]
pub type TIM2SMEN_R = crate::BitReader;
#[doc = "Field `TIM2SMEN` writer - TIM2 timer clocks enable during CPU1 Sleep mode"]
pub type TIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDSMEN` reader - LCD clocks enable during CPU1 Sleep mode"]
pub type LCDSMEN_R = crate::BitReader;
#[doc = "Field `LCDSMEN` writer - LCD clocks enable during CPU1 Sleep mode"]
pub type LCDSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAPBSMEN` reader - RTC APB clocks enable during CPU1 Sleep mode"]
pub type RTCAPBSMEN_R = crate::BitReader;
#[doc = "Field `RTCAPBSMEN` writer - RTC APB clocks enable during CPU1 Sleep mode"]
pub type RTCAPBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGSMEN` reader - Window watchdog clocks enable during CPU1 Sleep mode"]
pub type WWDGSMEN_R = crate::BitReader;
#[doc = "Field `WWDGSMEN` writer - Window watchdog clocks enable during CPU1 Sleep mode"]
pub type WWDGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2SMEN` reader - SPI2 clocks enable during CPU1 Sleep mode"]
pub type SPI2SMEN_R = crate::BitReader;
#[doc = "Field `SPI2SMEN` writer - SPI2 clocks enable during CPU1 Sleep mode"]
pub type SPI2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1SMEN` reader - I2C1 clocks enable during CPU1 Sleep mode"]
pub type I2C1SMEN_R = crate::BitReader;
#[doc = "Field `I2C1SMEN` writer - I2C1 clocks enable during CPU1 Sleep mode"]
pub type I2C1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3SMEN` reader - I2C3 clocks enable during CPU1 Sleep mode"]
pub type I2C3SMEN_R = crate::BitReader;
#[doc = "Field `I2C3SMEN` writer - I2C3 clocks enable during CPU1 Sleep mode"]
pub type I2C3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRSMEN` reader - CRS clocks enable during CPU1 Sleep mode"]
pub type CRSMEN_R = crate::BitReader;
#[doc = "Field `CRSMEN` writer - CRS clocks enable during CPU1 Sleep mode"]
pub type CRSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSMEN` reader - USB FS clocks enable during CPU1 Sleep mode"]
pub type USBSMEN_R = crate::BitReader;
#[doc = "Field `USBSMEN` writer - USB FS clocks enable during CPU1 Sleep mode"]
pub type USBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1SMEN` reader - Low power timer 1 clocks enable during CPU1 Sleep mode"]
pub type LPTIM1SMEN_R = crate::BitReader;
#[doc = "Field `LPTIM1SMEN` writer - Low power timer 1 clocks enable during CPU1 Sleep mode"]
pub type LPTIM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2 timer clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - LCD clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn lcdsmen(&self) -> LCDSMEN_R {
        LCDSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CRS clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn crsmen(&self) -> CRSMEN_R {
        CRSMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - USB FS clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn usbsmen(&self) -> USBSMEN_R {
        USBSMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 31 - Low power timer 1 clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<APB1SMENR1rs> {
        TIM2SMEN_W::new(self, 0)
    }
    #[doc = "Bit 9 - LCD clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn lcdsmen(&mut self) -> LCDSMEN_W<APB1SMENR1rs> {
        LCDSMEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - RTC APB clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<APB1SMENR1rs> {
        RTCAPBSMEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Window watchdog clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<APB1SMENR1rs> {
        WWDGSMEN_W::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W<APB1SMENR1rs> {
        SPI2SMEN_W::new(self, 14)
    }
    #[doc = "Bit 21 - I2C1 clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<APB1SMENR1rs> {
        I2C1SMEN_W::new(self, 21)
    }
    #[doc = "Bit 23 - I2C3 clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W<APB1SMENR1rs> {
        I2C3SMEN_W::new(self, 23)
    }
    #[doc = "Bit 24 - CRS clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn crsmen(&mut self) -> CRSMEN_W<APB1SMENR1rs> {
        CRSMEN_W::new(self, 24)
    }
    #[doc = "Bit 26 - USB FS clocks enable during CPU1 Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usbsmen(&mut self) -> USBSMEN_W<APB1SMENR1rs> {
        USBSMEN_W::new(self, 26)
    }
    #[doc = "Bit 31 - Low power timer 1 clocks enable during CPU1 Sleep mode"]
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
#[doc = "`reset()` method sets APB1SMENR1 to value 0x85a0_4e01"]
impl crate::Resettable for APB1SMENR1rs {
    const RESET_VALUE: u32 = 0x85a0_4e01;
}
