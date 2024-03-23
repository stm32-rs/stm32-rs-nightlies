#[doc = "Register `IOPENR` reader"]
pub type R = crate::R<IOPENRrs>;
#[doc = "Register `IOPENR` writer"]
pub type W = crate::W<IOPENRrs>;
#[doc = "IO port A clock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOPAEN {
    #[doc = "0: Port clock disabled"]
    Disabled = 0,
    #[doc = "1: Port clock enabled"]
    Enabled = 1,
}
impl From<IOPAEN> for bool {
    #[inline(always)]
    fn from(variant: IOPAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOPAEN` reader - IO port A clock enable bit"]
pub type IOPAEN_R = crate::BitReader<IOPAEN>;
impl IOPAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IOPAEN {
        match self.bits {
            false => IOPAEN::Disabled,
            true => IOPAEN::Enabled,
        }
    }
    #[doc = "Port clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IOPAEN::Disabled
    }
    #[doc = "Port clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IOPAEN::Enabled
    }
}
#[doc = "Field `IOPAEN` writer - IO port A clock enable bit"]
pub type IOPAEN_W<'a, REG> = crate::BitWriter<'a, REG, IOPAEN>;
impl<'a, REG> IOPAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IOPAEN::Disabled)
    }
    #[doc = "Port clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IOPAEN::Enabled)
    }
}
#[doc = "Field `IOPBEN` reader - IO port B clock enable bit"]
pub use IOPAEN_R as IOPBEN_R;
#[doc = "Field `IOPCEN` reader - IO port A clock enable bit"]
pub use IOPAEN_R as IOPCEN_R;
#[doc = "Field `IOPDEN` reader - I/O port D clock enable bit"]
pub use IOPAEN_R as IOPDEN_R;
#[doc = "Field `IOPEEN` reader - IO port E clock enable bit"]
pub use IOPAEN_R as IOPEEN_R;
#[doc = "Field `IOPHEN` reader - I/O port H clock enable bit"]
pub use IOPAEN_R as IOPHEN_R;
#[doc = "Field `IOPBEN` writer - IO port B clock enable bit"]
pub use IOPAEN_W as IOPBEN_W;
#[doc = "Field `IOPCEN` writer - IO port A clock enable bit"]
pub use IOPAEN_W as IOPCEN_W;
#[doc = "Field `IOPDEN` writer - I/O port D clock enable bit"]
pub use IOPAEN_W as IOPDEN_W;
#[doc = "Field `IOPEEN` writer - IO port E clock enable bit"]
pub use IOPAEN_W as IOPEEN_W;
#[doc = "Field `IOPHEN` writer - I/O port H clock enable bit"]
pub use IOPAEN_W as IOPHEN_W;
impl R {
    #[doc = "Bit 0 - IO port A clock enable bit"]
    #[inline(always)]
    pub fn iopaen(&self) -> IOPAEN_R {
        IOPAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable bit"]
    #[inline(always)]
    pub fn iopben(&self) -> IOPBEN_R {
        IOPBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port A clock enable bit"]
    #[inline(always)]
    pub fn iopcen(&self) -> IOPCEN_R {
        IOPCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port D clock enable bit"]
    #[inline(always)]
    pub fn iopden(&self) -> IOPDEN_R {
        IOPDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable bit"]
    #[inline(always)]
    pub fn iopeen(&self) -> IOPEEN_R {
        IOPEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - I/O port H clock enable bit"]
    #[inline(always)]
    pub fn iophen(&self) -> IOPHEN_R {
        IOPHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn iopaen(&mut self) -> IOPAEN_W<IOPENRrs> {
        IOPAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn iopben(&mut self) -> IOPBEN_W<IOPENRrs> {
        IOPBEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - IO port A clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn iopcen(&mut self) -> IOPCEN_W<IOPENRrs> {
        IOPCEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - I/O port D clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn iopden(&mut self) -> IOPDEN_W<IOPENRrs> {
        IOPDEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn iopeen(&mut self) -> IOPEEN_W<IOPENRrs> {
        IOPEEN_W::new(self, 4)
    }
    #[doc = "Bit 7 - I/O port H clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn iophen(&mut self) -> IOPHEN_W<IOPENRrs> {
        IOPHEN_W::new(self, 7)
    }
}
#[doc = "GPIO clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iopenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iopenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOPENRrs;
impl crate::RegisterSpec for IOPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iopenr::R`](R) reader structure"]
impl crate::Readable for IOPENRrs {}
#[doc = "`write(|w| ..)` method takes [`iopenr::W`](W) writer structure"]
impl crate::Writable for IOPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOPENR to value 0"]
impl crate::Resettable for IOPENRrs {
    const RESET_VALUE: u32 = 0;
}
