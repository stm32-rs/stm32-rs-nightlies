#[doc = "Register `OENR` reader"]
pub type R = crate::R<OENRrs>;
#[doc = "Register `OENR` writer"]
pub type W = crate::W<OENRrs>;
#[doc = "Field `TA1OEN` reader - Timer A Output 1 Enable"]
pub type TA1OEN_R = crate::BitReader;
#[doc = "Field `TA1OEN` writer - Timer A Output 1 Enable"]
pub type TA1OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TA2OEN` reader - Timer A Output 2 Enable"]
pub type TA2OEN_R = crate::BitReader;
#[doc = "Field `TA2OEN` writer - Timer A Output 2 Enable"]
pub type TA2OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TB1OEN` reader - Timer B Output 1 Enable"]
pub type TB1OEN_R = crate::BitReader;
#[doc = "Field `TB1OEN` writer - Timer B Output 1 Enable"]
pub type TB1OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TB2OEN` reader - Timer B Output 2 Enable"]
pub type TB2OEN_R = crate::BitReader;
#[doc = "Field `TB2OEN` writer - Timer B Output 2 Enable"]
pub type TB2OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1OEN` reader - Timer C Output 1 Enable"]
pub type TC1OEN_R = crate::BitReader;
#[doc = "Field `TC1OEN` writer - Timer C Output 1 Enable"]
pub type TC1OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2OEN` reader - Timer C Output 2 Enable"]
pub type TC2OEN_R = crate::BitReader;
#[doc = "Field `TC2OEN` writer - Timer C Output 2 Enable"]
pub type TC2OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TD1OEN` reader - Timer D Output 1 Enable"]
pub type TD1OEN_R = crate::BitReader;
#[doc = "Field `TD1OEN` writer - Timer D Output 1 Enable"]
pub type TD1OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TD2OEN` reader - Timer D Output 2 Enable"]
pub type TD2OEN_R = crate::BitReader;
#[doc = "Field `TD2OEN` writer - Timer D Output 2 Enable"]
pub type TD2OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE1OEN` reader - Timer E Output 1 Enable"]
pub type TE1OEN_R = crate::BitReader;
#[doc = "Field `TE1OEN` writer - Timer E Output 1 Enable"]
pub type TE1OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE2OEN` reader - Timer E Output 2 Enable"]
pub type TE2OEN_R = crate::BitReader;
#[doc = "Field `TE2OEN` writer - Timer E Output 2 Enable"]
pub type TE2OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Bit 10 - Timer F Output 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TF1OENR {
    #[doc = "0: Output disabled"]
    Disabled = 0,
    #[doc = "1: Output enabled"]
    Enabled = 1,
}
impl From<TF1OENR> for bool {
    #[inline(always)]
    fn from(variant: TF1OENR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TF1OEN` reader - Bit 10 - Timer F Output 1 Enable"]
pub type TF1OEN_R = crate::BitReader<TF1OENR>;
impl TF1OEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TF1OENR {
        match self.bits {
            false => TF1OENR::Disabled,
            true => TF1OENR::Enabled,
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TF1OENR::Disabled
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TF1OENR::Enabled
    }
}
#[doc = "Bit 10 - Timer F Output 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TF1OENW {
    #[doc = "1: Enable output"]
    Enable = 1,
}
impl From<TF1OENW> for bool {
    #[inline(always)]
    fn from(variant: TF1OENW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TF1OEN` writer - Bit 10 - Timer F Output 1 Enable"]
pub type TF1OEN_W<'a, REG> = crate::BitWriter<'a, REG, TF1OENW>;
impl<'a, REG> TF1OEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TF1OENW::Enable)
    }
}
#[doc = "Bit 11 - Timer F Output 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TF2OENR {
    #[doc = "0: Output disabled"]
    Disabled = 0,
    #[doc = "1: Output enabled"]
    Enabled = 1,
}
impl From<TF2OENR> for bool {
    #[inline(always)]
    fn from(variant: TF2OENR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TF2OEN` reader - Bit 11 - Timer F Output 2 Enable"]
pub type TF2OEN_R = crate::BitReader<TF2OENR>;
impl TF2OEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TF2OENR {
        match self.bits {
            false => TF2OENR::Disabled,
            true => TF2OENR::Enabled,
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TF2OENR::Disabled
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TF2OENR::Enabled
    }
}
#[doc = "Bit 11 - Timer F Output 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TF2OENW {
    #[doc = "1: Enable output"]
    Enable = 1,
}
impl From<TF2OENW> for bool {
    #[inline(always)]
    fn from(variant: TF2OENW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TF2OEN` writer - Bit 11 - Timer F Output 2 Enable"]
pub type TF2OEN_W<'a, REG> = crate::BitWriter<'a, REG, TF2OENW>;
impl<'a, REG> TF2OEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TF2OENW::Enable)
    }
}
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
    #[doc = "Bit 10 - Bit 10 - Timer F Output 1 Enable"]
    #[inline(always)]
    pub fn tf1oen(&self) -> TF1OEN_R {
        TF1OEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bit 11 - Timer F Output 2 Enable"]
    #[inline(always)]
    pub fn tf2oen(&self) -> TF2OEN_R {
        TF2OEN_R::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 10 - Bit 10 - Timer F Output 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tf1oen(&mut self) -> TF1OEN_W<OENRrs> {
        TF1OEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Bit 11 - Timer F Output 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tf2oen(&mut self) -> TF2OEN_W<OENRrs> {
        TF2OEN_W::new(self, 11)
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
