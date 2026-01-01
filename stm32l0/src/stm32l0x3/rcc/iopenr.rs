///Register `IOPENR` reader
pub type R = crate::R<IOPENRrs>;
///Register `IOPENR` writer
pub type W = crate::W<IOPENRrs>;
/**IO port A clock enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOPAEN {
    ///0: Port clock disabled
    Disabled = 0,
    ///1: Port clock enabled
    Enabled = 1,
}
impl From<IOPAEN> for bool {
    #[inline(always)]
    fn from(variant: IOPAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `IOPAEN` reader - IO port A clock enable bit
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
    ///Port clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IOPAEN::Disabled
    }
    ///Port clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IOPAEN::Enabled
    }
}
///Field `IOPAEN` writer - IO port A clock enable bit
pub type IOPAEN_W<'a, REG> = crate::BitWriter<'a, REG, IOPAEN>;
impl<'a, REG> IOPAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IOPAEN::Disabled)
    }
    ///Port clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IOPAEN::Enabled)
    }
}
///Field `IOPBEN` reader - IO port B clock enable bit
pub use IOPAEN_R as IOPBEN_R;
///Field `IOPCEN` reader - IO port A clock enable bit
pub use IOPAEN_R as IOPCEN_R;
///Field `IOPDEN` reader - I/O port D clock enable bit
pub use IOPAEN_R as IOPDEN_R;
///Field `IOPEEN` reader - I/O port E clock enable bit
pub use IOPAEN_R as IOPEEN_R;
///Field `IOPHEN` reader - I/O port H clock enable bit
pub use IOPAEN_R as IOPHEN_R;
///Field `IOPBEN` writer - IO port B clock enable bit
pub use IOPAEN_W as IOPBEN_W;
///Field `IOPCEN` writer - IO port A clock enable bit
pub use IOPAEN_W as IOPCEN_W;
///Field `IOPDEN` writer - I/O port D clock enable bit
pub use IOPAEN_W as IOPDEN_W;
///Field `IOPEEN` writer - I/O port E clock enable bit
pub use IOPAEN_W as IOPEEN_W;
///Field `IOPHEN` writer - I/O port H clock enable bit
pub use IOPAEN_W as IOPHEN_W;
impl R {
    ///Bit 0 - IO port A clock enable bit
    #[inline(always)]
    pub fn iopaen(&self) -> IOPAEN_R {
        IOPAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B clock enable bit
    #[inline(always)]
    pub fn iopben(&self) -> IOPBEN_R {
        IOPBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port A clock enable bit
    #[inline(always)]
    pub fn iopcen(&self) -> IOPCEN_R {
        IOPCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O port D clock enable bit
    #[inline(always)]
    pub fn iopden(&self) -> IOPDEN_R {
        IOPDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - I/O port E clock enable bit
    #[inline(always)]
    pub fn iopeen(&self) -> IOPEEN_R {
        IOPEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - I/O port H clock enable bit
    #[inline(always)]
    pub fn iophen(&self) -> IOPHEN_R {
        IOPHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOPENR")
            .field("iopaen", &self.iopaen())
            .field("iophen", &self.iophen())
            .field("iopden", &self.iopden())
            .field("iopcen", &self.iopcen())
            .field("iopben", &self.iopben())
            .field("iopeen", &self.iopeen())
            .finish()
    }
}
impl W {
    ///Bit 0 - IO port A clock enable bit
    #[inline(always)]
    pub fn iopaen(&mut self) -> IOPAEN_W<'_, IOPENRrs> {
        IOPAEN_W::new(self, 0)
    }
    ///Bit 1 - IO port B clock enable bit
    #[inline(always)]
    pub fn iopben(&mut self) -> IOPBEN_W<'_, IOPENRrs> {
        IOPBEN_W::new(self, 1)
    }
    ///Bit 2 - IO port A clock enable bit
    #[inline(always)]
    pub fn iopcen(&mut self) -> IOPCEN_W<'_, IOPENRrs> {
        IOPCEN_W::new(self, 2)
    }
    ///Bit 3 - I/O port D clock enable bit
    #[inline(always)]
    pub fn iopden(&mut self) -> IOPDEN_W<'_, IOPENRrs> {
        IOPDEN_W::new(self, 3)
    }
    ///Bit 4 - I/O port E clock enable bit
    #[inline(always)]
    pub fn iopeen(&mut self) -> IOPEEN_W<'_, IOPENRrs> {
        IOPEEN_W::new(self, 4)
    }
    ///Bit 7 - I/O port H clock enable bit
    #[inline(always)]
    pub fn iophen(&mut self) -> IOPHEN_W<'_, IOPENRrs> {
        IOPHEN_W::new(self, 7)
    }
}
/**GPIO clock enable register

You can [`read`](crate::Reg::read) this register and get [`iopenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x3.html#RCC:IOPENR)*/
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
