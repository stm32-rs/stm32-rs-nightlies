#[doc = "Register `SWIER2` reader"]
pub type R = crate::R<SWIER2rs>;
#[doc = "Register `SWIER2` writer"]
pub type W = crate::W<SWIER2rs>;
#[doc = "Software interrupt on line 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI32W {
    #[doc = "1: Generates an interrupt request"]
    Pend = 1,
}
impl From<SWI32W> for bool {
    #[inline(always)]
    fn from(variant: SWI32W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI32` reader - Software interrupt on line 32"]
pub type SWI32_R = crate::BitReader<SWI32W>;
impl SWI32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWI32W> {
        match self.bits {
            true => Some(SWI32W::Pend),
            _ => None,
        }
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWI32W::Pend
    }
}
#[doc = "Field `SWI32` writer - Software interrupt on line 32"]
pub type SWI32_W<'a, REG> = crate::BitWriter<'a, REG, SWI32W>;
impl<'a, REG> SWI32_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut crate::W<REG> {
        self.variant(SWI32W::Pend)
    }
}
#[doc = "Field `SWI33` reader - Software interrupt on line 33"]
pub use SWI32_R as SWI33_R;
#[doc = "Field `SWI40` reader - Software interrupt on line 40"]
pub use SWI32_R as SWI40_R;
#[doc = "Field `SWI41` reader - Software interrupt on line 41"]
pub use SWI32_R as SWI41_R;
#[doc = "Field `SWI33` writer - Software interrupt on line 33"]
pub use SWI32_W as SWI33_W;
#[doc = "Field `SWI40` writer - Software interrupt on line 40"]
pub use SWI32_W as SWI40_W;
#[doc = "Field `SWI41` writer - Software interrupt on line 41"]
pub use SWI32_W as SWI41_W;
impl R {
    #[doc = "Bit 0 - Software interrupt on line 32"]
    #[inline(always)]
    pub fn swi32(&self) -> SWI32_R {
        SWI32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software interrupt on line 33"]
    #[inline(always)]
    pub fn swi33(&self) -> SWI33_R {
        SWI33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Software interrupt on line 40"]
    #[inline(always)]
    pub fn swi40(&self) -> SWI40_R {
        SWI40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software interrupt on line 41"]
    #[inline(always)]
    pub fn swi41(&self) -> SWI41_R {
        SWI41_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software interrupt on line 32"]
    #[inline(always)]
    #[must_use]
    pub fn swi32(&mut self) -> SWI32_W<SWIER2rs> {
        SWI32_W::new(self, 0)
    }
    #[doc = "Bit 1 - Software interrupt on line 33"]
    #[inline(always)]
    #[must_use]
    pub fn swi33(&mut self) -> SWI33_W<SWIER2rs> {
        SWI33_W::new(self, 1)
    }
    #[doc = "Bit 8 - Software interrupt on line 40"]
    #[inline(always)]
    #[must_use]
    pub fn swi40(&mut self) -> SWI40_W<SWIER2rs> {
        SWI40_W::new(self, 8)
    }
    #[doc = "Bit 9 - Software interrupt on line 41"]
    #[inline(always)]
    #[must_use]
    pub fn swi41(&mut self) -> SWI41_W<SWIER2rs> {
        SWI41_W::new(self, 9)
    }
}
#[doc = "Software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWIER2rs;
impl crate::RegisterSpec for SWIER2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swier2::R`](R) reader structure"]
impl crate::Readable for SWIER2rs {}
#[doc = "`write(|w| ..)` method takes [`swier2::W`](W) writer structure"]
impl crate::Writable for SWIER2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWIER2 to value 0"]
impl crate::Resettable for SWIER2rs {
    const RESET_VALUE: u32 = 0;
}
