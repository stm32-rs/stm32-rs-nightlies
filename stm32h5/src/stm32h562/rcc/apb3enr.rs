#[doc = "Register `APB3ENR` reader"]
pub type R = crate::R<APB3ENRrs>;
#[doc = "Register `APB3ENR` writer"]
pub type W = crate::W<APB3ENRrs>;
#[doc = "SBS clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBSEN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<SBSEN> for bool {
    #[inline(always)]
    fn from(variant: SBSEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBSEN` reader - SBS clock enable Set and reset by software."]
pub type SBSEN_R = crate::BitReader<SBSEN>;
impl SBSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBSEN {
        match self.bits {
            false => SBSEN::Disabled,
            true => SBSEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SBSEN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SBSEN::Enabled
    }
}
#[doc = "Field `SBSEN` writer - SBS clock enable Set and reset by software."]
pub type SBSEN_W<'a, REG> = crate::BitWriter<'a, REG, SBSEN>;
impl<'a, REG> SBSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SBSEN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SBSEN::Enabled)
    }
}
#[doc = "Field `SPI5EN` reader - SPI5 clock enable Set and reset by software."]
pub use SBSEN_R as SPI5EN_R;
#[doc = "Field `LPUART1EN` reader - LPUART1 clock enable Set and reset by software."]
pub use SBSEN_R as LPUART1EN_R;
#[doc = "Field `I2C3EN` reader - I2C3 clock enable Set and reset by software."]
pub use SBSEN_R as I2C3EN_R;
#[doc = "Field `I2C4EN` reader - I2C4 clock enable Set and reset by software."]
pub use SBSEN_R as I2C4EN_R;
#[doc = "Field `LPTIM1EN` reader - LPTIM1 clock enable Set and reset by software."]
pub use SBSEN_R as LPTIM1EN_R;
#[doc = "Field `LPTIM3EN` reader - LPTIM3 clock enable Set and reset by software."]
pub use SBSEN_R as LPTIM3EN_R;
#[doc = "Field `LPTIM4EN` reader - LPTIM4 clock enable Set and reset by software."]
pub use SBSEN_R as LPTIM4EN_R;
#[doc = "Field `LPTIM5EN` reader - LPTIM5 clock enable Set and reset by software."]
pub use SBSEN_R as LPTIM5EN_R;
#[doc = "Field `LPTIM6EN` reader - LPTIM6 clock enable Set and reset by software."]
pub use SBSEN_R as LPTIM6EN_R;
#[doc = "Field `VREFEN` reader - VREF clock enable Set and reset by software."]
pub use SBSEN_R as VREFEN_R;
#[doc = "Field `RTCAPBEN` reader - RTC APB interface clock enable Set and reset by software."]
pub use SBSEN_R as RTCAPBEN_R;
#[doc = "Field `SPI5EN` writer - SPI5 clock enable Set and reset by software."]
pub use SBSEN_W as SPI5EN_W;
#[doc = "Field `LPUART1EN` writer - LPUART1 clock enable Set and reset by software."]
pub use SBSEN_W as LPUART1EN_W;
#[doc = "Field `I2C3EN` writer - I2C3 clock enable Set and reset by software."]
pub use SBSEN_W as I2C3EN_W;
#[doc = "Field `I2C4EN` writer - I2C4 clock enable Set and reset by software."]
pub use SBSEN_W as I2C4EN_W;
#[doc = "Field `LPTIM1EN` writer - LPTIM1 clock enable Set and reset by software."]
pub use SBSEN_W as LPTIM1EN_W;
#[doc = "Field `LPTIM3EN` writer - LPTIM3 clock enable Set and reset by software."]
pub use SBSEN_W as LPTIM3EN_W;
#[doc = "Field `LPTIM4EN` writer - LPTIM4 clock enable Set and reset by software."]
pub use SBSEN_W as LPTIM4EN_W;
#[doc = "Field `LPTIM5EN` writer - LPTIM5 clock enable Set and reset by software."]
pub use SBSEN_W as LPTIM5EN_W;
#[doc = "Field `LPTIM6EN` writer - LPTIM6 clock enable Set and reset by software."]
pub use SBSEN_W as LPTIM6EN_W;
#[doc = "Field `VREFEN` writer - VREF clock enable Set and reset by software."]
pub use SBSEN_W as VREFEN_W;
#[doc = "Field `RTCAPBEN` writer - RTC APB interface clock enable Set and reset by software."]
pub use SBSEN_W as RTCAPBEN_W;
impl R {
    #[doc = "Bit 1 - SBS clock enable Set and reset by software."]
    #[inline(always)]
    pub fn sbsen(&self) -> SBSEN_R {
        SBSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI5 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn spi5en(&self) -> SPI5EN_R {
        SPI5EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LPUART1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C3 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C4 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPTIM3 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LPTIM4 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn lptim4en(&self) -> LPTIM4EN_R {
        LPTIM4EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LPTIM5 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn lptim5en(&self) -> LPTIM5EN_R {
        LPTIM5EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LPTIM6 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn lptim6en(&self) -> LPTIM6EN_R {
        LPTIM6EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - VREF clock enable Set and reset by software."]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RTC APB interface clock enable Set and reset by software."]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SBS clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sbsen(&mut self) -> SBSEN_W<APB3ENRrs> {
        SBSEN_W::new(self, 1)
    }
    #[doc = "Bit 5 - SPI5 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi5en(&mut self) -> SPI5EN_W<APB3ENRrs> {
        SPI5EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - LPUART1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<APB3ENRrs> {
        LPUART1EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - I2C3 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c3en(&mut self) -> I2C3EN_W<APB3ENRrs> {
        I2C3EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - I2C4 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c4en(&mut self) -> I2C4EN_W<APB3ENRrs> {
        I2C4EN_W::new(self, 8)
    }
    #[doc = "Bit 11 - LPTIM1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<APB3ENRrs> {
        LPTIM1EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - LPTIM3 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W<APB3ENRrs> {
        LPTIM3EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - LPTIM4 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim4en(&mut self) -> LPTIM4EN_W<APB3ENRrs> {
        LPTIM4EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - LPTIM5 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim5en(&mut self) -> LPTIM5EN_W<APB3ENRrs> {
        LPTIM5EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - LPTIM6 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim6en(&mut self) -> LPTIM6EN_W<APB3ENRrs> {
        LPTIM6EN_W::new(self, 15)
    }
    #[doc = "Bit 20 - VREF clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<APB3ENRrs> {
        VREFEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - RTC APB interface clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<APB3ENRrs> {
        RTCAPBEN_W::new(self, 21)
    }
}
#[doc = "RCC APB4 peripheral clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB3ENRrs;
impl crate::RegisterSpec for APB3ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb3enr::R`](R) reader structure"]
impl crate::Readable for APB3ENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb3enr::W`](W) writer structure"]
impl crate::Writable for APB3ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB3ENR to value 0"]
impl crate::Resettable for APB3ENRrs {
    const RESET_VALUE: u32 = 0;
}
