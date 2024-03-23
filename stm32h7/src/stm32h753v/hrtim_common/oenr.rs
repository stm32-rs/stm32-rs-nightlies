#[doc = "Register `OENR` writer"]
pub type W = crate::W<OENRrs>;
#[doc = "Field `TA1OEN` writer - Timer A Output 1 Enable"]
pub type TA1OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TA2OEN` writer - Timer A Output 2 Enable"]
pub type TA2OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TB1OEN` writer - Timer B Output 1 Enable"]
pub type TB1OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TB2OEN` writer - Timer B Output 2 Enable"]
pub type TB2OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1OEN` writer - Timer C Output 1 Enable"]
pub type TC1OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2OEN` writer - Timer C Output 2 Enable"]
pub type TC2OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TD1OEN` writer - Timer D Output 1 Enable"]
pub type TD1OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TD2OEN` writer - Timer D Output 2 Enable"]
pub type TD2OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE1OEN` writer - Timer E Output 1 Enable"]
pub type TE1OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE2OEN` writer - Timer E Output 2 Enable"]
pub type TE2OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Output Enable Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oenr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OENRrs;
impl crate::RegisterSpec for OENRrs {
    type Ux = u32;
}
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
