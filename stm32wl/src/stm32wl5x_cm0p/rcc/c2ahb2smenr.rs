///Register `C2AHB2SMENR` reader
pub type R = crate::R<C2AHB2SMENRrs>;
///Register `C2AHB2SMENR` writer
pub type W = crate::W<C2AHB2SMENRrs>;
/**IO port A clock enable during CPU2 CSleep mode.

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
///Field `GPIOASMEN` reader - IO port A clock enable during CPU2 CSleep mode.
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
///Field `GPIOASMEN` writer - IO port A clock enable during CPU2 CSleep mode.
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
///Field `GPIOBSMEN` reader - IO port B clock enable during CPU2 CSleep mode.
pub use GPIOASMEN_R as GPIOBSMEN_R;
///Field `GPIOCSMEN` reader - IO port C clock enable during CPU2 CSleep mode.
pub use GPIOASMEN_R as GPIOCSMEN_R;
///Field `GPIOHSMEN` reader - IO port H clock enable during CPU2 CSleep mode.
pub use GPIOASMEN_R as GPIOHSMEN_R;
///Field `GPIOBSMEN` writer - IO port B clock enable during CPU2 CSleep mode.
pub use GPIOASMEN_W as GPIOBSMEN_W;
///Field `GPIOCSMEN` writer - IO port C clock enable during CPU2 CSleep mode.
pub use GPIOASMEN_W as GPIOCSMEN_W;
///Field `GPIOHSMEN` writer - IO port H clock enable during CPU2 CSleep mode.
pub use GPIOASMEN_W as GPIOHSMEN_W;
impl R {
    ///Bit 0 - IO port A clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 7 - IO port H clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn gpiohsmen(&self) -> GPIOHSMEN_R {
        GPIOHSMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2AHB2SMENR")
            .field("gpioasmen", &self.gpioasmen())
            .field("gpiohsmen", &self.gpiohsmen())
            .field("gpiocsmen", &self.gpiocsmen())
            .field("gpiobsmen", &self.gpiobsmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - IO port A clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<'_, C2AHB2SMENRrs> {
        GPIOASMEN_W::new(self, 0)
    }
    ///Bit 1 - IO port B clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<'_, C2AHB2SMENRrs> {
        GPIOBSMEN_W::new(self, 1)
    }
    ///Bit 2 - IO port C clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<'_, C2AHB2SMENRrs> {
        GPIOCSMEN_W::new(self, 2)
    }
    ///Bit 7 - IO port H clock enable during CPU2 CSleep mode.
    #[inline(always)]
    pub fn gpiohsmen(&mut self) -> GPIOHSMEN_W<'_, C2AHB2SMENRrs> {
        GPIOHSMEN_W::new(self, 7)
    }
}
/**CPU2 AHB2 peripheral clocks enable in Sleep modes register \[dual core device only\]

You can [`read`](crate::Reg::read) this register and get [`c2ahb2smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2ahb2smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#RCC:C2AHB2SMENR)*/
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
///`reset()` method sets C2AHB2SMENR to value 0x87
impl crate::Resettable for C2AHB2SMENRrs {
    const RESET_VALUE: u32 = 0x87;
}
