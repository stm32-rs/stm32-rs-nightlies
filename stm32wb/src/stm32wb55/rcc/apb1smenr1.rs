///Register `APB1SMENR1` reader
pub type R = crate::R<APB1SMENR1rs>;
///Register `APB1SMENR1` writer
pub type W = crate::W<APB1SMENR1rs>;
/**TIM2 timer clocks enable during CPU1 Sleep mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2SMEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<TIM2SMEN> for bool {
    #[inline(always)]
    fn from(variant: TIM2SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2SMEN` reader - TIM2 timer clocks enable during CPU1 Sleep mode
pub type TIM2SMEN_R = crate::BitReader<TIM2SMEN>;
impl TIM2SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM2SMEN {
        match self.bits {
            false => TIM2SMEN::Disabled,
            true => TIM2SMEN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2SMEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2SMEN::Enabled
    }
}
///Field `TIM2SMEN` writer - TIM2 timer clocks enable during CPU1 Sleep mode
pub type TIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2SMEN>;
impl<'a, REG> TIM2SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2SMEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2SMEN::Enabled)
    }
}
///Field `LCDSMEN` reader - LCD clocks enable during CPU1 Sleep mode
pub use TIM2SMEN_R as LCDSMEN_R;
///Field `RTCAPBSMEN` reader - RTC APB clocks enable during CPU1 Sleep mode
pub use TIM2SMEN_R as RTCAPBSMEN_R;
///Field `WWDGSMEN` reader - Window watchdog clocks enable during CPU1 Sleep mode
pub use TIM2SMEN_R as WWDGSMEN_R;
///Field `SPI2SMEN` reader - SPI2 clocks enable during CPU1 Sleep mode
pub use TIM2SMEN_R as SPI2SMEN_R;
///Field `I2C1SMEN` reader - I2C1 clocks enable during CPU1 Sleep mode
pub use TIM2SMEN_R as I2C1SMEN_R;
///Field `I2C3SMEN` reader - I2C3 clocks enable during CPU1 Sleep mode
pub use TIM2SMEN_R as I2C3SMEN_R;
///Field `CRSMEN` reader - CRS clocks enable during CPU1 Sleep mode
pub use TIM2SMEN_R as CRSMEN_R;
///Field `USBSMEN` reader - USB FS clocks enable during CPU1 Sleep mode
pub use TIM2SMEN_R as USBSMEN_R;
///Field `LPTIM1SMEN` reader - Low power timer 1 clocks enable during CPU1 Sleep mode
pub use TIM2SMEN_R as LPTIM1SMEN_R;
///Field `LCDSMEN` writer - LCD clocks enable during CPU1 Sleep mode
pub use TIM2SMEN_W as LCDSMEN_W;
///Field `RTCAPBSMEN` writer - RTC APB clocks enable during CPU1 Sleep mode
pub use TIM2SMEN_W as RTCAPBSMEN_W;
///Field `WWDGSMEN` writer - Window watchdog clocks enable during CPU1 Sleep mode
pub use TIM2SMEN_W as WWDGSMEN_W;
///Field `SPI2SMEN` writer - SPI2 clocks enable during CPU1 Sleep mode
pub use TIM2SMEN_W as SPI2SMEN_W;
///Field `I2C1SMEN` writer - I2C1 clocks enable during CPU1 Sleep mode
pub use TIM2SMEN_W as I2C1SMEN_W;
///Field `I2C3SMEN` writer - I2C3 clocks enable during CPU1 Sleep mode
pub use TIM2SMEN_W as I2C3SMEN_W;
///Field `CRSMEN` writer - CRS clocks enable during CPU1 Sleep mode
pub use TIM2SMEN_W as CRSMEN_W;
///Field `USBSMEN` writer - USB FS clocks enable during CPU1 Sleep mode
pub use TIM2SMEN_W as USBSMEN_W;
///Field `LPTIM1SMEN` writer - Low power timer 1 clocks enable during CPU1 Sleep mode
pub use TIM2SMEN_W as LPTIM1SMEN_W;
impl R {
    ///Bit 0 - TIM2 timer clocks enable during CPU1 Sleep mode
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 9 - LCD clocks enable during CPU1 Sleep mode
    #[inline(always)]
    pub fn lcdsmen(&self) -> LCDSMEN_R {
        LCDSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RTC APB clocks enable during CPU1 Sleep mode
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Window watchdog clocks enable during CPU1 Sleep mode
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2 clocks enable during CPU1 Sleep mode
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 21 - I2C1 clocks enable during CPU1 Sleep mode
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - I2C3 clocks enable during CPU1 Sleep mode
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CRS clocks enable during CPU1 Sleep mode
    #[inline(always)]
    pub fn crsmen(&self) -> CRSMEN_R {
        CRSMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - USB FS clocks enable during CPU1 Sleep mode
    #[inline(always)]
    pub fn usbsmen(&self) -> USBSMEN_R {
        USBSMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 31 - Low power timer 1 clocks enable during CPU1 Sleep mode
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1SMENR1")
            .field("tim2smen", &self.tim2smen())
            .field("lptim1smen", &self.lptim1smen())
            .field("usbsmen", &self.usbsmen())
            .field("crsmen", &self.crsmen())
            .field("i2c3smen", &self.i2c3smen())
            .field("i2c1smen", &self.i2c1smen())
            .field("spi2smen", &self.spi2smen())
            .field("wwdgsmen", &self.wwdgsmen())
            .field("rtcapbsmen", &self.rtcapbsmen())
            .field("lcdsmen", &self.lcdsmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 timer clocks enable during CPU1 Sleep mode
    #[inline(always)]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<'_, APB1SMENR1rs> {
        TIM2SMEN_W::new(self, 0)
    }
    ///Bit 9 - LCD clocks enable during CPU1 Sleep mode
    #[inline(always)]
    pub fn lcdsmen(&mut self) -> LCDSMEN_W<'_, APB1SMENR1rs> {
        LCDSMEN_W::new(self, 9)
    }
    ///Bit 10 - RTC APB clocks enable during CPU1 Sleep mode
    #[inline(always)]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<'_, APB1SMENR1rs> {
        RTCAPBSMEN_W::new(self, 10)
    }
    ///Bit 11 - Window watchdog clocks enable during CPU1 Sleep mode
    #[inline(always)]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<'_, APB1SMENR1rs> {
        WWDGSMEN_W::new(self, 11)
    }
    ///Bit 14 - SPI2 clocks enable during CPU1 Sleep mode
    #[inline(always)]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W<'_, APB1SMENR1rs> {
        SPI2SMEN_W::new(self, 14)
    }
    ///Bit 21 - I2C1 clocks enable during CPU1 Sleep mode
    #[inline(always)]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<'_, APB1SMENR1rs> {
        I2C1SMEN_W::new(self, 21)
    }
    ///Bit 23 - I2C3 clocks enable during CPU1 Sleep mode
    #[inline(always)]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W<'_, APB1SMENR1rs> {
        I2C3SMEN_W::new(self, 23)
    }
    ///Bit 24 - CRS clocks enable during CPU1 Sleep mode
    #[inline(always)]
    pub fn crsmen(&mut self) -> CRSMEN_W<'_, APB1SMENR1rs> {
        CRSMEN_W::new(self, 24)
    }
    ///Bit 26 - USB FS clocks enable during CPU1 Sleep mode
    #[inline(always)]
    pub fn usbsmen(&mut self) -> USBSMEN_W<'_, APB1SMENR1rs> {
        USBSMEN_W::new(self, 26)
    }
    ///Bit 31 - Low power timer 1 clocks enable during CPU1 Sleep mode
    #[inline(always)]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W<'_, APB1SMENR1rs> {
        LPTIM1SMEN_W::new(self, 31)
    }
}
/**APB1SMENR1

You can [`read`](crate::Reg::read) this register and get [`apb1smenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:APB1SMENR1)*/
pub struct APB1SMENR1rs;
impl crate::RegisterSpec for APB1SMENR1rs {
    type Ux = u32;
}
///`read()` method returns [`apb1smenr1::R`](R) reader structure
impl crate::Readable for APB1SMENR1rs {}
///`write(|w| ..)` method takes [`apb1smenr1::W`](W) writer structure
impl crate::Writable for APB1SMENR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1SMENR1 to value 0x85a0_4e01
impl crate::Resettable for APB1SMENR1rs {
    const RESET_VALUE: u32 = 0x85a0_4e01;
}
