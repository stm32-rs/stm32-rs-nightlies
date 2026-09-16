///Register `APB4ENR` reader
pub type R = crate::R<APB4ENRrs>;
///Register `APB4ENR` writer
pub type W = crate::W<APB4ENRrs>;
/**SBS peripheral clock enable

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
///Field `SBSEN` reader - SBS peripheral clock enable
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
///Field `SBSEN` writer - SBS peripheral clock enable
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
///Field `LPUART1EN` reader - LPUART1 peripheral clocks enable
pub use SBSEN_R as LPUART1EN_R;
///Field `SPI6EN` reader - SPI/I2S6 peripheral clocks enable
pub use SBSEN_R as SPI6EN_R;
///Field `LPTIM2EN` reader - LPTIM2 peripheral clocks enable
pub use SBSEN_R as LPTIM2EN_R;
///Field `LPTIM3EN` reader - LPTIM3 peripheral clocks enable
pub use SBSEN_R as LPTIM3EN_R;
///Field `LPTIM4EN` reader - LPTIM4 peripheral clocks enable
pub use SBSEN_R as LPTIM4EN_R;
///Field `LPTIM5EN` reader - LPTIM5 peripheral clocks enable
pub use SBSEN_R as LPTIM5EN_R;
///Field `VREFEN` reader - VREF peripheral clock enable
pub use SBSEN_R as VREFEN_R;
///Field `RTCAPBEN` reader - RTC APB clock enable
pub use SBSEN_R as RTCAPBEN_R;
///Field `DTSEN` reader - Temperature Sensor peripheral clock enable
pub use SBSEN_R as DTSEN_R;
///Field `LPUART1EN` writer - LPUART1 peripheral clocks enable
pub use SBSEN_W as LPUART1EN_W;
///Field `SPI6EN` writer - SPI/I2S6 peripheral clocks enable
pub use SBSEN_W as SPI6EN_W;
///Field `LPTIM2EN` writer - LPTIM2 peripheral clocks enable
pub use SBSEN_W as LPTIM2EN_W;
///Field `LPTIM3EN` writer - LPTIM3 peripheral clocks enable
pub use SBSEN_W as LPTIM3EN_W;
///Field `LPTIM4EN` writer - LPTIM4 peripheral clocks enable
pub use SBSEN_W as LPTIM4EN_W;
///Field `LPTIM5EN` writer - LPTIM5 peripheral clocks enable
pub use SBSEN_W as LPTIM5EN_W;
///Field `VREFEN` writer - VREF peripheral clock enable
pub use SBSEN_W as VREFEN_W;
///Field `RTCAPBEN` writer - RTC APB clock enable
pub use SBSEN_W as RTCAPBEN_W;
///Field `DTSEN` writer - Temperature Sensor peripheral clock enable
pub use SBSEN_W as DTSEN_W;
impl R {
    ///Bit 1 - SBS peripheral clock enable
    #[inline(always)]
    pub fn sbsen(&self) -> SBSEN_R {
        SBSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - LPUART1 peripheral clocks enable
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - SPI/I2S6 peripheral clocks enable
    #[inline(always)]
    pub fn spi6en(&self) -> SPI6EN_R {
        SPI6EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - LPTIM2 peripheral clocks enable
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - LPTIM3 peripheral clocks enable
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - LPTIM4 peripheral clocks enable
    #[inline(always)]
    pub fn lptim4en(&self) -> LPTIM4EN_R {
        LPTIM4EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM5 peripheral clocks enable
    #[inline(always)]
    pub fn lptim5en(&self) -> LPTIM5EN_R {
        LPTIM5EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - VREF peripheral clock enable
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - RTC APB clock enable
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 26 - Temperature Sensor peripheral clock enable
    #[inline(always)]
    pub fn dtsen(&self) -> DTSEN_R {
        DTSEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB4ENR")
            .field("sbsen", &self.sbsen())
            .field("lpuart1en", &self.lpuart1en())
            .field("spi6en", &self.spi6en())
            .field("lptim2en", &self.lptim2en())
            .field("lptim3en", &self.lptim3en())
            .field("lptim4en", &self.lptim4en())
            .field("lptim5en", &self.lptim5en())
            .field("vrefen", &self.vrefen())
            .field("rtcapben", &self.rtcapben())
            .field("dtsen", &self.dtsen())
            .finish()
    }
}
impl W {
    ///Bit 1 - SBS peripheral clock enable
    #[inline(always)]
    pub fn sbsen(&mut self) -> SBSEN_W<'_, APB4ENRrs> {
        SBSEN_W::new(self, 1)
    }
    ///Bit 3 - LPUART1 peripheral clocks enable
    #[inline(always)]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<'_, APB4ENRrs> {
        LPUART1EN_W::new(self, 3)
    }
    ///Bit 5 - SPI/I2S6 peripheral clocks enable
    #[inline(always)]
    pub fn spi6en(&mut self) -> SPI6EN_W<'_, APB4ENRrs> {
        SPI6EN_W::new(self, 5)
    }
    ///Bit 9 - LPTIM2 peripheral clocks enable
    #[inline(always)]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<'_, APB4ENRrs> {
        LPTIM2EN_W::new(self, 9)
    }
    ///Bit 10 - LPTIM3 peripheral clocks enable
    #[inline(always)]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W<'_, APB4ENRrs> {
        LPTIM3EN_W::new(self, 10)
    }
    ///Bit 11 - LPTIM4 peripheral clocks enable
    #[inline(always)]
    pub fn lptim4en(&mut self) -> LPTIM4EN_W<'_, APB4ENRrs> {
        LPTIM4EN_W::new(self, 11)
    }
    ///Bit 12 - LPTIM5 peripheral clocks enable
    #[inline(always)]
    pub fn lptim5en(&mut self) -> LPTIM5EN_W<'_, APB4ENRrs> {
        LPTIM5EN_W::new(self, 12)
    }
    ///Bit 15 - VREF peripheral clock enable
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W<'_, APB4ENRrs> {
        VREFEN_W::new(self, 15)
    }
    ///Bit 16 - RTC APB clock enable
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<'_, APB4ENRrs> {
        RTCAPBEN_W::new(self, 16)
    }
    ///Bit 26 - Temperature Sensor peripheral clock enable
    #[inline(always)]
    pub fn dtsen(&mut self) -> DTSEN_W<'_, APB4ENRrs> {
        DTSEN_W::new(self, 26)
    }
}
/**RCC APB4 clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb4enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RCC:APB4ENR)*/
pub struct APB4ENRrs;
impl crate::RegisterSpec for APB4ENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb4enr::R`](R) reader structure
impl crate::Readable for APB4ENRrs {}
///`write(|w| ..)` method takes [`apb4enr::W`](W) writer structure
impl crate::Writable for APB4ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4ENR to value 0x0001_0000
impl crate::Resettable for APB4ENRrs {
    const RESET_VALUE: u32 = 0x0001_0000;
}
