#[doc = "Register `PR2` reader"]
pub type R = crate::R<PR2rs>;
#[doc = "Register `PR2` writer"]
pub type W = crate::W<PR2rs>;
#[doc = "Pending interrupt flag on line 35\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF35R {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<PIF35R> for bool {
    #[inline(always)]
    fn from(variant: PIF35R) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIF35` reader - Pending interrupt flag on line 35"]
pub type PIF35_R = crate::BitReader<PIF35R>;
impl PIF35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIF35R {
        match self.bits {
            false => PIF35R::NotPending,
            true => PIF35R::Pending,
        }
    }
    #[doc = "No trigger request occurred"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PIF35R::NotPending
    }
    #[doc = "Selected trigger request occurred"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PIF35R::Pending
    }
}
#[doc = "Pending interrupt flag on line 35\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF35W {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<PIF35W> for bool {
    #[inline(always)]
    fn from(variant: PIF35W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIF35` writer - Pending interrupt flag on line 35"]
pub type PIF35_W<'a, REG> = crate::BitWriter1C<'a, REG, PIF35W>;
impl<'a, REG> PIF35_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PIF35W::Clear)
    }
}
#[doc = "Field `PIF36` reader - Pending interrupt flag on line 36"]
pub use PIF35_R as PIF36_R;
#[doc = "Field `PIF37` reader - Pending interrupt flag on line 37"]
pub use PIF35_R as PIF37_R;
#[doc = "Field `PIF38` reader - Pending interrupt flag on line 38"]
pub use PIF35_R as PIF38_R;
#[doc = "Field `PIF36` writer - Pending interrupt flag on line 36"]
pub use PIF35_W as PIF36_W;
#[doc = "Field `PIF37` writer - Pending interrupt flag on line 37"]
pub use PIF35_W as PIF37_W;
#[doc = "Field `PIF38` writer - Pending interrupt flag on line 38"]
pub use PIF35_W as PIF38_W;
impl R {
    #[doc = "Bit 3 - Pending interrupt flag on line 35"]
    #[inline(always)]
    pub fn pif35(&self) -> PIF35_R {
        PIF35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pending interrupt flag on line 36"]
    #[inline(always)]
    pub fn pif36(&self) -> PIF36_R {
        PIF36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pending interrupt flag on line 37"]
    #[inline(always)]
    pub fn pif37(&self) -> PIF37_R {
        PIF37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pending interrupt flag on line 38"]
    #[inline(always)]
    pub fn pif38(&self) -> PIF38_R {
        PIF38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Pending interrupt flag on line 35"]
    #[inline(always)]
    #[must_use]
    pub fn pif35(&mut self) -> PIF35_W<PR2rs> {
        PIF35_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pending interrupt flag on line 36"]
    #[inline(always)]
    #[must_use]
    pub fn pif36(&mut self) -> PIF36_W<PR2rs> {
        PIF36_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pending interrupt flag on line 37"]
    #[inline(always)]
    #[must_use]
    pub fn pif37(&mut self) -> PIF37_W<PR2rs> {
        PIF37_W::new(self, 5)
    }
    #[doc = "Bit 6 - Pending interrupt flag on line 38"]
    #[inline(always)]
    #[must_use]
    pub fn pif38(&mut self) -> PIF38_W<PR2rs> {
        PIF38_W::new(self, 6)
    }
}
#[doc = "Pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PR2rs;
impl crate::RegisterSpec for PR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr2::R`](R) reader structure"]
impl crate::Readable for PR2rs {}
#[doc = "`write(|w| ..)` method takes [`pr2::W`](W) writer structure"]
impl crate::Writable for PR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x78;
}
#[doc = "`reset()` method sets PR2 to value 0"]
impl crate::Resettable for PR2rs {
    const RESET_VALUE: u32 = 0;
}
