///Register `APBENR1` reader
pub type R = crate::R<APBENR1rs>;
///Register `APBENR1` writer
pub type W = crate::W<APBENR1rs>;
///Field `TIM3EN` reader - TIM3 timer clock enable Set and cleared by software.
pub type TIM3EN_R = crate::BitReader;
///Field `TIM3EN` writer - TIM3 timer clock enable Set and cleared by software.
pub type TIM3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCAPBEN` reader - RTC APB clock enable Set and cleared by software.
pub type RTCAPBEN_R = crate::BitReader;
///Field `RTCAPBEN` writer - RTC APB clock enable Set and cleared by software.
pub type RTCAPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGEN` reader - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0.
pub type WWDGEN_R = crate::BitReader;
///Field `WWDGEN` writer - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0.
pub type WWDGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2EN` reader - USART2 clock enable Set and cleared by software.
pub type USART2EN_R = crate::BitReader;
///Field `USART2EN` writer - USART2 clock enable Set and cleared by software.
pub type USART2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1EN` reader - I2C1 clock enable Set and cleared by software.
pub type I2C1EN_R = crate::BitReader;
///Field `I2C1EN` writer - I2C1 clock enable Set and cleared by software.
pub type I2C1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGEN` reader - Debug support clock enable Set and cleared by software.
pub type DBGEN_R = crate::BitReader;
///Field `DBGEN` writer - Debug support clock enable Set and cleared by software.
pub type DBGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWREN` reader - Power interface clock enable Set and cleared by software.
pub type PWREN_R = crate::BitReader;
///Field `PWREN` writer - Power interface clock enable Set and cleared by software.
pub type PWREN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - TIM3 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 10 - RTC APB clock enable Set and cleared by software.
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0.
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 17 - USART2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 27 - Debug support clock enable Set and cleared by software.
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Power interface clock enable Set and cleared by software.
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APBENR1")
            .field("tim3en", &self.tim3en())
            .field("rtcapben", &self.rtcapben())
            .field("wwdgen", &self.wwdgen())
            .field("usart2en", &self.usart2en())
            .field("i2c1en", &self.i2c1en())
            .field("dbgen", &self.dbgen())
            .field("pwren", &self.pwren())
            .finish()
    }
}
impl W {
    ///Bit 1 - TIM3 timer clock enable Set and cleared by software.
    #[inline(always)]
    pub fn tim3en(&mut self) -> TIM3EN_W<APBENR1rs> {
        TIM3EN_W::new(self, 1)
    }
    ///Bit 10 - RTC APB clock enable Set and cleared by software.
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<APBENR1rs> {
        RTCAPBEN_W::new(self, 10)
    }
    ///Bit 11 - WWDG clock enable Set by software to enable the window watchdog clock. Cleared by hardware system reset This bit can also be set by hardware if the WWDG_SW option bit is 0.
    #[inline(always)]
    pub fn wwdgen(&mut self) -> WWDGEN_W<APBENR1rs> {
        WWDGEN_W::new(self, 11)
    }
    ///Bit 17 - USART2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn usart2en(&mut self) -> USART2EN_W<APBENR1rs> {
        USART2EN_W::new(self, 17)
    }
    ///Bit 21 - I2C1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W<APBENR1rs> {
        I2C1EN_W::new(self, 21)
    }
    ///Bit 27 - Debug support clock enable Set and cleared by software.
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W<APBENR1rs> {
        DBGEN_W::new(self, 27)
    }
    ///Bit 28 - Power interface clock enable Set and cleared by software.
    #[inline(always)]
    pub fn pwren(&mut self) -> PWREN_W<APBENR1rs> {
        PWREN_W::new(self, 28)
    }
}
/**RCC APB peripheral clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`apbenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#RCC:APBENR1)*/
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
