///Register `APB3ENR` reader
pub type R = crate::R<APB3ENRrs>;
///Register `APB3ENR` writer
pub type W = crate::W<APB3ENRrs>;
/**SBS clock enable Set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBSEN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<SBSEN> for bool {
    #[inline(always)]
    fn from(variant: SBSEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SBSEN` reader - SBS clock enable Set and reset by software.
pub type SBSEN_R = crate::BitReader<SBSEN>;
impl SBSEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SBSEN {
        match self.bits {
            false => SBSEN::Disabled,
            true => SBSEN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SBSEN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SBSEN::Enabled
    }
}
///Field `SBSEN` writer - SBS clock enable Set and reset by software.
pub type SBSEN_W<'a, REG> = crate::BitWriter<'a, REG, SBSEN>;
impl<'a, REG> SBSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SBSEN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SBSEN::Enabled)
    }
}
///Field `SPI5EN` reader - SPI5 clock enable Set and reset by software.
pub use SBSEN_R as SPI5EN_R;
///Field `LPUART1EN` reader - LPUART1 clock enable Set and reset by software.
pub use SBSEN_R as LPUART1EN_R;
///Field `I2C3EN` reader - I2C3 clock enable Set and reset by software.
pub use SBSEN_R as I2C3EN_R;
///Field `I2C4EN` reader - I2C4 clock enable Set and reset by software.
pub use SBSEN_R as I2C4EN_R;
///Field `I3C2EN` reader - I3C2 clock enable
pub use SBSEN_R as I3C2EN_R;
///Field `LPTIM1EN` reader - LPTIM1 clock enable Set and reset by software.
pub use SBSEN_R as LPTIM1EN_R;
///Field `LPTIM3EN` reader - LPTIM3 clock enable Set and reset by software.
pub use SBSEN_R as LPTIM3EN_R;
///Field `LPTIM4EN` reader - LPTIM4 clock enable Set and reset by software.
pub use SBSEN_R as LPTIM4EN_R;
///Field `LPTIM5EN` reader - LPTIM5 clock enable Set and reset by software.
pub use SBSEN_R as LPTIM5EN_R;
///Field `LPTIM6EN` reader - LPTIM6 clock enable Set and reset by software.
pub use SBSEN_R as LPTIM6EN_R;
///Field `VREFEN` reader - VREF clock enable Set and reset by software.
pub use SBSEN_R as VREFEN_R;
///Field `RTCAPBEN` reader - RTC APB interface clock enable Set and reset by software.
pub use SBSEN_R as RTCAPBEN_R;
///Field `SPI5EN` writer - SPI5 clock enable Set and reset by software.
pub use SBSEN_W as SPI5EN_W;
///Field `LPUART1EN` writer - LPUART1 clock enable Set and reset by software.
pub use SBSEN_W as LPUART1EN_W;
///Field `I2C3EN` writer - I2C3 clock enable Set and reset by software.
pub use SBSEN_W as I2C3EN_W;
///Field `I2C4EN` writer - I2C4 clock enable Set and reset by software.
pub use SBSEN_W as I2C4EN_W;
///Field `I3C2EN` writer - I3C2 clock enable
pub use SBSEN_W as I3C2EN_W;
///Field `LPTIM1EN` writer - LPTIM1 clock enable Set and reset by software.
pub use SBSEN_W as LPTIM1EN_W;
///Field `LPTIM3EN` writer - LPTIM3 clock enable Set and reset by software.
pub use SBSEN_W as LPTIM3EN_W;
///Field `LPTIM4EN` writer - LPTIM4 clock enable Set and reset by software.
pub use SBSEN_W as LPTIM4EN_W;
///Field `LPTIM5EN` writer - LPTIM5 clock enable Set and reset by software.
pub use SBSEN_W as LPTIM5EN_W;
///Field `LPTIM6EN` writer - LPTIM6 clock enable Set and reset by software.
pub use SBSEN_W as LPTIM6EN_W;
///Field `VREFEN` writer - VREF clock enable Set and reset by software.
pub use SBSEN_W as VREFEN_W;
///Field `RTCAPBEN` writer - RTC APB interface clock enable Set and reset by software.
pub use SBSEN_W as RTCAPBEN_W;
impl R {
    ///Bit 1 - SBS clock enable Set and reset by software.
    #[inline(always)]
    pub fn sbsen(&self) -> SBSEN_R {
        SBSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - SPI5 clock enable Set and reset by software.
    #[inline(always)]
    pub fn spi5en(&self) -> SPI5EN_R {
        SPI5EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LPUART1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C3 clock enable Set and reset by software.
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - I2C4 clock enable Set and reset by software.
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - I3C2 clock enable
    #[inline(always)]
    pub fn i3c2en(&self) -> I3C2EN_R {
        I3C2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - LPTIM1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM3 clock enable Set and reset by software.
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - LPTIM4 clock enable Set and reset by software.
    #[inline(always)]
    pub fn lptim4en(&self) -> LPTIM4EN_R {
        LPTIM4EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - LPTIM5 clock enable Set and reset by software.
    #[inline(always)]
    pub fn lptim5en(&self) -> LPTIM5EN_R {
        LPTIM5EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - LPTIM6 clock enable Set and reset by software.
    #[inline(always)]
    pub fn lptim6en(&self) -> LPTIM6EN_R {
        LPTIM6EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 20 - VREF clock enable Set and reset by software.
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - RTC APB interface clock enable Set and reset by software.
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB3ENR")
            .field("sbsen", &self.sbsen())
            .field("spi5en", &self.spi5en())
            .field("lpuart1en", &self.lpuart1en())
            .field("i2c3en", &self.i2c3en())
            .field("i2c4en", &self.i2c4en())
            .field("lptim1en", &self.lptim1en())
            .field("lptim3en", &self.lptim3en())
            .field("lptim4en", &self.lptim4en())
            .field("lptim5en", &self.lptim5en())
            .field("lptim6en", &self.lptim6en())
            .field("vrefen", &self.vrefen())
            .field("rtcapben", &self.rtcapben())
            .field("i3c2en", &self.i3c2en())
            .finish()
    }
}
impl W {
    ///Bit 1 - SBS clock enable Set and reset by software.
    #[inline(always)]
    pub fn sbsen(&mut self) -> SBSEN_W<'_, APB3ENRrs> {
        SBSEN_W::new(self, 1)
    }
    ///Bit 5 - SPI5 clock enable Set and reset by software.
    #[inline(always)]
    pub fn spi5en(&mut self) -> SPI5EN_W<'_, APB3ENRrs> {
        SPI5EN_W::new(self, 5)
    }
    ///Bit 6 - LPUART1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<'_, APB3ENRrs> {
        LPUART1EN_W::new(self, 6)
    }
    ///Bit 7 - I2C3 clock enable Set and reset by software.
    #[inline(always)]
    pub fn i2c3en(&mut self) -> I2C3EN_W<'_, APB3ENRrs> {
        I2C3EN_W::new(self, 7)
    }
    ///Bit 8 - I2C4 clock enable Set and reset by software.
    #[inline(always)]
    pub fn i2c4en(&mut self) -> I2C4EN_W<'_, APB3ENRrs> {
        I2C4EN_W::new(self, 8)
    }
    ///Bit 9 - I3C2 clock enable
    #[inline(always)]
    pub fn i3c2en(&mut self) -> I3C2EN_W<'_, APB3ENRrs> {
        I3C2EN_W::new(self, 9)
    }
    ///Bit 11 - LPTIM1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<'_, APB3ENRrs> {
        LPTIM1EN_W::new(self, 11)
    }
    ///Bit 12 - LPTIM3 clock enable Set and reset by software.
    #[inline(always)]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W<'_, APB3ENRrs> {
        LPTIM3EN_W::new(self, 12)
    }
    ///Bit 13 - LPTIM4 clock enable Set and reset by software.
    #[inline(always)]
    pub fn lptim4en(&mut self) -> LPTIM4EN_W<'_, APB3ENRrs> {
        LPTIM4EN_W::new(self, 13)
    }
    ///Bit 14 - LPTIM5 clock enable Set and reset by software.
    #[inline(always)]
    pub fn lptim5en(&mut self) -> LPTIM5EN_W<'_, APB3ENRrs> {
        LPTIM5EN_W::new(self, 14)
    }
    ///Bit 15 - LPTIM6 clock enable Set and reset by software.
    #[inline(always)]
    pub fn lptim6en(&mut self) -> LPTIM6EN_W<'_, APB3ENRrs> {
        LPTIM6EN_W::new(self, 15)
    }
    ///Bit 20 - VREF clock enable Set and reset by software.
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W<'_, APB3ENRrs> {
        VREFEN_W::new(self, 20)
    }
    ///Bit 21 - RTC APB interface clock enable Set and reset by software.
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<'_, APB3ENRrs> {
        RTCAPBEN_W::new(self, 21)
    }
}
/**RCC APB4 peripheral clock register

You can [`read`](crate::Reg::read) this register and get [`apb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#RCC:APB3ENR)*/
pub struct APB3ENRrs;
impl crate::RegisterSpec for APB3ENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb3enr::R`](R) reader structure
impl crate::Readable for APB3ENRrs {}
///`write(|w| ..)` method takes [`apb3enr::W`](W) writer structure
impl crate::Writable for APB3ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3ENR to value 0
impl crate::Resettable for APB3ENRrs {}
