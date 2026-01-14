///Register `APB4LENR` reader
pub type R = crate::R<APB4LENRrs>;
///Register `APB4LENR` writer
pub type W = crate::W<APB4LENRrs>;
///Field `HDPEN` reader - HDP enable
pub type HDPEN_R = crate::BitReader;
///Field `HDPEN` writer - HDP enable
pub type HDPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1EN` reader - LPUART1 enable
pub type LPUART1EN_R = crate::BitReader;
///Field `LPUART1EN` writer - LPUART1 enable
pub type LPUART1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI6EN` reader - SPI6 enable
pub type SPI6EN_R = crate::BitReader;
///Field `SPI6EN` writer - SPI6 enable
pub type SPI6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4EN` reader - I2C4 enable
pub type I2C4EN_R = crate::BitReader;
///Field `I2C4EN` writer - I2C4 enable
pub type I2C4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2EN` reader - LPTIM2 enable
pub type LPTIM2EN_R = crate::BitReader;
///Field `LPTIM2EN` writer - LPTIM2 enable
pub type LPTIM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3EN` reader - LPTIM3 enable
pub type LPTIM3EN_R = crate::BitReader;
///Field `LPTIM3EN` writer - LPTIM3 enable
pub type LPTIM3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM4EN` reader - LPTIM4 enable
pub type LPTIM4EN_R = crate::BitReader;
///Field `LPTIM4EN` writer - LPTIM4 enable
pub type LPTIM4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM5EN` reader - LPTIM5 enable
pub type LPTIM5EN_R = crate::BitReader;
///Field `LPTIM5EN` writer - LPTIM5 enable
pub type LPTIM5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFBUFEN` reader - VREFBUF enable
pub type VREFBUFEN_R = crate::BitReader;
///Field `VREFBUFEN` writer - VREFBUF enable
pub type VREFBUFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCEN` reader - RTC enable
pub type RTCEN_R = crate::BitReader;
///Field `RTCEN` writer - RTC enable
pub type RTCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCAPBEN` reader - RTCAPB enable
pub type RTCAPBEN_R = crate::BitReader;
///Field `RTCAPBEN` writer - RTCAPB enable
pub type RTCAPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R2GRETEN` reader - R2GRET enable
pub type R2GRETEN_R = crate::BitReader;
///Field `R2GRETEN` writer - R2GRET enable
pub type R2GRETEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R2GNPUEN` reader - R2GNPU enable
pub type R2GNPUEN_R = crate::BitReader;
///Field `R2GNPUEN` writer - R2GNPU enable
pub type R2GNPUEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SERFEN` reader - SERF enable
pub type SERFEN_R = crate::BitReader;
///Field `SERFEN` writer - SERF enable
pub type SERFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - HDP enable
    #[inline(always)]
    pub fn hdpen(&self) -> HDPEN_R {
        HDPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LPUART1 enable
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - SPI6 enable
    #[inline(always)]
    pub fn spi6en(&self) -> SPI6EN_R {
        SPI6EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - I2C4 enable
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - LPTIM2 enable
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - LPTIM3 enable
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - LPTIM4 enable
    #[inline(always)]
    pub fn lptim4en(&self) -> LPTIM4EN_R {
        LPTIM4EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM5 enable
    #[inline(always)]
    pub fn lptim5en(&self) -> LPTIM5EN_R {
        LPTIM5EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - VREFBUF enable
    #[inline(always)]
    pub fn vrefbufen(&self) -> VREFBUFEN_R {
        VREFBUFEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - RTC enable
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - RTCAPB enable
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 22 - R2GRET enable
    #[inline(always)]
    pub fn r2greten(&self) -> R2GRETEN_R {
        R2GRETEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - R2GNPU enable
    #[inline(always)]
    pub fn r2gnpuen(&self) -> R2GNPUEN_R {
        R2GNPUEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 31 - SERF enable
    #[inline(always)]
    pub fn serfen(&self) -> SERFEN_R {
        SERFEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB4LENR")
            .field("hdpen", &self.hdpen())
            .field("lpuart1en", &self.lpuart1en())
            .field("spi6en", &self.spi6en())
            .field("i2c4en", &self.i2c4en())
            .field("lptim2en", &self.lptim2en())
            .field("lptim3en", &self.lptim3en())
            .field("lptim4en", &self.lptim4en())
            .field("lptim5en", &self.lptim5en())
            .field("vrefbufen", &self.vrefbufen())
            .field("rtcen", &self.rtcen())
            .field("rtcapben", &self.rtcapben())
            .field("r2greten", &self.r2greten())
            .field("r2gnpuen", &self.r2gnpuen())
            .field("serfen", &self.serfen())
            .finish()
    }
}
impl W {
    ///Bit 2 - HDP enable
    #[inline(always)]
    pub fn hdpen(&mut self) -> HDPEN_W<'_, APB4LENRrs> {
        HDPEN_W::new(self, 2)
    }
    ///Bit 3 - LPUART1 enable
    #[inline(always)]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<'_, APB4LENRrs> {
        LPUART1EN_W::new(self, 3)
    }
    ///Bit 5 - SPI6 enable
    #[inline(always)]
    pub fn spi6en(&mut self) -> SPI6EN_W<'_, APB4LENRrs> {
        SPI6EN_W::new(self, 5)
    }
    ///Bit 7 - I2C4 enable
    #[inline(always)]
    pub fn i2c4en(&mut self) -> I2C4EN_W<'_, APB4LENRrs> {
        I2C4EN_W::new(self, 7)
    }
    ///Bit 9 - LPTIM2 enable
    #[inline(always)]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<'_, APB4LENRrs> {
        LPTIM2EN_W::new(self, 9)
    }
    ///Bit 10 - LPTIM3 enable
    #[inline(always)]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W<'_, APB4LENRrs> {
        LPTIM3EN_W::new(self, 10)
    }
    ///Bit 11 - LPTIM4 enable
    #[inline(always)]
    pub fn lptim4en(&mut self) -> LPTIM4EN_W<'_, APB4LENRrs> {
        LPTIM4EN_W::new(self, 11)
    }
    ///Bit 12 - LPTIM5 enable
    #[inline(always)]
    pub fn lptim5en(&mut self) -> LPTIM5EN_W<'_, APB4LENRrs> {
        LPTIM5EN_W::new(self, 12)
    }
    ///Bit 15 - VREFBUF enable
    #[inline(always)]
    pub fn vrefbufen(&mut self) -> VREFBUFEN_W<'_, APB4LENRrs> {
        VREFBUFEN_W::new(self, 15)
    }
    ///Bit 16 - RTC enable
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W<'_, APB4LENRrs> {
        RTCEN_W::new(self, 16)
    }
    ///Bit 17 - RTCAPB enable
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<'_, APB4LENRrs> {
        RTCAPBEN_W::new(self, 17)
    }
    ///Bit 22 - R2GRET enable
    #[inline(always)]
    pub fn r2greten(&mut self) -> R2GRETEN_W<'_, APB4LENRrs> {
        R2GRETEN_W::new(self, 22)
    }
    ///Bit 23 - R2GNPU enable
    #[inline(always)]
    pub fn r2gnpuen(&mut self) -> R2GNPUEN_W<'_, APB4LENRrs> {
        R2GNPUEN_W::new(self, 23)
    }
    ///Bit 31 - SERF enable
    #[inline(always)]
    pub fn serfen(&mut self) -> SERFEN_W<'_, APB4LENRrs> {
        SERFEN_W::new(self, 31)
    }
}
/**RCC APB4L enable register

You can [`read`](crate::Reg::read) this register and get [`apb4lenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4lenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB4LENR)*/
pub struct APB4LENRrs;
impl crate::RegisterSpec for APB4LENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb4lenr::R`](R) reader structure
impl crate::Readable for APB4LENRrs {}
///`write(|w| ..)` method takes [`apb4lenr::W`](W) writer structure
impl crate::Writable for APB4LENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4LENR to value 0
impl crate::Resettable for APB4LENRrs {}
