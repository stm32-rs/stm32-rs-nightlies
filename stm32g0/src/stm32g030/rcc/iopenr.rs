///Register `IOPENR` reader
pub type R = crate::R<IOPENRrs>;
///Register `IOPENR` writer
pub type W = crate::W<IOPENRrs>;
/**I/O port A clock enable during Sleep mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAEN {
    ///0: Peripheral disabled (typically saves power)
    Disabled = 0,
    ///1: Peripheral enabled
    Enabled = 1,
}
impl From<GPIOAEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOAEN` reader - I/O port A clock enable during Sleep mode
pub type GPIOAEN_R = crate::BitReader<GPIOAEN>;
impl GPIOAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOAEN {
        match self.bits {
            false => GPIOAEN::Disabled,
            true => GPIOAEN::Enabled,
        }
    }
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOAEN::Disabled
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOAEN::Enabled
    }
}
///Field `GPIOAEN` writer - I/O port A clock enable during Sleep mode
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOAEN>;
impl<'a, REG> GPIOAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Disabled)
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Enabled)
    }
}
///Field `GPIOBEN` reader - I/O port B clock enable during Sleep mode
pub use GPIOAEN_R as GPIOBEN_R;
///Field `GPIOCEN` reader - I/O port C clock enable during Sleep mode
pub use GPIOAEN_R as GPIOCEN_R;
///Field `GPIODEN` reader - I/O port D clock enable during Sleep mode
pub use GPIOAEN_R as GPIODEN_R;
///Field `GPIOEEN` reader - I/O port E clock enable during Sleep mode
pub use GPIOAEN_R as GPIOEEN_R;
///Field `GPIOFEN` reader - I/O port F clock enable during Sleep mode
pub use GPIOAEN_R as GPIOFEN_R;
///Field `GPIOBEN` writer - I/O port B clock enable during Sleep mode
pub use GPIOAEN_W as GPIOBEN_W;
///Field `GPIOCEN` writer - I/O port C clock enable during Sleep mode
pub use GPIOAEN_W as GPIOCEN_W;
///Field `GPIODEN` writer - I/O port D clock enable during Sleep mode
pub use GPIOAEN_W as GPIODEN_W;
///Field `GPIOEEN` writer - I/O port E clock enable during Sleep mode
pub use GPIOAEN_W as GPIOEEN_W;
///Field `GPIOFEN` writer - I/O port F clock enable during Sleep mode
pub use GPIOAEN_W as GPIOFEN_W;
impl R {
    ///Bit 0 - I/O port A clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I/O port B clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I/O port C clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O port D clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - I/O port E clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - I/O port F clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOPENR")
            .field("gpioaen", &self.gpioaen())
            .field("gpioben", &self.gpioben())
            .field("gpiocen", &self.gpiocen())
            .field("gpioden", &self.gpioden())
            .field("gpioeen", &self.gpioeen())
            .field("gpiofen", &self.gpiofen())
            .finish()
    }
}
impl W {
    ///Bit 0 - I/O port A clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<'_, IOPENRrs> {
        GPIOAEN_W::new(self, 0)
    }
    ///Bit 1 - I/O port B clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W<'_, IOPENRrs> {
        GPIOBEN_W::new(self, 1)
    }
    ///Bit 2 - I/O port C clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<'_, IOPENRrs> {
        GPIOCEN_W::new(self, 2)
    }
    ///Bit 3 - I/O port D clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W<'_, IOPENRrs> {
        GPIODEN_W::new(self, 3)
    }
    ///Bit 4 - I/O port E clock enable during Sleep mode
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<'_, IOPENRrs> {
        GPIOEEN_W::new(self, 4)
    }
    ///Bit 5 - I/O port F clock enable during Sleep mode
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<'_, IOPENRrs> {
        GPIOFEN_W::new(self, 5)
    }
}
/**GPIO clock enable register

You can [`read`](crate::Reg::read) this register and get [`iopenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G030.html#RCC:IOPENR)*/
pub struct IOPENRrs;
impl crate::RegisterSpec for IOPENRrs {
    type Ux = u32;
}
///`read()` method returns [`iopenr::R`](R) reader structure
impl crate::Readable for IOPENRrs {}
///`write(|w| ..)` method takes [`iopenr::W`](W) writer structure
impl crate::Writable for IOPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IOPENR to value 0
impl crate::Resettable for IOPENRrs {}
