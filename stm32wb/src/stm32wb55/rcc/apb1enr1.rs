///Register `APB1ENR1` reader
pub type R = crate::R<APB1ENR1rs>;
///Register `APB1ENR1` writer
pub type W = crate::W<APB1ENR1rs>;
///Field `TIM2EN` reader - CPU1 TIM2 timer clock enable
pub type TIM2EN_R = crate::BitReader;
///Field `TIM2EN` writer - CPU1 TIM2 timer clock enable
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCDEN` reader - CPU1 LCD clock enable
pub type LCDEN_R = crate::BitReader;
///Field `LCDEN` writer - CPU1 LCD clock enable
pub type LCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCAPBEN` reader - CPU1 RTC APB clock enable
pub type RTCAPBEN_R = crate::BitReader;
///Field `RTCAPBEN` writer - CPU1 RTC APB clock enable
pub type RTCAPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGEN` reader - CPU1 Window watchdog clock enable
pub type WWDGEN_R = crate::BitReader;
///Field `WWDGEN` writer - CPU1 Window watchdog clock enable
pub type WWDGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2EN` reader - CPU1 SPI2 clock enable
pub type SPI2EN_R = crate::BitReader;
///Field `SPI2EN` writer - CPU1 SPI2 clock enable
pub type SPI2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1EN` reader - CPU1 I2C1 clock enable
pub type I2C1EN_R = crate::BitReader;
///Field `I2C1EN` writer - CPU1 I2C1 clock enable
pub type I2C1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3EN` reader - CPU1 I2C3 clock enable
pub type I2C3EN_R = crate::BitReader;
///Field `I2C3EN` writer - CPU1 I2C3 clock enable
pub type I2C3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRSEN` reader - CPU1 CRS clock enable
pub type CRSEN_R = crate::BitReader;
///Field `CRSEN` writer - CPU1 CRS clock enable
pub type CRSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBEN` reader - CPU1 USB clock enable
pub type USBEN_R = crate::BitReader;
///Field `USBEN` writer - CPU1 USB clock enable
pub type USBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1EN` reader - CPU1 Low power timer 1 clock enable
pub type LPTIM1EN_R = crate::BitReader;
///Field `LPTIM1EN` writer - CPU1 Low power timer 1 clock enable
pub type LPTIM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CPU1 TIM2 timer clock enable
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 9 - CPU1 LCD clock enable
    #[inline(always)]
    pub fn lcden(&self) -> LCDEN_R {
        LCDEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CPU1 RTC APB clock enable
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CPU1 Window watchdog clock enable
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - CPU1 SPI2 clock enable
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 21 - CPU1 I2C1 clock enable
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - CPU1 I2C3 clock enable
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CPU1 CRS clock enable
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - CPU1 USB clock enable
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 31 - CPU1 Low power timer 1 clock enable
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1ENR1")
            .field("lptim1en", &self.lptim1en())
            .field("usben", &self.usben())
            .field("crsen", &self.crsen())
            .field("i2c3en", &self.i2c3en())
            .field("i2c1en", &self.i2c1en())
            .field("spi2en", &self.spi2en())
            .field("wwdgen", &self.wwdgen())
            .field("rtcapben", &self.rtcapben())
            .field("lcden", &self.lcden())
            .field("tim2en", &self.tim2en())
            .finish()
    }
}
impl W {
    ///Bit 0 - CPU1 TIM2 timer clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> TIM2EN_W<APB1ENR1rs> {
        TIM2EN_W::new(self, 0)
    }
    ///Bit 9 - CPU1 LCD clock enable
    #[inline(always)]
    #[must_use]
    pub fn lcden(&mut self) -> LCDEN_W<APB1ENR1rs> {
        LCDEN_W::new(self, 9)
    }
    ///Bit 10 - CPU1 RTC APB clock enable
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<APB1ENR1rs> {
        RTCAPBEN_W::new(self, 10)
    }
    ///Bit 11 - CPU1 Window watchdog clock enable
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<APB1ENR1rs> {
        WWDGEN_W::new(self, 11)
    }
    ///Bit 14 - CPU1 SPI2 clock enable
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<APB1ENR1rs> {
        SPI2EN_W::new(self, 14)
    }
    ///Bit 21 - CPU1 I2C1 clock enable
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<APB1ENR1rs> {
        I2C1EN_W::new(self, 21)
    }
    ///Bit 23 - CPU1 I2C3 clock enable
    #[inline(always)]
    #[must_use]
    pub fn i2c3en(&mut self) -> I2C3EN_W<APB1ENR1rs> {
        I2C3EN_W::new(self, 23)
    }
    ///Bit 24 - CPU1 CRS clock enable
    #[inline(always)]
    #[must_use]
    pub fn crsen(&mut self) -> CRSEN_W<APB1ENR1rs> {
        CRSEN_W::new(self, 24)
    }
    ///Bit 26 - CPU1 USB clock enable
    #[inline(always)]
    #[must_use]
    pub fn usben(&mut self) -> USBEN_W<APB1ENR1rs> {
        USBEN_W::new(self, 26)
    }
    ///Bit 31 - CPU1 Low power timer 1 clock enable
    #[inline(always)]
    #[must_use]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<APB1ENR1rs> {
        LPTIM1EN_W::new(self, 31)
    }
}
/**APB1ENR1

You can [`read`](crate::Reg::read) this register and get [`apb1enr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:APB1ENR1)*/
pub struct APB1ENR1rs;
impl crate::RegisterSpec for APB1ENR1rs {
    type Ux = u32;
}
///`read()` method returns [`apb1enr1::R`](R) reader structure
impl crate::Readable for APB1ENR1rs {}
///`write(|w| ..)` method takes [`apb1enr1::W`](W) writer structure
impl crate::Writable for APB1ENR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APB1ENR1 to value 0x0400
impl crate::Resettable for APB1ENR1rs {
    const RESET_VALUE: u32 = 0x0400;
}
