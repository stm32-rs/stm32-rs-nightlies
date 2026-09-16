///Register `IOPSMENR` reader
pub type R = crate::R<IOPSMENRrs>;
///Register `IOPSMENR` writer
pub type W = crate::W<IOPSMENRrs>;
/**I/O port A clock enable during Sleep mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOASMEN {
    ///0: Peripheral disabled (typically saves power)
    Disabled = 0,
    ///1: Peripheral enabled
    Enabled = 1,
}
impl From<GPIOASMEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOASMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOASMEN` reader - I/O port A clock enable during Sleep mode
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
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOASMEN::Disabled
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOASMEN::Enabled
    }
}
///Field `GPIOASMEN` writer - I/O port A clock enable during Sleep mode
pub type GPIOASMEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOASMEN>;
impl<'a, REG> GPIOASMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOASMEN::Disabled)
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOASMEN::Enabled)
    }
}
///Field `GPIOBSMEN` reader - I/O port B clock enable during Sleep mode
pub use GPIOASMEN_R as GPIOBSMEN_R;
///Field `GPIOCSMEN` reader - I/O port C clock enable during Sleep mode
pub use GPIOASMEN_R as GPIOCSMEN_R;
///Field `GPIODSMEN` reader - I/O port D clock enable during Sleep mode
pub use GPIOASMEN_R as GPIODSMEN_R;
///Field `GPIOESMEN` reader - I/O port E clock enable during Sleep mode
pub use GPIOASMEN_R as GPIOESMEN_R;
///Field `GPIOFSMEN` reader - I/O port F clock enable during Sleep mode
pub use GPIOASMEN_R as GPIOFSMEN_R;
///Field `GPIOBSMEN` writer - I/O port B clock enable during Sleep mode
pub use GPIOASMEN_W as GPIOBSMEN_W;
///Field `GPIOCSMEN` writer - I/O port C clock enable during Sleep mode
pub use GPIOASMEN_W as GPIOCSMEN_W;
///Field `GPIODSMEN` writer - I/O port D clock enable during Sleep mode
pub use GPIOASMEN_W as GPIODSMEN_W;
///Field `GPIOESMEN` writer - I/O port E clock enable during Sleep mode
pub use GPIOASMEN_W as GPIOESMEN_W;
///Field `GPIOFSMEN` writer - I/O port F clock enable during Sleep mode
pub use GPIOASMEN_W as GPIOFSMEN_W;
impl R {
    ///Bit 0 - I/O port A clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I/O port B clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I/O port C clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O port D clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - I/O port E clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioesmen(&self) -> GPIOESMEN_R {
        GPIOESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - I/O port F clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiofsmen(&self) -> GPIOFSMEN_R {
        GPIOFSMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOPSMENR")
            .field("gpioasmen", &self.gpioasmen())
            .field("gpiobsmen", &self.gpiobsmen())
            .field("gpiocsmen", &self.gpiocsmen())
            .field("gpiodsmen", &self.gpiodsmen())
            .field("gpioesmen", &self.gpioesmen())
            .field("gpiofsmen", &self.gpiofsmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - I/O port A clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<'_, IOPSMENRrs> {
        GPIOASMEN_W::new(self, 0)
    }
    ///Bit 1 - I/O port B clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<'_, IOPSMENRrs> {
        GPIOBSMEN_W::new(self, 1)
    }
    ///Bit 2 - I/O port C clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<'_, IOPSMENRrs> {
        GPIOCSMEN_W::new(self, 2)
    }
    ///Bit 3 - I/O port D clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiodsmen(&mut self) -> GPIODSMEN_W<'_, IOPSMENRrs> {
        GPIODSMEN_W::new(self, 3)
    }
    ///Bit 4 - I/O port E clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioesmen(&mut self) -> GPIOESMEN_W<'_, IOPSMENRrs> {
        GPIOESMEN_W::new(self, 4)
    }
    ///Bit 5 - I/O port F clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiofsmen(&mut self) -> GPIOFSMEN_W<'_, IOPSMENRrs> {
        GPIOFSMEN_W::new(self, 5)
    }
}
/**GPIO in Sleep mode clock enable register

You can [`read`](crate::Reg::read) this register and get [`iopsmenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopsmenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G030.html#RCC:IOPSMENR)*/
pub struct IOPSMENRrs;
impl crate::RegisterSpec for IOPSMENRrs {
    type Ux = u32;
}
///`read()` method returns [`iopsmenr::R`](R) reader structure
impl crate::Readable for IOPSMENRrs {}
///`write(|w| ..)` method takes [`iopsmenr::W`](W) writer structure
impl crate::Writable for IOPSMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IOPSMENR to value 0x3f
impl crate::Resettable for IOPSMENRrs {
    const RESET_VALUE: u32 = 0x3f;
}
