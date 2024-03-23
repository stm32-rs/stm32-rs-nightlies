#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1rs>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1rs>;
#[doc = "TAMP1E\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1E {
    #[doc = "0: Tamper detection on TAMP_INx is disabled"]
    Disabled = 0,
    #[doc = "1: Tamper detection on TAMP_IN3 is enabled"]
    Enabled = 1,
}
impl From<TAMP1E> for bool {
    #[inline(always)]
    fn from(variant: TAMP1E) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1E` reader - TAMP1E"]
pub type TAMP1E_R = crate::BitReader<TAMP1E>;
impl TAMP1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1E {
        match self.bits {
            false => TAMP1E::Disabled,
            true => TAMP1E::Enabled,
        }
    }
    #[doc = "Tamper detection on TAMP_INx is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMP1E::Disabled
    }
    #[doc = "Tamper detection on TAMP_IN3 is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMP1E::Enabled
    }
}
#[doc = "Field `TAMP1E` writer - TAMP1E"]
pub type TAMP1E_W<'a, REG> = crate::BitWriter<'a, REG, TAMP1E>;
impl<'a, REG> TAMP1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper detection on TAMP_INx is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1E::Disabled)
    }
    #[doc = "Tamper detection on TAMP_IN3 is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1E::Enabled)
    }
}
#[doc = "Field `TAMP2E` reader - TAMP2E"]
pub use TAMP1E_R as TAMP2E_R;
#[doc = "Field `TAMP3E` reader - TAMP2E"]
pub use TAMP1E_R as TAMP3E_R;
#[doc = "Field `TAMP2E` writer - TAMP2E"]
pub use TAMP1E_W as TAMP2E_W;
#[doc = "Field `TAMP3E` writer - TAMP2E"]
pub use TAMP1E_W as TAMP3E_W;
#[doc = "ITAMP3E\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP3E {
    #[doc = "0: Internal tamper x disabled"]
    Disabled = 0,
    #[doc = "1: Internal tamper x enabled"]
    Enabled = 1,
}
impl From<ITAMP3E> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3E) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP3E` reader - ITAMP3E"]
pub type ITAMP3E_R = crate::BitReader<ITAMP3E>;
impl ITAMP3E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP3E {
        match self.bits {
            false => ITAMP3E::Disabled,
            true => ITAMP3E::Enabled,
        }
    }
    #[doc = "Internal tamper x disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ITAMP3E::Disabled
    }
    #[doc = "Internal tamper x enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ITAMP3E::Enabled
    }
}
#[doc = "Field `ITAMP3E` writer - ITAMP3E"]
pub type ITAMP3E_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP3E>;
impl<'a, REG> ITAMP3E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper x disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP3E::Disabled)
    }
    #[doc = "Internal tamper x enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP3E::Enabled)
    }
}
#[doc = "Field `ITAMP5E` reader - ITAMP5E"]
pub use ITAMP3E_R as ITAMP5E_R;
#[doc = "Field `ITAMP6E` reader - ITAMP6E"]
pub use ITAMP3E_R as ITAMP6E_R;
#[doc = "Field `ITAMP8E` reader - ITAMP8E"]
pub use ITAMP3E_R as ITAMP8E_R;
#[doc = "Field `ITAMP5E` writer - ITAMP5E"]
pub use ITAMP3E_W as ITAMP5E_W;
#[doc = "Field `ITAMP6E` writer - ITAMP6E"]
pub use ITAMP3E_W as ITAMP6E_W;
#[doc = "Field `ITAMP8E` writer - ITAMP8E"]
pub use ITAMP3E_W as ITAMP8E_W;
impl R {
    #[doc = "Bit 0 - TAMP1E"]
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2E"]
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAMP2E"]
    #[inline(always)]
    pub fn tamp3e(&self) -> TAMP3E_R {
        TAMP3E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 18 - ITAMP3E"]
    #[inline(always)]
    pub fn itamp3e(&self) -> ITAMP3E_R {
        ITAMP3E_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - ITAMP5E"]
    #[inline(always)]
    pub fn itamp5e(&self) -> ITAMP5E_R {
        ITAMP5E_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ITAMP6E"]
    #[inline(always)]
    pub fn itamp6e(&self) -> ITAMP6E_R {
        ITAMP6E_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - ITAMP8E"]
    #[inline(always)]
    pub fn itamp8e(&self) -> ITAMP8E_R {
        ITAMP8E_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMP1E"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1e(&mut self) -> TAMP1E_W<CR1rs> {
        TAMP1E_W::new(self, 0)
    }
    #[doc = "Bit 1 - TAMP2E"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2e(&mut self) -> TAMP2E_W<CR1rs> {
        TAMP2E_W::new(self, 1)
    }
    #[doc = "Bit 2 - TAMP2E"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3e(&mut self) -> TAMP3E_W<CR1rs> {
        TAMP3E_W::new(self, 2)
    }
    #[doc = "Bit 18 - ITAMP3E"]
    #[inline(always)]
    #[must_use]
    pub fn itamp3e(&mut self) -> ITAMP3E_W<CR1rs> {
        ITAMP3E_W::new(self, 18)
    }
    #[doc = "Bit 20 - ITAMP5E"]
    #[inline(always)]
    #[must_use]
    pub fn itamp5e(&mut self) -> ITAMP5E_W<CR1rs> {
        ITAMP5E_W::new(self, 20)
    }
    #[doc = "Bit 21 - ITAMP6E"]
    #[inline(always)]
    #[must_use]
    pub fn itamp6e(&mut self) -> ITAMP6E_W<CR1rs> {
        ITAMP6E_W::new(self, 21)
    }
    #[doc = "Bit 23 - ITAMP8E"]
    #[inline(always)]
    #[must_use]
    pub fn itamp8e(&mut self) -> ITAMP8E_W<CR1rs> {
        ITAMP8E_W::new(self, 23)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1rs {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0xffff_0000"]
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0xffff_0000;
}
