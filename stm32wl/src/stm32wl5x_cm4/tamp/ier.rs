#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "TAMP1IE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1IE {
    #[doc = "0: Tamper x interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Tampoer x interrupt enabled"]
    Enabled = 1,
}
impl From<TAMP1IE> for bool {
    #[inline(always)]
    fn from(variant: TAMP1IE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1IE` reader - TAMP1IE"]
pub type TAMP1IE_R = crate::BitReader<TAMP1IE>;
impl TAMP1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1IE {
        match self.bits {
            false => TAMP1IE::Disabled,
            true => TAMP1IE::Enabled,
        }
    }
    #[doc = "Tamper x interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMP1IE::Disabled
    }
    #[doc = "Tampoer x interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMP1IE::Enabled
    }
}
#[doc = "Field `TAMP1IE` writer - TAMP1IE"]
pub type TAMP1IE_W<'a, REG> = crate::BitWriter<'a, REG, TAMP1IE>;
impl<'a, REG> TAMP1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper x interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1IE::Disabled)
    }
    #[doc = "Tampoer x interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1IE::Enabled)
    }
}
#[doc = "Field `TAMP2IE` reader - TAMP2IE"]
pub use TAMP1IE_R as TAMP2IE_R;
#[doc = "Field `TAMP3IE` reader - TAMP3IE"]
pub use TAMP1IE_R as TAMP3IE_R;
#[doc = "Field `TAMP2IE` writer - TAMP2IE"]
pub use TAMP1IE_W as TAMP2IE_W;
#[doc = "Field `TAMP3IE` writer - TAMP3IE"]
pub use TAMP1IE_W as TAMP3IE_W;
#[doc = "ITAMP3IE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP3IE {
    #[doc = "0: Internal tamper x interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Internal tamper x interrupt enabled"]
    Enabled = 1,
}
impl From<ITAMP3IE> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3IE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP3IE` reader - ITAMP3IE"]
pub type ITAMP3IE_R = crate::BitReader<ITAMP3IE>;
impl ITAMP3IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP3IE {
        match self.bits {
            false => ITAMP3IE::Disabled,
            true => ITAMP3IE::Enabled,
        }
    }
    #[doc = "Internal tamper x interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ITAMP3IE::Disabled
    }
    #[doc = "Internal tamper x interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ITAMP3IE::Enabled
    }
}
#[doc = "Field `ITAMP3IE` writer - ITAMP3IE"]
pub type ITAMP3IE_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP3IE>;
impl<'a, REG> ITAMP3IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper x interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP3IE::Disabled)
    }
    #[doc = "Internal tamper x interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP3IE::Enabled)
    }
}
#[doc = "Field `ITAMP5IE` reader - ITAMP5IE"]
pub use ITAMP3IE_R as ITAMP5IE_R;
#[doc = "Field `ITAMP6IE` reader - ITAMP6IE"]
pub use ITAMP3IE_R as ITAMP6IE_R;
#[doc = "Field `ITAMP8IE` reader - ITAMP8IE"]
pub use ITAMP3IE_R as ITAMP8IE_R;
#[doc = "Field `ITAMP5IE` writer - ITAMP5IE"]
pub use ITAMP3IE_W as ITAMP5IE_W;
#[doc = "Field `ITAMP6IE` writer - ITAMP6IE"]
pub use ITAMP3IE_W as ITAMP6IE_W;
#[doc = "Field `ITAMP8IE` writer - ITAMP8IE"]
pub use ITAMP3IE_W as ITAMP8IE_W;
impl R {
    #[doc = "Bit 0 - TAMP1IE"]
    #[inline(always)]
    pub fn tamp1ie(&self) -> TAMP1IE_R {
        TAMP1IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2IE"]
    #[inline(always)]
    pub fn tamp2ie(&self) -> TAMP2IE_R {
        TAMP2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAMP3IE"]
    #[inline(always)]
    pub fn tamp3ie(&self) -> TAMP3IE_R {
        TAMP3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 18 - ITAMP3IE"]
    #[inline(always)]
    pub fn itamp3ie(&self) -> ITAMP3IE_R {
        ITAMP3IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - ITAMP5IE"]
    #[inline(always)]
    pub fn itamp5ie(&self) -> ITAMP5IE_R {
        ITAMP5IE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ITAMP6IE"]
    #[inline(always)]
    pub fn itamp6ie(&self) -> ITAMP6IE_R {
        ITAMP6IE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - ITAMP8IE"]
    #[inline(always)]
    pub fn itamp8ie(&self) -> ITAMP8IE_R {
        ITAMP8IE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMP1IE"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1ie(&mut self) -> TAMP1IE_W<IERrs> {
        TAMP1IE_W::new(self, 0)
    }
    #[doc = "Bit 1 - TAMP2IE"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2ie(&mut self) -> TAMP2IE_W<IERrs> {
        TAMP2IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - TAMP3IE"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3ie(&mut self) -> TAMP3IE_W<IERrs> {
        TAMP3IE_W::new(self, 2)
    }
    #[doc = "Bit 18 - ITAMP3IE"]
    #[inline(always)]
    #[must_use]
    pub fn itamp3ie(&mut self) -> ITAMP3IE_W<IERrs> {
        ITAMP3IE_W::new(self, 18)
    }
    #[doc = "Bit 20 - ITAMP5IE"]
    #[inline(always)]
    #[must_use]
    pub fn itamp5ie(&mut self) -> ITAMP5IE_W<IERrs> {
        ITAMP5IE_W::new(self, 20)
    }
    #[doc = "Bit 21 - ITAMP6IE"]
    #[inline(always)]
    #[must_use]
    pub fn itamp6ie(&mut self) -> ITAMP6IE_W<IERrs> {
        ITAMP6IE_W::new(self, 21)
    }
    #[doc = "Bit 23 - ITAMP8IE"]
    #[inline(always)]
    #[must_use]
    pub fn itamp8ie(&mut self) -> ITAMP8IE_W<IERrs> {
        ITAMP8IE_W::new(self, 23)
    }
}
#[doc = "TAMP interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IERrs {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}
