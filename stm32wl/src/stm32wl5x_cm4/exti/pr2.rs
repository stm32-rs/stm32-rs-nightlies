#[doc = "Register `PR2` reader"]
pub type R = crate::R<PR2rs>;
#[doc = "Register `PR2` writer"]
pub type W = crate::W<PR2rs>;
#[doc = "Configurable event inputs 33 Pending bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF34R {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<PIF34R> for bool {
    #[inline(always)]
    fn from(variant: PIF34R) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIF34` reader - Configurable event inputs 33 Pending bit."]
pub type PIF34_R = crate::BitReader<PIF34R>;
impl PIF34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIF34R {
        match self.bits {
            false => PIF34R::NotPending,
            true => PIF34R::Pending,
        }
    }
    #[doc = "No trigger request occurred"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PIF34R::NotPending
    }
    #[doc = "Selected trigger request occurred"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PIF34R::Pending
    }
}
#[doc = "Configurable event inputs 33 Pending bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF34W {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<PIF34W> for bool {
    #[inline(always)]
    fn from(variant: PIF34W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIF34` writer - Configurable event inputs 33 Pending bit."]
pub type PIF34_W<'a, REG> = crate::BitWriter1C<'a, REG, PIF34W>;
impl<'a, REG> PIF34_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PIF34W::Clear)
    }
}
#[doc = "Field `PIF40` reader - Configurable event inputs 40_41 Pending bit."]
pub use PIF34_R as PIF40_R;
#[doc = "Field `PIF41` reader - Configurable event inputs 40_41 Pending bit."]
pub use PIF34_R as PIF41_R;
#[doc = "Field `PIF45` reader - Configurable event inputs 45 Pending bit."]
pub use PIF34_R as PIF45_R;
#[doc = "Field `PIF40` writer - Configurable event inputs 40_41 Pending bit."]
pub use PIF34_W as PIF40_W;
#[doc = "Field `PIF41` writer - Configurable event inputs 40_41 Pending bit."]
pub use PIF34_W as PIF41_W;
#[doc = "Field `PIF45` writer - Configurable event inputs 45 Pending bit."]
pub use PIF34_W as PIF45_W;
impl R {
    #[doc = "Bit 2 - Configurable event inputs 33 Pending bit."]
    #[inline(always)]
    pub fn pif34(&self) -> PIF34_R {
        PIF34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Configurable event inputs 40_41 Pending bit."]
    #[inline(always)]
    pub fn pif40(&self) -> PIF40_R {
        PIF40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configurable event inputs 40_41 Pending bit."]
    #[inline(always)]
    pub fn pif41(&self) -> PIF41_R {
        PIF41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Configurable event inputs 45 Pending bit."]
    #[inline(always)]
    pub fn pif45(&self) -> PIF45_R {
        PIF45_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Configurable event inputs 33 Pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn pif34(&mut self) -> PIF34_W<PR2rs> {
        PIF34_W::new(self, 2)
    }
    #[doc = "Bit 8 - Configurable event inputs 40_41 Pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn pif40(&mut self) -> PIF40_W<PR2rs> {
        PIF40_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configurable event inputs 40_41 Pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn pif41(&mut self) -> PIF41_W<PR2rs> {
        PIF41_W::new(self, 9)
    }
    #[doc = "Bit 13 - Configurable event inputs 45 Pending bit."]
    #[inline(always)]
    #[must_use]
    pub fn pif45(&mut self) -> PIF45_W<PR2rs> {
        PIF45_W::new(self, 13)
    }
}
#[doc = "pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x2304;
}
#[doc = "`reset()` method sets PR2 to value 0"]
impl crate::Resettable for PR2rs {
    const RESET_VALUE: u32 = 0;
}
