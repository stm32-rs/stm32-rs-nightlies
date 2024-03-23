#[doc = "Register `SWIER2` reader"]
pub type R = crate::R<SWIER2rs>;
#[doc = "Register `SWIER2` writer"]
pub type W = crate::W<SWIER2rs>;
#[doc = "Software interrupt on line 35\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI35W {
    #[doc = "1: Generates an interrupt request"]
    Pend = 1,
}
impl From<SWI35W> for bool {
    #[inline(always)]
    fn from(variant: SWI35W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI35` reader - Software interrupt on line 35"]
pub type SWI35_R = crate::BitReader<SWI35W>;
impl SWI35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWI35W> {
        match self.bits {
            true => Some(SWI35W::Pend),
            _ => None,
        }
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWI35W::Pend
    }
}
#[doc = "Field `SWI35` writer - Software interrupt on line 35"]
pub type SWI35_W<'a, REG> = crate::BitWriter<'a, REG, SWI35W>;
impl<'a, REG> SWI35_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut crate::W<REG> {
        self.variant(SWI35W::Pend)
    }
}
#[doc = "Field `SWI36` reader - Software interrupt on line 36"]
pub use SWI35_R as SWI36_R;
#[doc = "Field `SWI37` reader - Software interrupt on line 37"]
pub use SWI35_R as SWI37_R;
#[doc = "Field `SWI38` reader - Software interrupt on line 38"]
pub use SWI35_R as SWI38_R;
#[doc = "Field `SWI36` writer - Software interrupt on line 36"]
pub use SWI35_W as SWI36_W;
#[doc = "Field `SWI37` writer - Software interrupt on line 37"]
pub use SWI35_W as SWI37_W;
#[doc = "Field `SWI38` writer - Software interrupt on line 38"]
pub use SWI35_W as SWI38_W;
impl R {
    #[doc = "Bit 3 - Software interrupt on line 35"]
    #[inline(always)]
    pub fn swi35(&self) -> SWI35_R {
        SWI35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software interrupt on line 36"]
    #[inline(always)]
    pub fn swi36(&self) -> SWI36_R {
        SWI36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software interrupt on line 37"]
    #[inline(always)]
    pub fn swi37(&self) -> SWI37_R {
        SWI37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software interrupt on line 38"]
    #[inline(always)]
    pub fn swi38(&self) -> SWI38_R {
        SWI38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Software interrupt on line 35"]
    #[inline(always)]
    #[must_use]
    pub fn swi35(&mut self) -> SWI35_W<SWIER2rs> {
        SWI35_W::new(self, 3)
    }
    #[doc = "Bit 4 - Software interrupt on line 36"]
    #[inline(always)]
    #[must_use]
    pub fn swi36(&mut self) -> SWI36_W<SWIER2rs> {
        SWI36_W::new(self, 4)
    }
    #[doc = "Bit 5 - Software interrupt on line 37"]
    #[inline(always)]
    #[must_use]
    pub fn swi37(&mut self) -> SWI37_W<SWIER2rs> {
        SWI37_W::new(self, 5)
    }
    #[doc = "Bit 6 - Software interrupt on line 38"]
    #[inline(always)]
    #[must_use]
    pub fn swi38(&mut self) -> SWI38_W<SWIER2rs> {
        SWI38_W::new(self, 6)
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
