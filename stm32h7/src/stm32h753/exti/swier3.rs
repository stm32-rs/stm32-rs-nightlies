#[doc = "Register `SWIER3` reader"]
pub type R = crate::R<SWIER3rs>;
#[doc = "Register `SWIER3` writer"]
pub type W = crate::W<SWIER3rs>;
#[doc = "Software interrupt on line x+64\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWIER82W {
    #[doc = "1: Generates an interrupt request"]
    Pend = 1,
}
impl From<SWIER82W> for bool {
    #[inline(always)]
    fn from(variant: SWIER82W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWIER82` reader - Software interrupt on line x+64"]
pub type SWIER82_R = crate::BitReader<SWIER82W>;
impl SWIER82_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWIER82W> {
        match self.bits {
            true => Some(SWIER82W::Pend),
            _ => None,
        }
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWIER82W::Pend
    }
}
#[doc = "Field `SWIER82` writer - Software interrupt on line x+64"]
pub type SWIER82_W<'a, REG> = crate::BitWriter<'a, REG, SWIER82W>;
impl<'a, REG> SWIER82_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut crate::W<REG> {
        self.variant(SWIER82W::Pend)
    }
}
#[doc = "Field `SWIER84` reader - Software interrupt on line x+64"]
pub use SWIER82_R as SWIER84_R;
#[doc = "Field `SWIER85` reader - Software interrupt on line x+64"]
pub use SWIER82_R as SWIER85_R;
#[doc = "Field `SWIER86` reader - Software interrupt on line x+64"]
pub use SWIER82_R as SWIER86_R;
#[doc = "Field `SWIER84` writer - Software interrupt on line x+64"]
pub use SWIER82_W as SWIER84_W;
#[doc = "Field `SWIER85` writer - Software interrupt on line x+64"]
pub use SWIER82_W as SWIER85_W;
#[doc = "Field `SWIER86` writer - Software interrupt on line x+64"]
pub use SWIER82_W as SWIER86_W;
impl R {
    #[doc = "Bit 18 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier82(&self) -> SWIER82_R {
        SWIER82_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier84(&self) -> SWIER84_R {
        SWIER84_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier85(&self) -> SWIER85_R {
        SWIER85_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier86(&self) -> SWIER86_R {
        SWIER86_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Software interrupt on line x+64"]
    #[inline(always)]
    #[must_use]
    pub fn swier82(&mut self) -> SWIER82_W<SWIER3rs> {
        SWIER82_W::new(self, 18)
    }
    #[doc = "Bit 20 - Software interrupt on line x+64"]
    #[inline(always)]
    #[must_use]
    pub fn swier84(&mut self) -> SWIER84_W<SWIER3rs> {
        SWIER84_W::new(self, 20)
    }
    #[doc = "Bit 21 - Software interrupt on line x+64"]
    #[inline(always)]
    #[must_use]
    pub fn swier85(&mut self) -> SWIER85_W<SWIER3rs> {
        SWIER85_W::new(self, 21)
    }
    #[doc = "Bit 22 - Software interrupt on line x+64"]
    #[inline(always)]
    #[must_use]
    pub fn swier86(&mut self) -> SWIER86_W<SWIER3rs> {
        SWIER86_W::new(self, 22)
    }
}
#[doc = "EXTI software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWIER3rs;
impl crate::RegisterSpec for SWIER3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swier3::R`](R) reader structure"]
impl crate::Readable for SWIER3rs {}
#[doc = "`write(|w| ..)` method takes [`swier3::W`](W) writer structure"]
impl crate::Writable for SWIER3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWIER3 to value 0"]
impl crate::Resettable for SWIER3rs {
    const RESET_VALUE: u32 = 0;
}
