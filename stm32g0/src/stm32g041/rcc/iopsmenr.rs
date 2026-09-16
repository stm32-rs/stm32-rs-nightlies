///Register `IOPSMENR` reader
pub type R = crate::R<IOPSMENRrs>;
///Register `IOPSMENR` writer
pub type W = crate::W<IOPSMENRrs>;
/**I/O port A clock enable during Sleep mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOPASMEN {
    ///0: Peripheral disabled (typically saves power)
    Disabled = 0,
    ///1: Peripheral enabled
    Enabled = 1,
}
impl From<IOPASMEN> for bool {
    #[inline(always)]
    fn from(variant: IOPASMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `IOPASMEN` reader - I/O port A clock enable during Sleep mode
pub type IOPASMEN_R = crate::BitReader<IOPASMEN>;
impl IOPASMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IOPASMEN {
        match self.bits {
            false => IOPASMEN::Disabled,
            true => IOPASMEN::Enabled,
        }
    }
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IOPASMEN::Disabled
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IOPASMEN::Enabled
    }
}
///Field `IOPASMEN` writer - I/O port A clock enable during Sleep mode
pub type IOPASMEN_W<'a, REG> = crate::BitWriter<'a, REG, IOPASMEN>;
impl<'a, REG> IOPASMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IOPASMEN::Disabled)
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IOPASMEN::Enabled)
    }
}
///Field `IOPBSMEN` reader - I/O port B clock enable during Sleep mode
pub use IOPASMEN_R as IOPBSMEN_R;
///Field `IOPCSMEN` reader - I/O port C clock enable during Sleep mode
pub use IOPASMEN_R as IOPCSMEN_R;
///Field `IOPDSMEN` reader - I/O port D clock enable during Sleep mode
pub use IOPASMEN_R as IOPDSMEN_R;
///Field `IOPFSMEN` reader - I/O port F clock enable during Sleep mode
pub use IOPASMEN_R as IOPFSMEN_R;
///Field `IOPBSMEN` writer - I/O port B clock enable during Sleep mode
pub use IOPASMEN_W as IOPBSMEN_W;
///Field `IOPCSMEN` writer - I/O port C clock enable during Sleep mode
pub use IOPASMEN_W as IOPCSMEN_W;
///Field `IOPDSMEN` writer - I/O port D clock enable during Sleep mode
pub use IOPASMEN_W as IOPDSMEN_W;
///Field `IOPFSMEN` writer - I/O port F clock enable during Sleep mode
pub use IOPASMEN_W as IOPFSMEN_W;
impl R {
    ///Bit 0 - I/O port A clock enable during Sleep mode
    #[inline(always)]
    pub fn iopasmen(&self) -> IOPASMEN_R {
        IOPASMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I/O port B clock enable during Sleep mode
    #[inline(always)]
    pub fn iopbsmen(&self) -> IOPBSMEN_R {
        IOPBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I/O port C clock enable during Sleep mode
    #[inline(always)]
    pub fn iopcsmen(&self) -> IOPCSMEN_R {
        IOPCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O port D clock enable during Sleep mode
    #[inline(always)]
    pub fn iopdsmen(&self) -> IOPDSMEN_R {
        IOPDSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - I/O port F clock enable during Sleep mode
    #[inline(always)]
    pub fn iopfsmen(&self) -> IOPFSMEN_R {
        IOPFSMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOPSMENR")
            .field("iopasmen", &self.iopasmen())
            .field("iopbsmen", &self.iopbsmen())
            .field("iopcsmen", &self.iopcsmen())
            .field("iopdsmen", &self.iopdsmen())
            .field("iopfsmen", &self.iopfsmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - I/O port A clock enable during Sleep mode
    #[inline(always)]
    pub fn iopasmen(&mut self) -> IOPASMEN_W<'_, IOPSMENRrs> {
        IOPASMEN_W::new(self, 0)
    }
    ///Bit 1 - I/O port B clock enable during Sleep mode
    #[inline(always)]
    pub fn iopbsmen(&mut self) -> IOPBSMEN_W<'_, IOPSMENRrs> {
        IOPBSMEN_W::new(self, 1)
    }
    ///Bit 2 - I/O port C clock enable during Sleep mode
    #[inline(always)]
    pub fn iopcsmen(&mut self) -> IOPCSMEN_W<'_, IOPSMENRrs> {
        IOPCSMEN_W::new(self, 2)
    }
    ///Bit 3 - I/O port D clock enable during Sleep mode
    #[inline(always)]
    pub fn iopdsmen(&mut self) -> IOPDSMEN_W<'_, IOPSMENRrs> {
        IOPDSMEN_W::new(self, 3)
    }
    ///Bit 5 - I/O port F clock enable during Sleep mode
    #[inline(always)]
    pub fn iopfsmen(&mut self) -> IOPFSMEN_W<'_, IOPSMENRrs> {
        IOPFSMEN_W::new(self, 5)
    }
}
/**GPIO in Sleep mode clock enable register

You can [`read`](crate::Reg::read) this register and get [`iopsmenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopsmenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G041.html#RCC:IOPSMENR)*/
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
///`reset()` method sets IOPSMENR to value 0
impl crate::Resettable for IOPSMENRrs {}
