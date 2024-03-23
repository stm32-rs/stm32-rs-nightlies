#[doc = "Register `SWIER2` reader"]
pub type R = crate::R<SWIER2rs>;
#[doc = "Register `SWIER2` writer"]
pub type W = crate::W<SWIER2rs>;
#[doc = "Software interrupt on event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI34W {
    #[doc = "1: Generates an interrupt request"]
    Pend = 1,
}
impl From<SWI34W> for bool {
    #[inline(always)]
    fn from(variant: SWI34W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI34` reader - Software interrupt on event"]
pub type SWI34_R = crate::BitReader<SWI34W>;
impl SWI34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWI34W> {
        match self.bits {
            true => Some(SWI34W::Pend),
            _ => None,
        }
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWI34W::Pend
    }
}
#[doc = "Field `SWI34` writer - Software interrupt on event"]
pub type SWI34_W<'a, REG> = crate::BitWriter<'a, REG, SWI34W>;
impl<'a, REG> SWI34_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut crate::W<REG> {
        self.variant(SWI34W::Pend)
    }
}
#[doc = "Field `SWI40` reader - Software interrupt on event"]
pub use SWI34_R as SWI40_R;
#[doc = "Field `SWI41` reader - Software interrupt on event"]
pub use SWI34_R as SWI41_R;
#[doc = "Field `SWI45` reader - Software interrupt on event 45"]
pub use SWI34_R as SWI45_R;
#[doc = "Field `SWI40` writer - Software interrupt on event"]
pub use SWI34_W as SWI40_W;
#[doc = "Field `SWI41` writer - Software interrupt on event"]
pub use SWI34_W as SWI41_W;
#[doc = "Field `SWI45` writer - Software interrupt on event 45"]
pub use SWI34_W as SWI45_W;
impl R {
    #[doc = "Bit 2 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi34(&self) -> SWI34_R {
        SWI34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi40(&self) -> SWI40_R {
        SWI40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi41(&self) -> SWI41_R {
        SWI41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Software interrupt on event 45"]
    #[inline(always)]
    pub fn swi45(&self) -> SWI45_R {
        SWI45_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Software interrupt on event"]
    #[inline(always)]
    #[must_use]
    pub fn swi34(&mut self) -> SWI34_W<SWIER2rs> {
        SWI34_W::new(self, 2)
    }
    #[doc = "Bit 8 - Software interrupt on event"]
    #[inline(always)]
    #[must_use]
    pub fn swi40(&mut self) -> SWI40_W<SWIER2rs> {
        SWI40_W::new(self, 8)
    }
    #[doc = "Bit 9 - Software interrupt on event"]
    #[inline(always)]
    #[must_use]
    pub fn swi41(&mut self) -> SWI41_W<SWIER2rs> {
        SWI41_W::new(self, 9)
    }
    #[doc = "Bit 13 - Software interrupt on event 45"]
    #[inline(always)]
    #[must_use]
    pub fn swi45(&mut self) -> SWI45_W<SWIER2rs> {
        SWI45_W::new(self, 13)
    }
}
#[doc = "software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
