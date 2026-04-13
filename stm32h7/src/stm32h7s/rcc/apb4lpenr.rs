///Register `APB4LPENR` reader
pub type R = crate::R<APB4LPENRrs>;
///Register `APB4LPENR` writer
pub type W = crate::W<APB4LPENRrs>;
/**SBS peripheral clock enable in low-power mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBSLPEN {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<SBSLPEN> for bool {
    #[inline(always)]
    fn from(variant: SBSLPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SBSLPEN` reader - SBS peripheral clock enable in low-power mode
pub type SBSLPEN_R = crate::BitReader<SBSLPEN>;
impl SBSLPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SBSLPEN {
        match self.bits {
            false => SBSLPEN::Disabled,
            true => SBSLPEN::Enabled,
        }
    }
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SBSLPEN::Disabled
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SBSLPEN::Enabled
    }
}
///Field `SBSLPEN` writer - SBS peripheral clock enable in low-power mode
pub type SBSLPEN_W<'a, REG> = crate::BitWriter<'a, REG, SBSLPEN>;
impl<'a, REG> SBSLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SBSLPEN::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SBSLPEN::Enabled)
    }
}
///Field `LPUART1LPEN` reader - LPUART1 peripheral clocks enable in low-power mode
pub use SBSLPEN_R as LPUART1LPEN_R;
///Field `SPI6LPEN` reader - SPI/I2S6 peripheral clocks enable in low-power mode
pub use SBSLPEN_R as SPI6LPEN_R;
///Field `LPTIM2LPEN` reader - LPTIM2 peripheral clocks enable in low-power mode
pub use SBSLPEN_R as LPTIM2LPEN_R;
///Field `LPTIM3LPEN` reader - LPTIM3 peripheral clocks enable in low-power mode
pub use SBSLPEN_R as LPTIM3LPEN_R;
///Field `LPTIM4LPEN` reader - LPTIM4 peripheral clocks enable in low-power mode
pub use SBSLPEN_R as LPTIM4LPEN_R;
///Field `LPTIM5LPEN` reader - LPTIM5 peripheral clocks enable in low-power mode
pub use SBSLPEN_R as LPTIM5LPEN_R;
///Field `VREFLPEN` reader - VREF peripheral clock enable in low-power mode
pub use SBSLPEN_R as VREFLPEN_R;
///Field `RTCAPBLPEN` reader - RTC APB clock enable in low-power mode
pub use SBSLPEN_R as RTCAPBLPEN_R;
///Field `DTSLPEN` reader - temperature sensor peripheral clock enable in low-power mode
pub use SBSLPEN_R as DTSLPEN_R;
///Field `LPUART1LPEN` writer - LPUART1 peripheral clocks enable in low-power mode
pub use SBSLPEN_W as LPUART1LPEN_W;
///Field `SPI6LPEN` writer - SPI/I2S6 peripheral clocks enable in low-power mode
pub use SBSLPEN_W as SPI6LPEN_W;
///Field `LPTIM2LPEN` writer - LPTIM2 peripheral clocks enable in low-power mode
pub use SBSLPEN_W as LPTIM2LPEN_W;
///Field `LPTIM3LPEN` writer - LPTIM3 peripheral clocks enable in low-power mode
pub use SBSLPEN_W as LPTIM3LPEN_W;
///Field `LPTIM4LPEN` writer - LPTIM4 peripheral clocks enable in low-power mode
pub use SBSLPEN_W as LPTIM4LPEN_W;
///Field `LPTIM5LPEN` writer - LPTIM5 peripheral clocks enable in low-power mode
pub use SBSLPEN_W as LPTIM5LPEN_W;
///Field `VREFLPEN` writer - VREF peripheral clock enable in low-power mode
pub use SBSLPEN_W as VREFLPEN_W;
///Field `RTCAPBLPEN` writer - RTC APB clock enable in low-power mode
pub use SBSLPEN_W as RTCAPBLPEN_W;
///Field `DTSLPEN` writer - temperature sensor peripheral clock enable in low-power mode
pub use SBSLPEN_W as DTSLPEN_W;
impl R {
    ///Bit 1 - SBS peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn sbslpen(&self) -> SBSLPEN_R {
        SBSLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - LPUART1 peripheral clocks enable in low-power mode
    #[inline(always)]
    pub fn lpuart1lpen(&self) -> LPUART1LPEN_R {
        LPUART1LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - SPI/I2S6 peripheral clocks enable in low-power mode
    #[inline(always)]
    pub fn spi6lpen(&self) -> SPI6LPEN_R {
        SPI6LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - LPTIM2 peripheral clocks enable in low-power mode
    #[inline(always)]
    pub fn lptim2lpen(&self) -> LPTIM2LPEN_R {
        LPTIM2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - LPTIM3 peripheral clocks enable in low-power mode
    #[inline(always)]
    pub fn lptim3lpen(&self) -> LPTIM3LPEN_R {
        LPTIM3LPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - LPTIM4 peripheral clocks enable in low-power mode
    #[inline(always)]
    pub fn lptim4lpen(&self) -> LPTIM4LPEN_R {
        LPTIM4LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM5 peripheral clocks enable in low-power mode
    #[inline(always)]
    pub fn lptim5lpen(&self) -> LPTIM5LPEN_R {
        LPTIM5LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - VREF peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn vreflpen(&self) -> VREFLPEN_R {
        VREFLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - RTC APB clock enable in low-power mode
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 26 - temperature sensor peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn dtslpen(&self) -> DTSLPEN_R {
        DTSLPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB4LPENR")
            .field("sbslpen", &self.sbslpen())
            .field("lpuart1lpen", &self.lpuart1lpen())
            .field("spi6lpen", &self.spi6lpen())
            .field("lptim2lpen", &self.lptim2lpen())
            .field("lptim3lpen", &self.lptim3lpen())
            .field("lptim4lpen", &self.lptim4lpen())
            .field("lptim5lpen", &self.lptim5lpen())
            .field("vreflpen", &self.vreflpen())
            .field("rtcapblpen", &self.rtcapblpen())
            .field("dtslpen", &self.dtslpen())
            .finish()
    }
}
impl W {
    ///Bit 1 - SBS peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn sbslpen(&mut self) -> SBSLPEN_W<'_, APB4LPENRrs> {
        SBSLPEN_W::new(self, 1)
    }
    ///Bit 3 - LPUART1 peripheral clocks enable in low-power mode
    #[inline(always)]
    pub fn lpuart1lpen(&mut self) -> LPUART1LPEN_W<'_, APB4LPENRrs> {
        LPUART1LPEN_W::new(self, 3)
    }
    ///Bit 5 - SPI/I2S6 peripheral clocks enable in low-power mode
    #[inline(always)]
    pub fn spi6lpen(&mut self) -> SPI6LPEN_W<'_, APB4LPENRrs> {
        SPI6LPEN_W::new(self, 5)
    }
    ///Bit 9 - LPTIM2 peripheral clocks enable in low-power mode
    #[inline(always)]
    pub fn lptim2lpen(&mut self) -> LPTIM2LPEN_W<'_, APB4LPENRrs> {
        LPTIM2LPEN_W::new(self, 9)
    }
    ///Bit 10 - LPTIM3 peripheral clocks enable in low-power mode
    #[inline(always)]
    pub fn lptim3lpen(&mut self) -> LPTIM3LPEN_W<'_, APB4LPENRrs> {
        LPTIM3LPEN_W::new(self, 10)
    }
    ///Bit 11 - LPTIM4 peripheral clocks enable in low-power mode
    #[inline(always)]
    pub fn lptim4lpen(&mut self) -> LPTIM4LPEN_W<'_, APB4LPENRrs> {
        LPTIM4LPEN_W::new(self, 11)
    }
    ///Bit 12 - LPTIM5 peripheral clocks enable in low-power mode
    #[inline(always)]
    pub fn lptim5lpen(&mut self) -> LPTIM5LPEN_W<'_, APB4LPENRrs> {
        LPTIM5LPEN_W::new(self, 12)
    }
    ///Bit 15 - VREF peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn vreflpen(&mut self) -> VREFLPEN_W<'_, APB4LPENRrs> {
        VREFLPEN_W::new(self, 15)
    }
    ///Bit 16 - RTC APB clock enable in low-power mode
    #[inline(always)]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W<'_, APB4LPENRrs> {
        RTCAPBLPEN_W::new(self, 16)
    }
    ///Bit 26 - temperature sensor peripheral clock enable in low-power mode
    #[inline(always)]
    pub fn dtslpen(&mut self) -> DTSLPEN_W<'_, APB4LPENRrs> {
        DTSLPEN_W::new(self, 26)
    }
}
/**RCC APB4 low-power clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb4lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RCC:APB4LPENR)*/
pub struct APB4LPENRrs;
impl crate::RegisterSpec for APB4LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb4lpenr::R`](R) reader structure
impl crate::Readable for APB4LPENRrs {}
///`write(|w| ..)` method takes [`apb4lpenr::W`](W) writer structure
impl crate::Writable for APB4LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4LPENR to value 0x0401_9e2a
impl crate::Resettable for APB4LPENRrs {
    const RESET_VALUE: u32 = 0x0401_9e2a;
}
