#[doc = "Register `IOPSMEN` reader"]
pub type R = crate::R<IOPSMENrs>;
#[doc = "Register `IOPSMEN` writer"]
pub type W = crate::W<IOPSMENrs>;
#[doc = "IOPASMEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOPASMEN {
    #[doc = "0: Port x clock is disabled in Sleep mode"]
    Disabled = 0,
    #[doc = "1: Port x clock is enabled in Sleep mode (if enabled by IOPHEN)"]
    Enabled = 1,
}
impl From<IOPASMEN> for bool {
    #[inline(always)]
    fn from(variant: IOPASMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOPASMEN` reader - IOPASMEN"]
pub type IOPASMEN_R = crate::BitReader<IOPASMEN>;
impl IOPASMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IOPASMEN {
        match self.bits {
            false => IOPASMEN::Disabled,
            true => IOPASMEN::Enabled,
        }
    }
    #[doc = "Port x clock is disabled in Sleep mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IOPASMEN::Disabled
    }
    #[doc = "Port x clock is enabled in Sleep mode (if enabled by IOPHEN)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IOPASMEN::Enabled
    }
}
#[doc = "Field `IOPASMEN` writer - IOPASMEN"]
pub type IOPASMEN_W<'a, REG> = crate::BitWriter<'a, REG, IOPASMEN>;
impl<'a, REG> IOPASMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port x clock is disabled in Sleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IOPASMEN::Disabled)
    }
    #[doc = "Port x clock is enabled in Sleep mode (if enabled by IOPHEN)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IOPASMEN::Enabled)
    }
}
#[doc = "Field `IOPBSMEN` reader - IOPBSMEN"]
pub use IOPASMEN_R as IOPBSMEN_R;
#[doc = "Field `IOPCSMEN` reader - IOPCSMEN"]
pub use IOPASMEN_R as IOPCSMEN_R;
#[doc = "Field `IOPDSMEN` reader - IOPDSMEN"]
pub use IOPASMEN_R as IOPDSMEN_R;
#[doc = "Field `IOPESMEN` reader - Port E clock enable during Sleep mode bit"]
pub use IOPASMEN_R as IOPESMEN_R;
#[doc = "Field `IOPHSMEN` reader - IOPHSMEN"]
pub use IOPASMEN_R as IOPHSMEN_R;
#[doc = "Field `IOPBSMEN` writer - IOPBSMEN"]
pub use IOPASMEN_W as IOPBSMEN_W;
#[doc = "Field `IOPCSMEN` writer - IOPCSMEN"]
pub use IOPASMEN_W as IOPCSMEN_W;
#[doc = "Field `IOPDSMEN` writer - IOPDSMEN"]
pub use IOPASMEN_W as IOPDSMEN_W;
#[doc = "Field `IOPESMEN` writer - Port E clock enable during Sleep mode bit"]
pub use IOPASMEN_W as IOPESMEN_W;
#[doc = "Field `IOPHSMEN` writer - IOPHSMEN"]
pub use IOPASMEN_W as IOPHSMEN_W;
impl R {
    #[doc = "Bit 0 - IOPASMEN"]
    #[inline(always)]
    pub fn iopasmen(&self) -> IOPASMEN_R {
        IOPASMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IOPBSMEN"]
    #[inline(always)]
    pub fn iopbsmen(&self) -> IOPBSMEN_R {
        IOPBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IOPCSMEN"]
    #[inline(always)]
    pub fn iopcsmen(&self) -> IOPCSMEN_R {
        IOPCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IOPDSMEN"]
    #[inline(always)]
    pub fn iopdsmen(&self) -> IOPDSMEN_R {
        IOPDSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port E clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopesmen(&self) -> IOPESMEN_R {
        IOPESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - IOPHSMEN"]
    #[inline(always)]
    pub fn iophsmen(&self) -> IOPHSMEN_R {
        IOPHSMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IOPASMEN"]
    #[inline(always)]
    #[must_use]
    pub fn iopasmen(&mut self) -> IOPASMEN_W<IOPSMENrs> {
        IOPASMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - IOPBSMEN"]
    #[inline(always)]
    #[must_use]
    pub fn iopbsmen(&mut self) -> IOPBSMEN_W<IOPSMENrs> {
        IOPBSMEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - IOPCSMEN"]
    #[inline(always)]
    #[must_use]
    pub fn iopcsmen(&mut self) -> IOPCSMEN_W<IOPSMENrs> {
        IOPCSMEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - IOPDSMEN"]
    #[inline(always)]
    #[must_use]
    pub fn iopdsmen(&mut self) -> IOPDSMEN_W<IOPSMENrs> {
        IOPDSMEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port E clock enable during Sleep mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn iopesmen(&mut self) -> IOPESMEN_W<IOPSMENrs> {
        IOPESMEN_W::new(self, 4)
    }
    #[doc = "Bit 7 - IOPHSMEN"]
    #[inline(always)]
    #[must_use]
    pub fn iophsmen(&mut self) -> IOPHSMEN_W<IOPSMENrs> {
        IOPHSMEN_W::new(self, 7)
    }
}
#[doc = "GPIO clock enable in sleep mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iopsmen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iopsmen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOPSMENrs;
impl crate::RegisterSpec for IOPSMENrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iopsmen::R`](R) reader structure"]
impl crate::Readable for IOPSMENrs {}
#[doc = "`write(|w| ..)` method takes [`iopsmen::W`](W) writer structure"]
impl crate::Writable for IOPSMENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOPSMEN to value 0x8f"]
impl crate::Resettable for IOPSMENrs {
    const RESET_VALUE: u32 = 0x8f;
}
