#[doc = "Register `PR2` reader"]
pub type R = crate::R<PR2rs>;
#[doc = "Register `PR2` writer"]
pub type W = crate::W<PR2rs>;
#[doc = "Pending bit 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF32R {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<PIF32R> for bool {
    #[inline(always)]
    fn from(variant: PIF32R) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIF32` reader - Pending bit 32"]
pub type PIF32_R = crate::BitReader<PIF32R>;
impl PIF32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIF32R {
        match self.bits {
            false => PIF32R::NotPending,
            true => PIF32R::Pending,
        }
    }
    #[doc = "No trigger request occurred"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PIF32R::NotPending
    }
    #[doc = "Selected trigger request occurred"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PIF32R::Pending
    }
}
#[doc = "Pending bit 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF32W {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<PIF32W> for bool {
    #[inline(always)]
    fn from(variant: PIF32W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIF32` writer - Pending bit 32"]
pub type PIF32_W<'a, REG> = crate::BitWriter1C<'a, REG, PIF32W>;
impl<'a, REG> PIF32_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PIF32W::Clear)
    }
}
#[doc = "Field `PIF33` reader - Pending bit 33"]
pub use PIF32_R as PIF33_R;
#[doc = "Field `PIF40` reader - Pending bit 40"]
pub use PIF32_R as PIF40_R;
#[doc = "Field `PIF41` reader - Pending bit 41"]
pub use PIF32_R as PIF41_R;
#[doc = "Field `PIF33` writer - Pending bit 33"]
pub use PIF32_W as PIF33_W;
#[doc = "Field `PIF40` writer - Pending bit 40"]
pub use PIF32_W as PIF40_W;
#[doc = "Field `PIF41` writer - Pending bit 41"]
pub use PIF32_W as PIF41_W;
impl R {
    #[doc = "Bit 0 - Pending bit 32"]
    #[inline(always)]
    pub fn pif32(&self) -> PIF32_R {
        PIF32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pending bit 33"]
    #[inline(always)]
    pub fn pif33(&self) -> PIF33_R {
        PIF33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Pending bit 40"]
    #[inline(always)]
    pub fn pif40(&self) -> PIF40_R {
        PIF40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pending bit 41"]
    #[inline(always)]
    pub fn pif41(&self) -> PIF41_R {
        PIF41_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pending bit 32"]
    #[inline(always)]
    #[must_use]
    pub fn pif32(&mut self) -> PIF32_W<PR2rs> {
        PIF32_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pending bit 33"]
    #[inline(always)]
    #[must_use]
    pub fn pif33(&mut self) -> PIF33_W<PR2rs> {
        PIF33_W::new(self, 1)
    }
    #[doc = "Bit 8 - Pending bit 40"]
    #[inline(always)]
    #[must_use]
    pub fn pif40(&mut self) -> PIF40_W<PR2rs> {
        PIF40_W::new(self, 8)
    }
    #[doc = "Bit 9 - Pending bit 41"]
    #[inline(always)]
    #[must_use]
    pub fn pif41(&mut self) -> PIF41_W<PR2rs> {
        PIF41_W::new(self, 9)
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0303;
}
#[doc = "`reset()` method sets PR2 to value 0"]
impl crate::Resettable for PR2rs {
    const RESET_VALUE: u32 = 0;
}
