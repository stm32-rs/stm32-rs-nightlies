#[doc = "Register `ODISR` reader"]
pub type R = crate::R<ODISRrs>;
#[doc = "Register `ODISR` writer"]
pub type W = crate::W<ODISRrs>;
#[doc = "TA1ODIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TA1ODIS {
    #[doc = "1: Disable output"]
    Disable = 1,
}
impl From<TA1ODIS> for bool {
    #[inline(always)]
    fn from(variant: TA1ODIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TA1ODIS` reader - TA1ODIS"]
pub type TA1ODIS_R = crate::BitReader<TA1ODIS>;
impl TA1ODIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TA1ODIS> {
        match self.bits {
            true => Some(TA1ODIS::Disable),
            _ => None,
        }
    }
    #[doc = "Disable output"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TA1ODIS::Disable
    }
}
#[doc = "Field `TA1ODIS` writer - TA1ODIS"]
pub type TA1ODIS_W<'a, REG> = crate::BitWriter<'a, REG, TA1ODIS>;
impl<'a, REG> TA1ODIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TA1ODIS::Disable)
    }
}
#[doc = "Field `TA2ODIS` reader - TA2ODIS"]
pub use TA1ODIS_R as TA2ODIS_R;
#[doc = "Field `TB1ODIS` reader - TB1ODIS"]
pub use TA1ODIS_R as TB1ODIS_R;
#[doc = "Field `TB2ODIS` reader - TB2ODIS"]
pub use TA1ODIS_R as TB2ODIS_R;
#[doc = "Field `TC1ODIS` reader - TC1ODIS"]
pub use TA1ODIS_R as TC1ODIS_R;
#[doc = "Field `TC2ODIS` reader - TC2ODIS"]
pub use TA1ODIS_R as TC2ODIS_R;
#[doc = "Field `TD1ODIS` reader - TD1ODIS"]
pub use TA1ODIS_R as TD1ODIS_R;
#[doc = "Field `TD2ODIS` reader - TD2ODIS"]
pub use TA1ODIS_R as TD2ODIS_R;
#[doc = "Field `TE1ODIS` reader - TE1ODIS"]
pub use TA1ODIS_R as TE1ODIS_R;
#[doc = "Field `TE2ODIS` reader - TE2ODIS"]
pub use TA1ODIS_R as TE2ODIS_R;
#[doc = "Field `TA2ODIS` writer - TA2ODIS"]
pub use TA1ODIS_W as TA2ODIS_W;
#[doc = "Field `TB1ODIS` writer - TB1ODIS"]
pub use TA1ODIS_W as TB1ODIS_W;
#[doc = "Field `TB2ODIS` writer - TB2ODIS"]
pub use TA1ODIS_W as TB2ODIS_W;
#[doc = "Field `TC1ODIS` writer - TC1ODIS"]
pub use TA1ODIS_W as TC1ODIS_W;
#[doc = "Field `TC2ODIS` writer - TC2ODIS"]
pub use TA1ODIS_W as TC2ODIS_W;
#[doc = "Field `TD1ODIS` writer - TD1ODIS"]
pub use TA1ODIS_W as TD1ODIS_W;
#[doc = "Field `TD2ODIS` writer - TD2ODIS"]
pub use TA1ODIS_W as TD2ODIS_W;
#[doc = "Field `TE1ODIS` writer - TE1ODIS"]
pub use TA1ODIS_W as TE1ODIS_W;
#[doc = "Field `TE2ODIS` writer - TE2ODIS"]
pub use TA1ODIS_W as TE2ODIS_W;
impl R {
    #[doc = "Bit 0 - TA1ODIS"]
    #[inline(always)]
    pub fn ta1odis(&self) -> TA1ODIS_R {
        TA1ODIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TA2ODIS"]
    #[inline(always)]
    pub fn ta2odis(&self) -> TA2ODIS_R {
        TA2ODIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TB1ODIS"]
    #[inline(always)]
    pub fn tb1odis(&self) -> TB1ODIS_R {
        TB1ODIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TB2ODIS"]
    #[inline(always)]
    pub fn tb2odis(&self) -> TB2ODIS_R {
        TB2ODIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TC1ODIS"]
    #[inline(always)]
    pub fn tc1odis(&self) -> TC1ODIS_R {
        TC1ODIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TC2ODIS"]
    #[inline(always)]
    pub fn tc2odis(&self) -> TC2ODIS_R {
        TC2ODIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TD1ODIS"]
    #[inline(always)]
    pub fn td1odis(&self) -> TD1ODIS_R {
        TD1ODIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TD2ODIS"]
    #[inline(always)]
    pub fn td2odis(&self) -> TD2ODIS_R {
        TD2ODIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TE1ODIS"]
    #[inline(always)]
    pub fn te1odis(&self) -> TE1ODIS_R {
        TE1ODIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TE2ODIS"]
    #[inline(always)]
    pub fn te2odis(&self) -> TE2ODIS_R {
        TE2ODIS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TA1ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn ta1odis(&mut self) -> TA1ODIS_W<ODISRrs> {
        TA1ODIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - TA2ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn ta2odis(&mut self) -> TA2ODIS_W<ODISRrs> {
        TA2ODIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - TB1ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn tb1odis(&mut self) -> TB1ODIS_W<ODISRrs> {
        TB1ODIS_W::new(self, 2)
    }
    #[doc = "Bit 3 - TB2ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn tb2odis(&mut self) -> TB2ODIS_W<ODISRrs> {
        TB2ODIS_W::new(self, 3)
    }
    #[doc = "Bit 4 - TC1ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn tc1odis(&mut self) -> TC1ODIS_W<ODISRrs> {
        TC1ODIS_W::new(self, 4)
    }
    #[doc = "Bit 5 - TC2ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn tc2odis(&mut self) -> TC2ODIS_W<ODISRrs> {
        TC2ODIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - TD1ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn td1odis(&mut self) -> TD1ODIS_W<ODISRrs> {
        TD1ODIS_W::new(self, 6)
    }
    #[doc = "Bit 7 - TD2ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn td2odis(&mut self) -> TD2ODIS_W<ODISRrs> {
        TD2ODIS_W::new(self, 7)
    }
    #[doc = "Bit 8 - TE1ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn te1odis(&mut self) -> TE1ODIS_W<ODISRrs> {
        TE1ODIS_W::new(self, 8)
    }
    #[doc = "Bit 9 - TE2ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn te2odis(&mut self) -> TE2ODIS_W<ODISRrs> {
        TE2ODIS_W::new(self, 9)
    }
}
#[doc = "DISR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odisr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odisr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ODISRrs;
impl crate::RegisterSpec for ODISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odisr::R`](R) reader structure"]
impl crate::Readable for ODISRrs {}
#[doc = "`write(|w| ..)` method takes [`odisr::W`](W) writer structure"]
impl crate::Writable for ODISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ODISR to value 0"]
impl crate::Resettable for ODISRrs {
    const RESET_VALUE: u32 = 0;
}
