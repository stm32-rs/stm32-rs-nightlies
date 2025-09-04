///Register `C2AHB2SMENR` reader
pub type R = crate::R<C2AHB2SMENRrs>;
///Register `C2AHB2SMENR` writer
pub type W = crate::W<C2AHB2SMENRrs>;
/**CPU2 IO port A clocks enable during Sleep and Stop modes

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOASMEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<GPIOASMEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOASMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOASMEN` reader - CPU2 IO port A clocks enable during Sleep and Stop modes
pub type GPIOASMEN_R = crate::BitReader<GPIOASMEN>;
impl GPIOASMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOASMEN {
        match self.bits {
            false => GPIOASMEN::Disabled,
            true => GPIOASMEN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOASMEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOASMEN::Enabled
    }
}
///Field `GPIOASMEN` writer - CPU2 IO port A clocks enable during Sleep and Stop modes
pub type GPIOASMEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOASMEN>;
impl<'a, REG> GPIOASMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOASMEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOASMEN::Enabled)
    }
}
///Field `GPIOBSMEN` reader - CPU2 IO port B clocks enable during Sleep and Stop modes
pub use GPIOASMEN_R as GPIOBSMEN_R;
///Field `GPIOCSMEN` reader - CPU2 IO port C clocks enable during Sleep and Stop modes
pub use GPIOASMEN_R as GPIOCSMEN_R;
///Field `GPIODSMEN` reader - CPU2 IO port D clocks enable during Sleep and Stop modes
pub use GPIOASMEN_R as GPIODSMEN_R;
///Field `GPIOESMEN` reader - CPU2 IO port E clocks enable during Sleep and Stop modes
pub use GPIOASMEN_R as GPIOESMEN_R;
///Field `GPIOHSMEN` reader - CPU2 IO port H clocks enable during Sleep and Stop modes
pub use GPIOASMEN_R as GPIOHSMEN_R;
///Field `ADCFSSMEN` reader - CPU2 ADC clocks enable during Sleep and Stop modes
pub use GPIOASMEN_R as ADCFSSMEN_R;
///Field `AES1SMEN` reader - CPU2 AES1 accelerator clocks enable during Sleep and Stop modes
pub use GPIOASMEN_R as AES1SMEN_R;
///Field `GPIOBSMEN` writer - CPU2 IO port B clocks enable during Sleep and Stop modes
pub use GPIOASMEN_W as GPIOBSMEN_W;
///Field `GPIOCSMEN` writer - CPU2 IO port C clocks enable during Sleep and Stop modes
pub use GPIOASMEN_W as GPIOCSMEN_W;
///Field `GPIODSMEN` writer - CPU2 IO port D clocks enable during Sleep and Stop modes
pub use GPIOASMEN_W as GPIODSMEN_W;
///Field `GPIOESMEN` writer - CPU2 IO port E clocks enable during Sleep and Stop modes
pub use GPIOASMEN_W as GPIOESMEN_W;
///Field `GPIOHSMEN` writer - CPU2 IO port H clocks enable during Sleep and Stop modes
pub use GPIOASMEN_W as GPIOHSMEN_W;
///Field `ADCFSSMEN` writer - CPU2 ADC clocks enable during Sleep and Stop modes
pub use GPIOASMEN_W as ADCFSSMEN_W;
///Field `AES1SMEN` writer - CPU2 AES1 accelerator clocks enable during Sleep and Stop modes
pub use GPIOASMEN_W as AES1SMEN_W;
impl R {
    ///Bit 0 - CPU2 IO port A clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU2 IO port B clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CPU2 IO port C clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CPU2 IO port D clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CPU2 IO port E clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioesmen(&self) -> GPIOESMEN_R {
        GPIOESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - CPU2 IO port H clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiohsmen(&self) -> GPIOHSMEN_R {
        GPIOHSMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 13 - CPU2 ADC clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn adcfssmen(&self) -> ADCFSSMEN_R {
        ADCFSSMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - CPU2 AES1 accelerator clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn aes1smen(&self) -> AES1SMEN_R {
        AES1SMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2AHB2SMENR")
            .field("gpioasmen", &self.gpioasmen())
            .field("aes1smen", &self.aes1smen())
            .field("adcfssmen", &self.adcfssmen())
            .field("gpiohsmen", &self.gpiohsmen())
            .field("gpioesmen", &self.gpioesmen())
            .field("gpiodsmen", &self.gpiodsmen())
            .field("gpiocsmen", &self.gpiocsmen())
            .field("gpiobsmen", &self.gpiobsmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - CPU2 IO port A clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<C2AHB2SMENRrs> {
        GPIOASMEN_W::new(self, 0)
    }
    ///Bit 1 - CPU2 IO port B clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<C2AHB2SMENRrs> {
        GPIOBSMEN_W::new(self, 1)
    }
    ///Bit 2 - CPU2 IO port C clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<C2AHB2SMENRrs> {
        GPIOCSMEN_W::new(self, 2)
    }
    ///Bit 3 - CPU2 IO port D clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiodsmen(&mut self) -> GPIODSMEN_W<C2AHB2SMENRrs> {
        GPIODSMEN_W::new(self, 3)
    }
    ///Bit 4 - CPU2 IO port E clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioesmen(&mut self) -> GPIOESMEN_W<C2AHB2SMENRrs> {
        GPIOESMEN_W::new(self, 4)
    }
    ///Bit 7 - CPU2 IO port H clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiohsmen(&mut self) -> GPIOHSMEN_W<C2AHB2SMENRrs> {
        GPIOHSMEN_W::new(self, 7)
    }
    ///Bit 13 - CPU2 ADC clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn adcfssmen(&mut self) -> ADCFSSMEN_W<C2AHB2SMENRrs> {
        ADCFSSMEN_W::new(self, 13)
    }
    ///Bit 16 - CPU2 AES1 accelerator clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn aes1smen(&mut self) -> AES1SMEN_W<C2AHB2SMENRrs> {
        AES1SMEN_W::new(self, 16)
    }
}
/**CPU2 AHB2 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`c2ahb2smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2ahb2smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:C2AHB2SMENR)*/
pub struct C2AHB2SMENRrs;
impl crate::RegisterSpec for C2AHB2SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`c2ahb2smenr::R`](R) reader structure
impl crate::Readable for C2AHB2SMENRrs {}
///`write(|w| ..)` method takes [`c2ahb2smenr::W`](W) writer structure
impl crate::Writable for C2AHB2SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2AHB2SMENR to value 0x0001_209f
impl crate::Resettable for C2AHB2SMENRrs {
    const RESET_VALUE: u32 = 0x0001_209f;
}
