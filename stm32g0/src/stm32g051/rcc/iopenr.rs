///Register `IOPENR` reader
pub type R = crate::R<IOPENRrs>;
///Register `IOPENR` writer
pub type W = crate::W<IOPENRrs>;
/**I/O port A clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOPAEN {
    ///0: Peripheral disabled (typically saves power)
    Disabled = 0,
    ///1: Peripheral enabled
    Enabled = 1,
}
impl From<IOPAEN> for bool {
    #[inline(always)]
    fn from(variant: IOPAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `IOPAEN` reader - I/O port A clock enable
pub type IOPAEN_R = crate::BitReader<IOPAEN>;
impl IOPAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IOPAEN {
        match self.bits {
            false => IOPAEN::Disabled,
            true => IOPAEN::Enabled,
        }
    }
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IOPAEN::Disabled
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IOPAEN::Enabled
    }
}
///Field `IOPAEN` writer - I/O port A clock enable
pub type IOPAEN_W<'a, REG> = crate::BitWriter<'a, REG, IOPAEN>;
impl<'a, REG> IOPAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IOPAEN::Disabled)
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IOPAEN::Enabled)
    }
}
///Field `IOPBEN` reader - I/O port B clock enable
pub use IOPAEN_R as IOPBEN_R;
///Field `IOPCEN` reader - I/O port C clock enable
pub use IOPAEN_R as IOPCEN_R;
///Field `IOPDEN` reader - I/O port D clock enable
pub use IOPAEN_R as IOPDEN_R;
///Field `IOPFEN` reader - I/O port F clock enable
pub use IOPAEN_R as IOPFEN_R;
///Field `IOPBEN` writer - I/O port B clock enable
pub use IOPAEN_W as IOPBEN_W;
///Field `IOPCEN` writer - I/O port C clock enable
pub use IOPAEN_W as IOPCEN_W;
///Field `IOPDEN` writer - I/O port D clock enable
pub use IOPAEN_W as IOPDEN_W;
///Field `IOPFEN` writer - I/O port F clock enable
pub use IOPAEN_W as IOPFEN_W;
impl R {
    ///Bit 0 - I/O port A clock enable
    #[inline(always)]
    pub fn iopaen(&self) -> IOPAEN_R {
        IOPAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I/O port B clock enable
    #[inline(always)]
    pub fn iopben(&self) -> IOPBEN_R {
        IOPBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I/O port C clock enable
    #[inline(always)]
    pub fn iopcen(&self) -> IOPCEN_R {
        IOPCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O port D clock enable
    #[inline(always)]
    pub fn iopden(&self) -> IOPDEN_R {
        IOPDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - I/O port F clock enable
    #[inline(always)]
    pub fn iopfen(&self) -> IOPFEN_R {
        IOPFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOPENR")
            .field("iopaen", &self.iopaen())
            .field("iopben", &self.iopben())
            .field("iopcen", &self.iopcen())
            .field("iopden", &self.iopden())
            .field("iopfen", &self.iopfen())
            .finish()
    }
}
impl W {
    ///Bit 0 - I/O port A clock enable
    #[inline(always)]
    pub fn iopaen(&mut self) -> IOPAEN_W<IOPENRrs> {
        IOPAEN_W::new(self, 0)
    }
    ///Bit 1 - I/O port B clock enable
    #[inline(always)]
    pub fn iopben(&mut self) -> IOPBEN_W<IOPENRrs> {
        IOPBEN_W::new(self, 1)
    }
    ///Bit 2 - I/O port C clock enable
    #[inline(always)]
    pub fn iopcen(&mut self) -> IOPCEN_W<IOPENRrs> {
        IOPCEN_W::new(self, 2)
    }
    ///Bit 3 - I/O port D clock enable
    #[inline(always)]
    pub fn iopden(&mut self) -> IOPDEN_W<IOPENRrs> {
        IOPDEN_W::new(self, 3)
    }
    ///Bit 5 - I/O port F clock enable
    #[inline(always)]
    pub fn iopfen(&mut self) -> IOPFEN_W<IOPENRrs> {
        IOPFEN_W::new(self, 5)
    }
}
/**GPIO clock enable register

You can [`read`](crate::Reg::read) this register and get [`iopenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G051.html#RCC:IOPENR)*/
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
