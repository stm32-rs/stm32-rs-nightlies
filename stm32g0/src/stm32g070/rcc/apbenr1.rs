///Register `APBENR1` reader
pub type R = crate::R<APBENR1rs>;
///Register `APBENR1` writer
pub type W = crate::W<APBENR1rs>;
///Field `TIM3EN` reader - TIM3 timer clock enable
pub type TIM3EN_R = crate::BitReader;
///Field `TIM3EN` writer - TIM3 timer clock enable
pub type TIM3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6EN` reader - TIM6 timer clock enable
pub type TIM6EN_R = crate::BitReader;
///Field `TIM6EN` writer - TIM6 timer clock enable
pub type TIM6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7EN` reader - TIM7 timer clock enable
pub type TIM7EN_R = crate::BitReader;
///Field `TIM7EN` writer - TIM7 timer clock enable
pub type TIM7EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCAPBEN` reader - RTC APB clock enable
pub type RTCAPBEN_R = crate::BitReader;
///Field `RTCAPBEN` writer - RTC APB clock enable
pub type RTCAPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGEN` reader - WWDG clock enable
pub type WWDGEN_R = crate::BitReader;
///Field `WWDGEN` writer - WWDG clock enable
pub type WWDGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2EN` reader - SPI2 clock enable
pub type SPI2EN_R = crate::BitReader;
///Field `SPI2EN` writer - SPI2 clock enable
pub type SPI2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2EN` reader - USART2 clock enable
pub type USART2EN_R = crate::BitReader;
///Field `USART2EN` writer - USART2 clock enable
pub type USART2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3EN` reader - USART3 clock enable
pub type USART3EN_R = crate::BitReader;
///Field `USART3EN` writer - USART3 clock enable
pub type USART3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART4EN` reader - USART4 clock enable
pub type USART4EN_R = crate::BitReader;
///Field `USART4EN` writer - USART4 clock enable
pub type USART4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1EN` reader - I2C1 clock enable
pub type I2C1EN_R = crate::BitReader;
///Field `I2C1EN` writer - I2C1 clock enable
pub type I2C1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2EN` reader - I2C2 clock enable
pub type I2C2EN_R = crate::BitReader;
///Field `I2C2EN` writer - I2C2 clock enable
pub type I2C2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGEN` reader - Debug support clock enable
pub type DBGEN_R = crate::BitReader;
///Field `DBGEN` writer - Debug support clock enable
pub type DBGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWREN` reader - Power interface clock enable
pub type PWREN_R = crate::BitReader;
///Field `PWREN` writer - Power interface clock enable
pub type PWREN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - TIM3 timer clock enable
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - TIM6 timer clock enable
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 timer clock enable
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 10 - RTC APB clock enable
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - WWDG clock enable
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2 clock enable
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - USART2 clock enable
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 clock enable
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - USART4 clock enable
    #[inline(always)]
    pub fn usart4en(&self) -> USART4EN_R {
        USART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clock enable
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 27 - Debug support clock enable
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Power interface clock enable
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APBENR1")
            .field("tim3en", &self.tim3en())
            .field("tim6en", &self.tim6en())
            .field("tim7en", &self.tim7en())
            .field("rtcapben", &self.rtcapben())
            .field("wwdgen", &self.wwdgen())
            .field("spi2en", &self.spi2en())
            .field("usart2en", &self.usart2en())
            .field("usart3en", &self.usart3en())
            .field("usart4en", &self.usart4en())
            .field("i2c1en", &self.i2c1en())
            .field("i2c2en", &self.i2c2en())
            .field("dbgen", &self.dbgen())
            .field("pwren", &self.pwren())
            .finish()
    }
}
impl W {
    ///Bit 1 - TIM3 timer clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim3en(&mut self) -> TIM3EN_W<APBENR1rs> {
        TIM3EN_W::new(self, 1)
    }
    ///Bit 4 - TIM6 timer clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim6en(&mut self) -> TIM6EN_W<APBENR1rs> {
        TIM6EN_W::new(self, 4)
    }
    ///Bit 5 - TIM7 timer clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim7en(&mut self) -> TIM7EN_W<APBENR1rs> {
        TIM7EN_W::new(self, 5)
    }
    ///Bit 10 - RTC APB clock enable
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<APBENR1rs> {
        RTCAPBEN_W::new(self, 10)
    }
    ///Bit 11 - WWDG clock enable
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<APBENR1rs> {
        WWDGEN_W::new(self, 11)
    }
    ///Bit 14 - SPI2 clock enable
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<APBENR1rs> {
        SPI2EN_W::new(self, 14)
    }
    ///Bit 17 - USART2 clock enable
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<APBENR1rs> {
        USART2EN_W::new(self, 17)
    }
    ///Bit 18 - USART3 clock enable
    #[inline(always)]
    #[must_use]
    pub fn usart3en(&mut self) -> USART3EN_W<APBENR1rs> {
        USART3EN_W::new(self, 18)
    }
    ///Bit 19 - USART4 clock enable
    #[inline(always)]
    #[must_use]
    pub fn usart4en(&mut self) -> USART4EN_W<APBENR1rs> {
        USART4EN_W::new(self, 19)
    }
    ///Bit 21 - I2C1 clock enable
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<APBENR1rs> {
        I2C1EN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 clock enable
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2C2EN_W<APBENR1rs> {
        I2C2EN_W::new(self, 22)
    }
    ///Bit 27 - Debug support clock enable
    #[inline(always)]
    #[must_use]
    pub fn dbgen(&mut self) -> DBGEN_W<APBENR1rs> {
        DBGEN_W::new(self, 27)
    }
    ///Bit 28 - Power interface clock enable
    #[inline(always)]
    #[must_use]
    pub fn pwren(&mut self) -> PWREN_W<APBENR1rs> {
        PWREN_W::new(self, 28)
    }
}
/**APB peripheral clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`apbenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#RCC:APBENR1)*/
pub struct APBENR1rs;
impl crate::RegisterSpec for APBENR1rs {
    type Ux = u32;
}
///`read()` method returns [`apbenr1::R`](R) reader structure
impl crate::Readable for APBENR1rs {}
///`write(|w| ..)` method takes [`apbenr1::W`](W) writer structure
impl crate::Writable for APBENR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APBENR1 to value 0
impl crate::Resettable for APBENR1rs {
    const RESET_VALUE: u32 = 0;
}
