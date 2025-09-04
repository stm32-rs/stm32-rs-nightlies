///Register `C2APB1ENR1` reader
pub type R = crate::R<C2APB1ENR1rs>;
///Register `C2APB1ENR1` writer
pub type W = crate::W<C2APB1ENR1rs>;
/**CPU2 TIM2 timer clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2EN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<TIM2EN> for bool {
    #[inline(always)]
    fn from(variant: TIM2EN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2EN` reader - CPU2 TIM2 timer clock enable
pub type TIM2EN_R = crate::BitReader<TIM2EN>;
impl TIM2EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM2EN {
        match self.bits {
            false => TIM2EN::Disabled,
            true => TIM2EN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2EN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2EN::Enabled
    }
}
///Field `TIM2EN` writer - CPU2 TIM2 timer clock enable
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2EN>;
impl<'a, REG> TIM2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN::Enabled)
    }
}
///Field `LCDEN` reader - CPU2 LCD clock enable
pub use TIM2EN_R as LCDEN_R;
///Field `RTCAPBEN` reader - CPU2 RTC APB clock enable
pub use TIM2EN_R as RTCAPBEN_R;
///Field `SPI2EN` reader - CPU2 SPI2 clock enable
pub use TIM2EN_R as SPI2EN_R;
///Field `I2C1EN` reader - CPU2 I2C1 clock enable
pub use TIM2EN_R as I2C1EN_R;
///Field `I2C3EN` reader - CPU2 I2C3 clock enable
pub use TIM2EN_R as I2C3EN_R;
///Field `CRSEN` reader - CPU2 CRS clock enable
pub use TIM2EN_R as CRSEN_R;
///Field `USBEN` reader - CPU2 USB clock enable
pub use TIM2EN_R as USBEN_R;
///Field `LPTIM1EN` reader - CPU2 Low power timer 1 clock enable
pub use TIM2EN_R as LPTIM1EN_R;
///Field `LCDEN` writer - CPU2 LCD clock enable
pub use TIM2EN_W as LCDEN_W;
///Field `RTCAPBEN` writer - CPU2 RTC APB clock enable
pub use TIM2EN_W as RTCAPBEN_W;
///Field `SPI2EN` writer - CPU2 SPI2 clock enable
pub use TIM2EN_W as SPI2EN_W;
///Field `I2C1EN` writer - CPU2 I2C1 clock enable
pub use TIM2EN_W as I2C1EN_W;
///Field `I2C3EN` writer - CPU2 I2C3 clock enable
pub use TIM2EN_W as I2C3EN_W;
///Field `CRSEN` writer - CPU2 CRS clock enable
pub use TIM2EN_W as CRSEN_W;
///Field `USBEN` writer - CPU2 USB clock enable
pub use TIM2EN_W as USBEN_W;
///Field `LPTIM1EN` writer - CPU2 Low power timer 1 clock enable
pub use TIM2EN_W as LPTIM1EN_W;
impl R {
    ///Bit 0 - CPU2 TIM2 timer clock enable
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 9 - CPU2 LCD clock enable
    #[inline(always)]
    pub fn lcden(&self) -> LCDEN_R {
        LCDEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CPU2 RTC APB clock enable
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 14 - CPU2 SPI2 clock enable
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 21 - CPU2 I2C1 clock enable
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - CPU2 I2C3 clock enable
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CPU2 CRS clock enable
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - CPU2 USB clock enable
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 31 - CPU2 Low power timer 1 clock enable
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2APB1ENR1")
            .field("tim2en", &self.tim2en())
            .field("lptim1en", &self.lptim1en())
            .field("usben", &self.usben())
            .field("crsen", &self.crsen())
            .field("i2c3en", &self.i2c3en())
            .field("i2c1en", &self.i2c1en())
            .field("spi2en", &self.spi2en())
            .field("rtcapben", &self.rtcapben())
            .field("lcden", &self.lcden())
            .finish()
    }
}
impl W {
    ///Bit 0 - CPU2 TIM2 timer clock enable
    #[inline(always)]
    pub fn tim2en(&mut self) -> TIM2EN_W<C2APB1ENR1rs> {
        TIM2EN_W::new(self, 0)
    }
    ///Bit 9 - CPU2 LCD clock enable
    #[inline(always)]
    pub fn lcden(&mut self) -> LCDEN_W<C2APB1ENR1rs> {
        LCDEN_W::new(self, 9)
    }
    ///Bit 10 - CPU2 RTC APB clock enable
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<C2APB1ENR1rs> {
        RTCAPBEN_W::new(self, 10)
    }
    ///Bit 14 - CPU2 SPI2 clock enable
    #[inline(always)]
    pub fn spi2en(&mut self) -> SPI2EN_W<C2APB1ENR1rs> {
        SPI2EN_W::new(self, 14)
    }
    ///Bit 21 - CPU2 I2C1 clock enable
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W<C2APB1ENR1rs> {
        I2C1EN_W::new(self, 21)
    }
    ///Bit 23 - CPU2 I2C3 clock enable
    #[inline(always)]
    pub fn i2c3en(&mut self) -> I2C3EN_W<C2APB1ENR1rs> {
        I2C3EN_W::new(self, 23)
    }
    ///Bit 24 - CPU2 CRS clock enable
    #[inline(always)]
    pub fn crsen(&mut self) -> CRSEN_W<C2APB1ENR1rs> {
        CRSEN_W::new(self, 24)
    }
    ///Bit 26 - CPU2 USB clock enable
    #[inline(always)]
    pub fn usben(&mut self) -> USBEN_W<C2APB1ENR1rs> {
        USBEN_W::new(self, 26)
    }
    ///Bit 31 - CPU2 Low power timer 1 clock enable
    #[inline(always)]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<C2APB1ENR1rs> {
        LPTIM1EN_W::new(self, 31)
    }
}
/**CPU2 APB1ENR1

You can [`read`](crate::Reg::read) this register and get [`c2apb1enr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb1enr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:C2APB1ENR1)*/
pub struct C2APB1ENR1rs;
impl crate::RegisterSpec for C2APB1ENR1rs {
    type Ux = u32;
}
///`read()` method returns [`c2apb1enr1::R`](R) reader structure
impl crate::Readable for C2APB1ENR1rs {}
///`write(|w| ..)` method takes [`c2apb1enr1::W`](W) writer structure
impl crate::Writable for C2APB1ENR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2APB1ENR1 to value 0x0400
impl crate::Resettable for C2APB1ENR1rs {
    const RESET_VALUE: u32 = 0x0400;
}
