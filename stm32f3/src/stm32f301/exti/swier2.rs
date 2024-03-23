#[doc = "Register `SWIER2` reader"]
pub type R = crate::R<SWIER2rs>;
#[doc = "Register `SWIER2` writer"]
pub type W = crate::W<SWIER2rs>;
#[doc = "Software interrupt on line 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWIER32W {
    #[doc = "1: Generates an interrupt request"]
    Pend = 1,
}
impl From<SWIER32W> for bool {
    #[inline(always)]
    fn from(variant: SWIER32W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWIER32` reader - Software interrupt on line 32"]
pub type SWIER32_R = crate::BitReader<SWIER32W>;
impl SWIER32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWIER32W> {
        match self.bits {
            true => Some(SWIER32W::Pend),
            _ => None,
        }
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWIER32W::Pend
    }
}
#[doc = "Field `SWIER32` writer - Software interrupt on line 32"]
pub type SWIER32_W<'a, REG> = crate::BitWriter<'a, REG, SWIER32W>;
impl<'a, REG> SWIER32_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut crate::W<REG> {
        self.variant(SWIER32W::Pend)
    }
}
#[doc = "Field `SWIER33` reader - Software interrupt on line 33"]
pub use SWIER32_R as SWIER33_R;
#[doc = "Field `SWIER33` writer - Software interrupt on line 33"]
pub use SWIER32_W as SWIER33_W;
impl R {
    #[doc = "Bit 0 - Software interrupt on line 32"]
    #[inline(always)]
    pub fn swier32(&self) -> SWIER32_R {
        SWIER32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software interrupt on line 33"]
    #[inline(always)]
    pub fn swier33(&self) -> SWIER33_R {
        SWIER33_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software interrupt on line 32"]
    #[inline(always)]
    #[must_use]
    pub fn swier32(&mut self) -> SWIER32_W<SWIER2rs> {
        SWIER32_W::new(self, 0)
    }
    #[doc = "Bit 1 - Software interrupt on line 33"]
    #[inline(always)]
    #[must_use]
    pub fn swier33(&mut self) -> SWIER33_W<SWIER2rs> {
        SWIER33_W::new(self, 1)
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
