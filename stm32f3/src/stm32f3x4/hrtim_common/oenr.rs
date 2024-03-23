#[doc = "Register `OENR` reader"]
pub type R = crate::R<OENRrs>;
#[doc = "Register `OENR` writer"]
pub type W = crate::W<OENRrs>;
#[doc = "Timer A Output 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TA1OENR {
    #[doc = "0: Output disabled"]
    Disabled = 0,
    #[doc = "1: Output enabled"]
    Enabled = 1,
}
impl From<TA1OENR> for bool {
    #[inline(always)]
    fn from(variant: TA1OENR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TA1OEN` reader - Timer A Output 1 Enable"]
pub type TA1OEN_R = crate::BitReader<TA1OENR>;
impl TA1OEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TA1OENR {
        match self.bits {
            false => TA1OENR::Disabled,
            true => TA1OENR::Enabled,
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TA1OENR::Disabled
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TA1OENR::Enabled
    }
}
#[doc = "Timer A Output 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TA1OENW {
    #[doc = "1: Enable output"]
    Enable = 1,
}
impl From<TA1OENW> for bool {
    #[inline(always)]
    fn from(variant: TA1OENW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TA1OEN` writer - Timer A Output 1 Enable"]
pub type TA1OEN_W<'a, REG> = crate::BitWriter<'a, REG, TA1OENW>;
impl<'a, REG> TA1OEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TA1OENW::Enable)
    }
}
#[doc = "Field `TA2OEN` reader - Timer A Output 2 Enable"]
pub use TA1OEN_R as TA2OEN_R;
#[doc = "Field `TB1OEN` reader - Timer B Output 1 Enable"]
pub use TA1OEN_R as TB1OEN_R;
#[doc = "Field `TB2OEN` reader - Timer B Output 2 Enable"]
pub use TA1OEN_R as TB2OEN_R;
#[doc = "Field `TC1OEN` reader - Timer C Output 1 Enable"]
pub use TA1OEN_R as TC1OEN_R;
#[doc = "Field `TC2OEN` reader - Timer C Output 2 Enable"]
pub use TA1OEN_R as TC2OEN_R;
#[doc = "Field `TD1OEN` reader - Timer D Output 1 Enable"]
pub use TA1OEN_R as TD1OEN_R;
#[doc = "Field `TD2OEN` reader - Timer D Output 2 Enable"]
pub use TA1OEN_R as TD2OEN_R;
#[doc = "Field `TE1OEN` reader - Timer E Output 1 Enable"]
pub use TA1OEN_R as TE1OEN_R;
#[doc = "Field `TE2OEN` reader - Timer E Output 2 Enable"]
pub use TA1OEN_R as TE2OEN_R;
#[doc = "Field `TA2OEN` writer - Timer A Output 2 Enable"]
pub use TA1OEN_W as TA2OEN_W;
#[doc = "Field `TB1OEN` writer - Timer B Output 1 Enable"]
pub use TA1OEN_W as TB1OEN_W;
#[doc = "Field `TB2OEN` writer - Timer B Output 2 Enable"]
pub use TA1OEN_W as TB2OEN_W;
#[doc = "Field `TC1OEN` writer - Timer C Output 1 Enable"]
pub use TA1OEN_W as TC1OEN_W;
#[doc = "Field `TC2OEN` writer - Timer C Output 2 Enable"]
pub use TA1OEN_W as TC2OEN_W;
#[doc = "Field `TD1OEN` writer - Timer D Output 1 Enable"]
pub use TA1OEN_W as TD1OEN_W;
#[doc = "Field `TD2OEN` writer - Timer D Output 2 Enable"]
pub use TA1OEN_W as TD2OEN_W;
#[doc = "Field `TE1OEN` writer - Timer E Output 1 Enable"]
pub use TA1OEN_W as TE1OEN_W;
#[doc = "Field `TE2OEN` writer - Timer E Output 2 Enable"]
pub use TA1OEN_W as TE2OEN_W;
impl R {
    #[doc = "Bit 0 - Timer A Output 1 Enable"]
    #[inline(always)]
    pub fn ta1oen(&self) -> TA1OEN_R {
        TA1OEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer A Output 2 Enable"]
    #[inline(always)]
    pub fn ta2oen(&self) -> TA2OEN_R {
        TA2OEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer B Output 1 Enable"]
    #[inline(always)]
    pub fn tb1oen(&self) -> TB1OEN_R {
        TB1OEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer B Output 2 Enable"]
    #[inline(always)]
    pub fn tb2oen(&self) -> TB2OEN_R {
        TB2OEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer C Output 1 Enable"]
    #[inline(always)]
    pub fn tc1oen(&self) -> TC1OEN_R {
        TC1OEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer C Output 2 Enable"]
    #[inline(always)]
    pub fn tc2oen(&self) -> TC2OEN_R {
        TC2OEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer D Output 1 Enable"]
    #[inline(always)]
    pub fn td1oen(&self) -> TD1OEN_R {
        TD1OEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer D Output 2 Enable"]
    #[inline(always)]
    pub fn td2oen(&self) -> TD2OEN_R {
        TD2OEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer E Output 1 Enable"]
    #[inline(always)]
    pub fn te1oen(&self) -> TE1OEN_R {
        TE1OEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timer E Output 2 Enable"]
    #[inline(always)]
    pub fn te2oen(&self) -> TE2OEN_R {
        TE2OEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer A Output 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ta1oen(&mut self) -> TA1OEN_W<OENRrs> {
        TA1OEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer A Output 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ta2oen(&mut self) -> TA2OEN_W<OENRrs> {
        TA2OEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer B Output 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tb1oen(&mut self) -> TB1OEN_W<OENRrs> {
        TB1OEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timer B Output 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tb2oen(&mut self) -> TB2OEN_W<OENRrs> {
        TB2OEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Timer C Output 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc1oen(&mut self) -> TC1OEN_W<OENRrs> {
        TC1OEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Timer C Output 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc2oen(&mut self) -> TC2OEN_W<OENRrs> {
        TC2OEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Timer D Output 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn td1oen(&mut self) -> TD1OEN_W<OENRrs> {
        TD1OEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Timer D Output 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn td2oen(&mut self) -> TD2OEN_W<OENRrs> {
        TD2OEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Timer E Output 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn te1oen(&mut self) -> TE1OEN_W<OENRrs> {
        TE1OEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Timer E Output 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn te2oen(&mut self) -> TE2OEN_W<OENRrs> {
        TE2OEN_W::new(self, 9)
    }
}
#[doc = "Output Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OENRrs;
impl crate::RegisterSpec for OENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oenr::R`](R) reader structure"]
impl crate::Readable for OENRrs {}
#[doc = "`write(|w| ..)` method takes [`oenr::W`](W) writer structure"]
impl crate::Writable for OENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OENR to value 0"]
impl crate::Resettable for OENRrs {
    const RESET_VALUE: u32 = 0;
}
