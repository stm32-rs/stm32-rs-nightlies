///Register `MP_APB5ENSETR` reader
pub type R = crate::R<MP_APB5ENSETRrs>;
///Register `MP_APB5ENSETR` writer
pub type W = crate::W<MP_APB5ENSETRrs>;
///Field `SPI6EN` reader - SPI6EN
pub type SPI6EN_R = crate::BitReader;
///Field `SPI6EN` writer - SPI6EN
pub type SPI6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4EN` reader - I2C4EN
pub type I2C4EN_R = crate::BitReader;
///Field `I2C4EN` writer - I2C4EN
pub type I2C4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C6EN` reader - I2C6EN
pub type I2C6EN_R = crate::BitReader;
///Field `I2C6EN` writer - I2C6EN
pub type I2C6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1EN` reader - USART1EN
pub type USART1EN_R = crate::BitReader;
///Field `USART1EN` writer - USART1EN
pub type USART1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCAPBEN` reader - RTCAPBEN
pub type RTCAPBEN_R = crate::BitReader;
///Field `RTCAPBEN` writer - RTCAPBEN
pub type RTCAPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZC1EN` reader - TZC1EN
pub type TZC1EN_R = crate::BitReader;
///Field `TZC1EN` writer - TZC1EN
pub type TZC1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZC2EN` reader - TZC2EN
pub type TZC2EN_R = crate::BitReader;
///Field `TZC2EN` writer - TZC2EN
pub type TZC2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TZPCEN` reader - TZPCEN
pub type TZPCEN_R = crate::BitReader;
///Field `TZPCEN` writer - TZPCEN
pub type TZPCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG1APBEN` reader - IWDG1APBEN
pub type IWDG1APBEN_R = crate::BitReader;
///Field `IWDG1APBEN` writer - IWDG1APBEN
pub type IWDG1APBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSECEN` reader - BSECEN
pub type BSECEN_R = crate::BitReader;
///Field `BSECEN` writer - BSECEN
pub type BSECEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STGENEN` reader - STGENEN
pub type STGENEN_R = crate::BitReader;
///Field `STGENEN` writer - STGENEN
pub type STGENEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SPI6EN
    #[inline(always)]
    pub fn spi6en(&self) -> SPI6EN_R {
        SPI6EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - I2C4EN
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I2C6EN
    #[inline(always)]
    pub fn i2c6en(&self) -> I2C6EN_R {
        I2C6EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - USART1EN
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - RTCAPBEN
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - TZC1EN
    #[inline(always)]
    pub fn tzc1en(&self) -> TZC1EN_R {
        TZC1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TZC2EN
    #[inline(always)]
    pub fn tzc2en(&self) -> TZC2EN_R {
        TZC2EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TZPCEN
    #[inline(always)]
    pub fn tzpcen(&self) -> TZPCEN_R {
        TZPCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - IWDG1APBEN
    #[inline(always)]
    pub fn iwdg1apben(&self) -> IWDG1APBEN_R {
        IWDG1APBEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - BSECEN
    #[inline(always)]
    pub fn bsecen(&self) -> BSECEN_R {
        BSECEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - STGENEN
    #[inline(always)]
    pub fn stgenen(&self) -> STGENEN_R {
        STGENEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MP_APB5ENSETR")
            .field("spi6en", &self.spi6en())
            .field("i2c4en", &self.i2c4en())
            .field("i2c6en", &self.i2c6en())
            .field("usart1en", &self.usart1en())
            .field("rtcapben", &self.rtcapben())
            .field("tzc1en", &self.tzc1en())
            .field("tzc2en", &self.tzc2en())
            .field("tzpcen", &self.tzpcen())
            .field("iwdg1apben", &self.iwdg1apben())
            .field("bsecen", &self.bsecen())
            .field("stgenen", &self.stgenen())
            .finish()
    }
}
impl W {
    ///Bit 0 - SPI6EN
    #[inline(always)]
    pub fn spi6en(&mut self) -> SPI6EN_W<'_, MP_APB5ENSETRrs> {
        SPI6EN_W::new(self, 0)
    }
    ///Bit 2 - I2C4EN
    #[inline(always)]
    pub fn i2c4en(&mut self) -> I2C4EN_W<'_, MP_APB5ENSETRrs> {
        I2C4EN_W::new(self, 2)
    }
    ///Bit 3 - I2C6EN
    #[inline(always)]
    pub fn i2c6en(&mut self) -> I2C6EN_W<'_, MP_APB5ENSETRrs> {
        I2C6EN_W::new(self, 3)
    }
    ///Bit 4 - USART1EN
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W<'_, MP_APB5ENSETRrs> {
        USART1EN_W::new(self, 4)
    }
    ///Bit 8 - RTCAPBEN
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<'_, MP_APB5ENSETRrs> {
        RTCAPBEN_W::new(self, 8)
    }
    ///Bit 11 - TZC1EN
    #[inline(always)]
    pub fn tzc1en(&mut self) -> TZC1EN_W<'_, MP_APB5ENSETRrs> {
        TZC1EN_W::new(self, 11)
    }
    ///Bit 12 - TZC2EN
    #[inline(always)]
    pub fn tzc2en(&mut self) -> TZC2EN_W<'_, MP_APB5ENSETRrs> {
        TZC2EN_W::new(self, 12)
    }
    ///Bit 13 - TZPCEN
    #[inline(always)]
    pub fn tzpcen(&mut self) -> TZPCEN_W<'_, MP_APB5ENSETRrs> {
        TZPCEN_W::new(self, 13)
    }
    ///Bit 15 - IWDG1APBEN
    #[inline(always)]
    pub fn iwdg1apben(&mut self) -> IWDG1APBEN_W<'_, MP_APB5ENSETRrs> {
        IWDG1APBEN_W::new(self, 15)
    }
    ///Bit 16 - BSECEN
    #[inline(always)]
    pub fn bsecen(&mut self) -> BSECEN_W<'_, MP_APB5ENSETRrs> {
        BSECEN_W::new(self, 16)
    }
    ///Bit 20 - STGENEN
    #[inline(always)]
    pub fn stgenen(&mut self) -> STGENEN_W<'_, MP_APB5ENSETRrs> {
        STGENEN_W::new(self, 20)
    }
}
/**This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .

You can [`read`](crate::Reg::read) this register and get [`mp_apb5ensetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb5ensetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MP_APB5ENSETR)*/
pub struct MP_APB5ENSETRrs;
impl crate::RegisterSpec for MP_APB5ENSETRrs {
    type Ux = u32;
}
///`read()` method returns [`mp_apb5ensetr::R`](R) reader structure
impl crate::Readable for MP_APB5ENSETRrs {}
///`write(|w| ..)` method takes [`mp_apb5ensetr::W`](W) writer structure
impl crate::Writable for MP_APB5ENSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MP_APB5ENSETR to value 0
impl crate::Resettable for MP_APB5ENSETRrs {}
