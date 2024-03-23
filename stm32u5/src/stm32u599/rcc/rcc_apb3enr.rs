#[doc = "Register `RCC_APB3ENR` reader"]
pub type R = crate::R<RCC_APB3ENRrs>;
#[doc = "Register `RCC_APB3ENR` writer"]
pub type W = crate::W<RCC_APB3ENRrs>;
#[doc = "Field `SYSCFGEN` reader - SYSCFG clock enable This bit is set and cleared by software."]
pub type SYSCFGEN_R = crate::BitReader;
#[doc = "Field `SYSCFGEN` writer - SYSCFG clock enable This bit is set and cleared by software."]
pub type SYSCFGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3EN` reader - SPI3 clock enable This bit is set and cleared by software."]
pub type SPI3EN_R = crate::BitReader;
#[doc = "Field `SPI3EN` writer - SPI3 clock enable This bit is set and cleared by software."]
pub type SPI3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUART1EN` reader - LPUART1 clock enable This bit is set and cleared by software."]
pub type LPUART1EN_R = crate::BitReader;
#[doc = "Field `LPUART1EN` writer - LPUART1 clock enable This bit is set and cleared by software."]
pub type LPUART1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3EN` reader - I2C3 clock enable This bit is set and cleared by software."]
pub type I2C3EN_R = crate::BitReader;
#[doc = "Field `I2C3EN` writer - I2C3 clock enable This bit is set and cleared by software."]
pub type I2C3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1EN` reader - LPTIM1 clock enable This bit is set and cleared by software."]
pub type LPTIM1EN_R = crate::BitReader;
#[doc = "Field `LPTIM1EN` writer - LPTIM1 clock enable This bit is set and cleared by software."]
pub type LPTIM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM3EN` reader - LPTIM3 clock enable This bit is set and cleared by software."]
pub type LPTIM3EN_R = crate::BitReader;
#[doc = "Field `LPTIM3EN` writer - LPTIM3 clock enable This bit is set and cleared by software."]
pub type LPTIM3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM4EN` reader - LPTIM4 clock enable This bit is set and cleared by software."]
pub type LPTIM4EN_R = crate::BitReader;
#[doc = "Field `LPTIM4EN` writer - LPTIM4 clock enable This bit is set and cleared by software."]
pub type LPTIM4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAMPEN` reader - OPAMP clock enable This bit is set and cleared by software."]
pub type OPAMPEN_R = crate::BitReader;
#[doc = "Field `OPAMPEN` writer - OPAMP clock enable This bit is set and cleared by software."]
pub type OPAMPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPEN` reader - COMP clock enable This bit is set and cleared by software."]
pub type COMPEN_R = crate::BitReader;
#[doc = "Field `COMPEN` writer - COMP clock enable This bit is set and cleared by software."]
pub type COMPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFEN` reader - VREFBUF clock enable This bit is set and cleared by software."]
pub type VREFEN_R = crate::BitReader;
#[doc = "Field `VREFEN` writer - VREFBUF clock enable This bit is set and cleared by software."]
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAPBEN` reader - RTC and TAMP APB clock enable This bit is set and cleared by software."]
pub type RTCAPBEN_R = crate::BitReader;
#[doc = "Field `RTCAPBEN` writer - RTC and TAMP APB clock enable This bit is set and cleared by software."]
pub type RTCAPBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - SYSCFG clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI3 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LPUART1 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C3 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM1 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPTIM3 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LPTIM4 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn lptim4en(&self) -> LPTIM4EN_R {
        LPTIM4EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - OPAMP clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn opampen(&self) -> OPAMPEN_R {
        OPAMPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - COMP clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn compen(&self) -> COMPEN_R {
        COMPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - VREFBUF clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RTC and TAMP APB clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SYSCFG clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<RCC_APB3ENRrs> {
        SYSCFGEN_W::new(self, 1)
    }
    #[doc = "Bit 5 - SPI3 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi3en(&mut self) -> SPI3EN_W<RCC_APB3ENRrs> {
        SPI3EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - LPUART1 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<RCC_APB3ENRrs> {
        LPUART1EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - I2C3 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c3en(&mut self) -> I2C3EN_W<RCC_APB3ENRrs> {
        I2C3EN_W::new(self, 7)
    }
    #[doc = "Bit 11 - LPTIM1 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<RCC_APB3ENRrs> {
        LPTIM1EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - LPTIM3 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W<RCC_APB3ENRrs> {
        LPTIM3EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - LPTIM4 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim4en(&mut self) -> LPTIM4EN_W<RCC_APB3ENRrs> {
        LPTIM4EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - OPAMP clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn opampen(&mut self) -> OPAMPEN_W<RCC_APB3ENRrs> {
        OPAMPEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - COMP clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn compen(&mut self) -> COMPEN_W<RCC_APB3ENRrs> {
        COMPEN_W::new(self, 15)
    }
    #[doc = "Bit 20 - VREFBUF clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<RCC_APB3ENRrs> {
        VREFEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - RTC and TAMP APB clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<RCC_APB3ENRrs> {
        RTCAPBEN_W::new(self, 21)
    }
}
#[doc = "RCC APB3 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb3enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb3enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_APB3ENRrs;
impl crate::RegisterSpec for RCC_APB3ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb3enr::R`](R) reader structure"]
impl crate::Readable for RCC_APB3ENRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb3enr::W`](W) writer structure"]
impl crate::Writable for RCC_APB3ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APB3ENR to value 0"]
impl crate::Resettable for RCC_APB3ENRrs {
    const RESET_VALUE: u32 = 0;
}
