///Register `RCC_APB3ENR` reader
pub type R = crate::R<RCC_APB3ENRrs>;
///Register `RCC_APB3ENR` writer
pub type W = crate::W<RCC_APB3ENRrs>;
///Field `SYSCFGEN` reader - SYSCFG clock enable This bit is set and cleared by software.
pub type SYSCFGEN_R = crate::BitReader;
///Field `SYSCFGEN` writer - SYSCFG clock enable This bit is set and cleared by software.
pub type SYSCFGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3EN` reader - SPI3 clock enable This bit is set and cleared by software.
pub type SPI3EN_R = crate::BitReader;
///Field `SPI3EN` writer - SPI3 clock enable This bit is set and cleared by software.
pub type SPI3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1EN` reader - LPUART1 clock enable This bit is set and cleared by software.
pub type LPUART1EN_R = crate::BitReader;
///Field `LPUART1EN` writer - LPUART1 clock enable This bit is set and cleared by software.
pub type LPUART1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3EN` reader - I2C3 clock enable This bit is set and cleared by software.
pub type I2C3EN_R = crate::BitReader;
///Field `I2C3EN` writer - I2C3 clock enable This bit is set and cleared by software.
pub type I2C3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1EN` reader - LPTIM1 clock enable This bit is set and cleared by software.
pub type LPTIM1EN_R = crate::BitReader;
///Field `LPTIM1EN` writer - LPTIM1 clock enable This bit is set and cleared by software.
pub type LPTIM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3EN` reader - LPTIM3 clock enable This bit is set and cleared by software.
pub type LPTIM3EN_R = crate::BitReader;
///Field `LPTIM3EN` writer - LPTIM3 clock enable This bit is set and cleared by software.
pub type LPTIM3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM4EN` reader - LPTIM4 clock enable This bit is set and cleared by software.
pub type LPTIM4EN_R = crate::BitReader;
///Field `LPTIM4EN` writer - LPTIM4 clock enable This bit is set and cleared by software.
pub type LPTIM4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPAMPEN` reader - OPAMP clock enable This bit is set and cleared by software.
pub type OPAMPEN_R = crate::BitReader;
///Field `OPAMPEN` writer - OPAMP clock enable This bit is set and cleared by software.
pub type OPAMPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMPEN` reader - COMP clock enable This bit is set and cleared by software.
pub type COMPEN_R = crate::BitReader;
///Field `COMPEN` writer - COMP clock enable This bit is set and cleared by software.
pub type COMPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFEN` reader - VREFBUF clock enable This bit is set and cleared by software.
pub type VREFEN_R = crate::BitReader;
///Field `VREFEN` writer - VREFBUF clock enable This bit is set and cleared by software.
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCAPBEN` reader - RTC and TAMP APB clock enable This bit is set and cleared by software.
pub type RTCAPBEN_R = crate::BitReader;
///Field `RTCAPBEN` writer - RTC and TAMP APB clock enable This bit is set and cleared by software.
pub type RTCAPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - SYSCFG clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - SPI3 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LPUART1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C3 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - LPTIM1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM3 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - LPTIM4 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn lptim4en(&self) -> LPTIM4EN_R {
        LPTIM4EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - OPAMP clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn opampen(&self) -> OPAMPEN_R {
        OPAMPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - COMP clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn compen(&self) -> COMPEN_R {
        COMPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 20 - VREFBUF clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - RTC and TAMP APB clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APB3ENR")
            .field("syscfgen", &self.syscfgen())
            .field("spi3en", &self.spi3en())
            .field("lpuart1en", &self.lpuart1en())
            .field("i2c3en", &self.i2c3en())
            .field("lptim1en", &self.lptim1en())
            .field("lptim3en", &self.lptim3en())
            .field("lptim4en", &self.lptim4en())
            .field("opampen", &self.opampen())
            .field("compen", &self.compen())
            .field("vrefen", &self.vrefen())
            .field("rtcapben", &self.rtcapben())
            .finish()
    }
}
impl W {
    ///Bit 1 - SYSCFG clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<RCC_APB3ENRrs> {
        SYSCFGEN_W::new(self, 1)
    }
    ///Bit 5 - SPI3 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn spi3en(&mut self) -> SPI3EN_W<RCC_APB3ENRrs> {
        SPI3EN_W::new(self, 5)
    }
    ///Bit 6 - LPUART1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<RCC_APB3ENRrs> {
        LPUART1EN_W::new(self, 6)
    }
    ///Bit 7 - I2C3 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn i2c3en(&mut self) -> I2C3EN_W<RCC_APB3ENRrs> {
        I2C3EN_W::new(self, 7)
    }
    ///Bit 11 - LPTIM1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<RCC_APB3ENRrs> {
        LPTIM1EN_W::new(self, 11)
    }
    ///Bit 12 - LPTIM3 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W<RCC_APB3ENRrs> {
        LPTIM3EN_W::new(self, 12)
    }
    ///Bit 13 - LPTIM4 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lptim4en(&mut self) -> LPTIM4EN_W<RCC_APB3ENRrs> {
        LPTIM4EN_W::new(self, 13)
    }
    ///Bit 14 - OPAMP clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn opampen(&mut self) -> OPAMPEN_W<RCC_APB3ENRrs> {
        OPAMPEN_W::new(self, 14)
    }
    ///Bit 15 - COMP clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn compen(&mut self) -> COMPEN_W<RCC_APB3ENRrs> {
        COMPEN_W::new(self, 15)
    }
    ///Bit 20 - VREFBUF clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<RCC_APB3ENRrs> {
        VREFEN_W::new(self, 20)
    }
    ///Bit 21 - RTC and TAMP APB clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<RCC_APB3ENRrs> {
        RTCAPBEN_W::new(self, 21)
    }
}
/**RCC APB3 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`rcc_apb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#RCC:RCC_APB3ENR)*/
pub struct RCC_APB3ENRrs;
impl crate::RegisterSpec for RCC_APB3ENRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_apb3enr::R`](R) reader structure
impl crate::Readable for RCC_APB3ENRrs {}
///`write(|w| ..)` method takes [`rcc_apb3enr::W`](W) writer structure
impl crate::Writable for RCC_APB3ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_APB3ENR to value 0
impl crate::Resettable for RCC_APB3ENRrs {
    const RESET_VALUE: u32 = 0;
}
