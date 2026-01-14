///Register `APB3LPENR` reader
pub type R = crate::R<APB3LPENRrs>;
///Register `APB3LPENR` writer
pub type W = crate::W<APB3LPENRrs>;
/**SBS clock enable during Sleep mode

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
///Field `SBSLPEN` reader - SBS clock enable during Sleep mode
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
///Field `SBSLPEN` writer - SBS clock enable during Sleep mode
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
///Field `LPUART1LPEN` reader - LPUART1 clock enable during Sleep mode
pub use SBSLPEN_R as LPUART1LPEN_R;
///Field `I2C3LPEN` reader - I2C3 clock enable during Sleep mode
pub use SBSLPEN_R as I2C3LPEN_R;
///Field `I3C2LPEN` reader - I3C2 clock enable during Sleep mode
pub use SBSLPEN_R as I3C2LPEN_R;
///Field `LPTIM1LPEN` reader - LPTIM1 clock enable during Sleep mode
pub use SBSLPEN_R as LPTIM1LPEN_R;
///Field `LPTIM3LPEN` reader - LPTIM3 clock enable during Sleep mode
pub use SBSLPEN_R as LPTIM3LPEN_R;
///Field `LPTIM4LPEN` reader - LPTIM4 clock enable during Sleep mode
pub use SBSLPEN_R as LPTIM4LPEN_R;
///Field `LPTIM5LPEN` reader - LPTIM5 clock enable during Sleep mode
pub use SBSLPEN_R as LPTIM5LPEN_R;
///Field `LPTIM6LPEN` reader - LPTIM6 clock enable during Sleep mode
pub use SBSLPEN_R as LPTIM6LPEN_R;
///Field `VREFLPEN` reader - VREFBUF clock enable during Sleep mode
pub use SBSLPEN_R as VREFLPEN_R;
///Field `RTCAPBLPEN` reader - RTC APB interface clock enable during Sleep mode
pub use SBSLPEN_R as RTCAPBLPEN_R;
///Field `LPUART1LPEN` writer - LPUART1 clock enable during Sleep mode
pub use SBSLPEN_W as LPUART1LPEN_W;
///Field `I2C3LPEN` writer - I2C3 clock enable during Sleep mode
pub use SBSLPEN_W as I2C3LPEN_W;
///Field `I3C2LPEN` writer - I3C2 clock enable during Sleep mode
pub use SBSLPEN_W as I3C2LPEN_W;
///Field `LPTIM1LPEN` writer - LPTIM1 clock enable during Sleep mode
pub use SBSLPEN_W as LPTIM1LPEN_W;
///Field `LPTIM3LPEN` writer - LPTIM3 clock enable during Sleep mode
pub use SBSLPEN_W as LPTIM3LPEN_W;
///Field `LPTIM4LPEN` writer - LPTIM4 clock enable during Sleep mode
pub use SBSLPEN_W as LPTIM4LPEN_W;
///Field `LPTIM5LPEN` writer - LPTIM5 clock enable during Sleep mode
pub use SBSLPEN_W as LPTIM5LPEN_W;
///Field `LPTIM6LPEN` writer - LPTIM6 clock enable during Sleep mode
pub use SBSLPEN_W as LPTIM6LPEN_W;
///Field `VREFLPEN` writer - VREFBUF clock enable during Sleep mode
pub use SBSLPEN_W as VREFLPEN_W;
///Field `RTCAPBLPEN` writer - RTC APB interface clock enable during Sleep mode
pub use SBSLPEN_W as RTCAPBLPEN_W;
impl R {
    ///Bit 1 - SBS clock enable during Sleep mode
    #[inline(always)]
    pub fn sbslpen(&self) -> SBSLPEN_R {
        SBSLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 6 - LPUART1 clock enable during Sleep mode
    #[inline(always)]
    pub fn lpuart1lpen(&self) -> LPUART1LPEN_R {
        LPUART1LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C3 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c3lpen(&self) -> I2C3LPEN_R {
        I2C3LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - I3C2 clock enable during Sleep mode
    #[inline(always)]
    pub fn i3c2lpen(&self) -> I3C2LPEN_R {
        I3C2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - LPTIM1 clock enable during Sleep mode
    #[inline(always)]
    pub fn lptim1lpen(&self) -> LPTIM1LPEN_R {
        LPTIM1LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM3 clock enable during Sleep mode
    #[inline(always)]
    pub fn lptim3lpen(&self) -> LPTIM3LPEN_R {
        LPTIM3LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - LPTIM4 clock enable during Sleep mode
    #[inline(always)]
    pub fn lptim4lpen(&self) -> LPTIM4LPEN_R {
        LPTIM4LPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - LPTIM5 clock enable during Sleep mode
    #[inline(always)]
    pub fn lptim5lpen(&self) -> LPTIM5LPEN_R {
        LPTIM5LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - LPTIM6 clock enable during Sleep mode
    #[inline(always)]
    pub fn lptim6lpen(&self) -> LPTIM6LPEN_R {
        LPTIM6LPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 20 - VREFBUF clock enable during Sleep mode
    #[inline(always)]
    pub fn vreflpen(&self) -> VREFLPEN_R {
        VREFLPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - RTC APB interface clock enable during Sleep mode
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB3LPENR")
            .field("sbslpen", &self.sbslpen())
            .field("lpuart1lpen", &self.lpuart1lpen())
            .field("i2c3lpen", &self.i2c3lpen())
            .field("i3c2lpen", &self.i3c2lpen())
            .field("lptim1lpen", &self.lptim1lpen())
            .field("lptim3lpen", &self.lptim3lpen())
            .field("lptim4lpen", &self.lptim4lpen())
            .field("lptim5lpen", &self.lptim5lpen())
            .field("lptim6lpen", &self.lptim6lpen())
            .field("vreflpen", &self.vreflpen())
            .field("rtcapblpen", &self.rtcapblpen())
            .finish()
    }
}
impl W {
    ///Bit 1 - SBS clock enable during Sleep mode
    #[inline(always)]
    pub fn sbslpen(&mut self) -> SBSLPEN_W<'_, APB3LPENRrs> {
        SBSLPEN_W::new(self, 1)
    }
    ///Bit 6 - LPUART1 clock enable during Sleep mode
    #[inline(always)]
    pub fn lpuart1lpen(&mut self) -> LPUART1LPEN_W<'_, APB3LPENRrs> {
        LPUART1LPEN_W::new(self, 6)
    }
    ///Bit 7 - I2C3 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c3lpen(&mut self) -> I2C3LPEN_W<'_, APB3LPENRrs> {
        I2C3LPEN_W::new(self, 7)
    }
    ///Bit 9 - I3C2 clock enable during Sleep mode
    #[inline(always)]
    pub fn i3c2lpen(&mut self) -> I3C2LPEN_W<'_, APB3LPENRrs> {
        I3C2LPEN_W::new(self, 9)
    }
    ///Bit 11 - LPTIM1 clock enable during Sleep mode
    #[inline(always)]
    pub fn lptim1lpen(&mut self) -> LPTIM1LPEN_W<'_, APB3LPENRrs> {
        LPTIM1LPEN_W::new(self, 11)
    }
    ///Bit 12 - LPTIM3 clock enable during Sleep mode
    #[inline(always)]
    pub fn lptim3lpen(&mut self) -> LPTIM3LPEN_W<'_, APB3LPENRrs> {
        LPTIM3LPEN_W::new(self, 12)
    }
    ///Bit 13 - LPTIM4 clock enable during Sleep mode
    #[inline(always)]
    pub fn lptim4lpen(&mut self) -> LPTIM4LPEN_W<'_, APB3LPENRrs> {
        LPTIM4LPEN_W::new(self, 13)
    }
    ///Bit 14 - LPTIM5 clock enable during Sleep mode
    #[inline(always)]
    pub fn lptim5lpen(&mut self) -> LPTIM5LPEN_W<'_, APB3LPENRrs> {
        LPTIM5LPEN_W::new(self, 14)
    }
    ///Bit 15 - LPTIM6 clock enable during Sleep mode
    #[inline(always)]
    pub fn lptim6lpen(&mut self) -> LPTIM6LPEN_W<'_, APB3LPENRrs> {
        LPTIM6LPEN_W::new(self, 15)
    }
    ///Bit 20 - VREFBUF clock enable during Sleep mode
    #[inline(always)]
    pub fn vreflpen(&mut self) -> VREFLPEN_W<'_, APB3LPENRrs> {
        VREFLPEN_W::new(self, 20)
    }
    ///Bit 21 - RTC APB interface clock enable during Sleep mode
    #[inline(always)]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W<'_, APB3LPENRrs> {
        RTCAPBLPEN_W::new(self, 21)
    }
}
/**RCC APB3 sleep clock register

You can [`read`](crate::Reg::read) this register and get [`apb3lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#RCC:APB3LPENR)*/
pub struct APB3LPENRrs;
impl crate::RegisterSpec for APB3LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb3lpenr::R`](R) reader structure
impl crate::Readable for APB3LPENRrs {}
///`write(|w| ..)` method takes [`apb3lpenr::W`](W) writer structure
impl crate::Writable for APB3LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3LPENR to value 0x0030_fae2
impl crate::Resettable for APB3LPENRrs {
    const RESET_VALUE: u32 = 0x0030_fae2;
}
