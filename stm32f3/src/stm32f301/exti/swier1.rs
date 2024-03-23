#[doc = "Register `SWIER1` reader"]
pub type R = crate::R<SWIER1rs>;
#[doc = "Register `SWIER1` writer"]
pub type W = crate::W<SWIER1rs>;
#[doc = "Software Interrupt on line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWIER0W {
    #[doc = "1: Generates an interrupt request"]
    Pend = 1,
}
impl From<SWIER0W> for bool {
    #[inline(always)]
    fn from(variant: SWIER0W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWIER0` reader - Software Interrupt on line 0"]
pub type SWIER0_R = crate::BitReader<SWIER0W>;
impl SWIER0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWIER0W> {
        match self.bits {
            true => Some(SWIER0W::Pend),
            _ => None,
        }
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWIER0W::Pend
    }
}
#[doc = "Field `SWIER0` writer - Software Interrupt on line 0"]
pub type SWIER0_W<'a, REG> = crate::BitWriter<'a, REG, SWIER0W>;
impl<'a, REG> SWIER0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut crate::W<REG> {
        self.variant(SWIER0W::Pend)
    }
}
#[doc = "Field `SWIER1` reader - Software Interrupt on line 1"]
pub use SWIER0_R as SWIER1_R;
#[doc = "Field `SWIER2` reader - Software Interrupt on line 2"]
pub use SWIER0_R as SWIER2_R;
#[doc = "Field `SWIER3` reader - Software Interrupt on line 3"]
pub use SWIER0_R as SWIER3_R;
#[doc = "Field `SWIER4` reader - Software Interrupt on line 4"]
pub use SWIER0_R as SWIER4_R;
#[doc = "Field `SWIER5` reader - Software Interrupt on line 5"]
pub use SWIER0_R as SWIER5_R;
#[doc = "Field `SWIER6` reader - Software Interrupt on line 6"]
pub use SWIER0_R as SWIER6_R;
#[doc = "Field `SWIER7` reader - Software Interrupt on line 7"]
pub use SWIER0_R as SWIER7_R;
#[doc = "Field `SWIER8` reader - Software Interrupt on line 8"]
pub use SWIER0_R as SWIER8_R;
#[doc = "Field `SWIER9` reader - Software Interrupt on line 9"]
pub use SWIER0_R as SWIER9_R;
#[doc = "Field `SWIER10` reader - Software Interrupt on line 10"]
pub use SWIER0_R as SWIER10_R;
#[doc = "Field `SWIER11` reader - Software Interrupt on line 11"]
pub use SWIER0_R as SWIER11_R;
#[doc = "Field `SWIER12` reader - Software Interrupt on line 12"]
pub use SWIER0_R as SWIER12_R;
#[doc = "Field `SWIER13` reader - Software Interrupt on line 13"]
pub use SWIER0_R as SWIER13_R;
#[doc = "Field `SWIER14` reader - Software Interrupt on line 14"]
pub use SWIER0_R as SWIER14_R;
#[doc = "Field `SWIER15` reader - Software Interrupt on line 15"]
pub use SWIER0_R as SWIER15_R;
#[doc = "Field `SWIER16` reader - Software Interrupt on line 16"]
pub use SWIER0_R as SWIER16_R;
#[doc = "Field `SWIER17` reader - Software Interrupt on line 17"]
pub use SWIER0_R as SWIER17_R;
#[doc = "Field `SWIER18` reader - Software Interrupt on line 18"]
pub use SWIER0_R as SWIER18_R;
#[doc = "Field `SWIER19` reader - Software Interrupt on line 19"]
pub use SWIER0_R as SWIER19_R;
#[doc = "Field `SWIER20` reader - Software Interrupt on line 20"]
pub use SWIER0_R as SWIER20_R;
#[doc = "Field `SWIER21` reader - Software Interrupt on line 21"]
pub use SWIER0_R as SWIER21_R;
#[doc = "Field `SWIER22` reader - Software Interrupt on line 22"]
pub use SWIER0_R as SWIER22_R;
#[doc = "Field `SWIER29` reader - Software Interrupt on line 29"]
pub use SWIER0_R as SWIER29_R;
#[doc = "Field `SWIER30` reader - Software Interrupt on line 309"]
pub use SWIER0_R as SWIER30_R;
#[doc = "Field `SWIER31` reader - Software Interrupt on line 319"]
pub use SWIER0_R as SWIER31_R;
#[doc = "Field `SWIER1` writer - Software Interrupt on line 1"]
pub use SWIER0_W as SWIER1_W;
#[doc = "Field `SWIER2` writer - Software Interrupt on line 2"]
pub use SWIER0_W as SWIER2_W;
#[doc = "Field `SWIER3` writer - Software Interrupt on line 3"]
pub use SWIER0_W as SWIER3_W;
#[doc = "Field `SWIER4` writer - Software Interrupt on line 4"]
pub use SWIER0_W as SWIER4_W;
#[doc = "Field `SWIER5` writer - Software Interrupt on line 5"]
pub use SWIER0_W as SWIER5_W;
#[doc = "Field `SWIER6` writer - Software Interrupt on line 6"]
pub use SWIER0_W as SWIER6_W;
#[doc = "Field `SWIER7` writer - Software Interrupt on line 7"]
pub use SWIER0_W as SWIER7_W;
#[doc = "Field `SWIER8` writer - Software Interrupt on line 8"]
pub use SWIER0_W as SWIER8_W;
#[doc = "Field `SWIER9` writer - Software Interrupt on line 9"]
pub use SWIER0_W as SWIER9_W;
#[doc = "Field `SWIER10` writer - Software Interrupt on line 10"]
pub use SWIER0_W as SWIER10_W;
#[doc = "Field `SWIER11` writer - Software Interrupt on line 11"]
pub use SWIER0_W as SWIER11_W;
#[doc = "Field `SWIER12` writer - Software Interrupt on line 12"]
pub use SWIER0_W as SWIER12_W;
#[doc = "Field `SWIER13` writer - Software Interrupt on line 13"]
pub use SWIER0_W as SWIER13_W;
#[doc = "Field `SWIER14` writer - Software Interrupt on line 14"]
pub use SWIER0_W as SWIER14_W;
#[doc = "Field `SWIER15` writer - Software Interrupt on line 15"]
pub use SWIER0_W as SWIER15_W;
#[doc = "Field `SWIER16` writer - Software Interrupt on line 16"]
pub use SWIER0_W as SWIER16_W;
#[doc = "Field `SWIER17` writer - Software Interrupt on line 17"]
pub use SWIER0_W as SWIER17_W;
#[doc = "Field `SWIER18` writer - Software Interrupt on line 18"]
pub use SWIER0_W as SWIER18_W;
#[doc = "Field `SWIER19` writer - Software Interrupt on line 19"]
pub use SWIER0_W as SWIER19_W;
#[doc = "Field `SWIER20` writer - Software Interrupt on line 20"]
pub use SWIER0_W as SWIER20_W;
#[doc = "Field `SWIER21` writer - Software Interrupt on line 21"]
pub use SWIER0_W as SWIER21_W;
#[doc = "Field `SWIER22` writer - Software Interrupt on line 22"]
pub use SWIER0_W as SWIER22_W;
#[doc = "Field `SWIER29` writer - Software Interrupt on line 29"]
pub use SWIER0_W as SWIER29_W;
#[doc = "Field `SWIER30` writer - Software Interrupt on line 309"]
pub use SWIER0_W as SWIER30_W;
#[doc = "Field `SWIER31` writer - Software Interrupt on line 319"]
pub use SWIER0_W as SWIER31_W;
impl R {
    #[doc = "Bit 0 - Software Interrupt on line 0"]
    #[inline(always)]
    pub fn swier0(&self) -> SWIER0_R {
        SWIER0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Interrupt on line 1"]
    #[inline(always)]
    pub fn swier1(&self) -> SWIER1_R {
        SWIER1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Interrupt on line 2"]
    #[inline(always)]
    pub fn swier2(&self) -> SWIER2_R {
        SWIER2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software Interrupt on line 3"]
    #[inline(always)]
    pub fn swier3(&self) -> SWIER3_R {
        SWIER3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software Interrupt on line 4"]
    #[inline(always)]
    pub fn swier4(&self) -> SWIER4_R {
        SWIER4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software Interrupt on line 5"]
    #[inline(always)]
    pub fn swier5(&self) -> SWIER5_R {
        SWIER5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software Interrupt on line 6"]
    #[inline(always)]
    pub fn swier6(&self) -> SWIER6_R {
        SWIER6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software Interrupt on line 7"]
    #[inline(always)]
    pub fn swier7(&self) -> SWIER7_R {
        SWIER7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software Interrupt on line 8"]
    #[inline(always)]
    pub fn swier8(&self) -> SWIER8_R {
        SWIER8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software Interrupt on line 9"]
    #[inline(always)]
    pub fn swier9(&self) -> SWIER9_R {
        SWIER9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software Interrupt on line 10"]
    #[inline(always)]
    pub fn swier10(&self) -> SWIER10_R {
        SWIER10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software Interrupt on line 11"]
    #[inline(always)]
    pub fn swier11(&self) -> SWIER11_R {
        SWIER11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software Interrupt on line 12"]
    #[inline(always)]
    pub fn swier12(&self) -> SWIER12_R {
        SWIER12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Software Interrupt on line 13"]
    #[inline(always)]
    pub fn swier13(&self) -> SWIER13_R {
        SWIER13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software Interrupt on line 14"]
    #[inline(always)]
    pub fn swier14(&self) -> SWIER14_R {
        SWIER14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Software Interrupt on line 15"]
    #[inline(always)]
    pub fn swier15(&self) -> SWIER15_R {
        SWIER15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Software Interrupt on line 16"]
    #[inline(always)]
    pub fn swier16(&self) -> SWIER16_R {
        SWIER16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Software Interrupt on line 17"]
    #[inline(always)]
    pub fn swier17(&self) -> SWIER17_R {
        SWIER17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Software Interrupt on line 18"]
    #[inline(always)]
    pub fn swier18(&self) -> SWIER18_R {
        SWIER18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Software Interrupt on line 19"]
    #[inline(always)]
    pub fn swier19(&self) -> SWIER19_R {
        SWIER19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Software Interrupt on line 20"]
    #[inline(always)]
    pub fn swier20(&self) -> SWIER20_R {
        SWIER20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Software Interrupt on line 21"]
    #[inline(always)]
    pub fn swier21(&self) -> SWIER21_R {
        SWIER21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Software Interrupt on line 22"]
    #[inline(always)]
    pub fn swier22(&self) -> SWIER22_R {
        SWIER22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 29 - Software Interrupt on line 29"]
    #[inline(always)]
    pub fn swier29(&self) -> SWIER29_R {
        SWIER29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Software Interrupt on line 309"]
    #[inline(always)]
    pub fn swier30(&self) -> SWIER30_R {
        SWIER30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Software Interrupt on line 319"]
    #[inline(always)]
    pub fn swier31(&self) -> SWIER31_R {
        SWIER31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Interrupt on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn swier0(&mut self) -> SWIER0_W<SWIER1rs> {
        SWIER0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Software Interrupt on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn swier1(&mut self) -> SWIER1_W<SWIER1rs> {
        SWIER1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Software Interrupt on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn swier2(&mut self) -> SWIER2_W<SWIER1rs> {
        SWIER2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Software Interrupt on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn swier3(&mut self) -> SWIER3_W<SWIER1rs> {
        SWIER3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Software Interrupt on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn swier4(&mut self) -> SWIER4_W<SWIER1rs> {
        SWIER4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Software Interrupt on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn swier5(&mut self) -> SWIER5_W<SWIER1rs> {
        SWIER5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Software Interrupt on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn swier6(&mut self) -> SWIER6_W<SWIER1rs> {
        SWIER6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Software Interrupt on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn swier7(&mut self) -> SWIER7_W<SWIER1rs> {
        SWIER7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Software Interrupt on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn swier8(&mut self) -> SWIER8_W<SWIER1rs> {
        SWIER8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Software Interrupt on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn swier9(&mut self) -> SWIER9_W<SWIER1rs> {
        SWIER9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Software Interrupt on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn swier10(&mut self) -> SWIER10_W<SWIER1rs> {
        SWIER10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Software Interrupt on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn swier11(&mut self) -> SWIER11_W<SWIER1rs> {
        SWIER11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Software Interrupt on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn swier12(&mut self) -> SWIER12_W<SWIER1rs> {
        SWIER12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Software Interrupt on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn swier13(&mut self) -> SWIER13_W<SWIER1rs> {
        SWIER13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Software Interrupt on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn swier14(&mut self) -> SWIER14_W<SWIER1rs> {
        SWIER14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Software Interrupt on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn swier15(&mut self) -> SWIER15_W<SWIER1rs> {
        SWIER15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Software Interrupt on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn swier16(&mut self) -> SWIER16_W<SWIER1rs> {
        SWIER16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Software Interrupt on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn swier17(&mut self) -> SWIER17_W<SWIER1rs> {
        SWIER17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Software Interrupt on line 18"]
    #[inline(always)]
    #[must_use]
    pub fn swier18(&mut self) -> SWIER18_W<SWIER1rs> {
        SWIER18_W::new(self, 18)
    }
    #[doc = "Bit 19 - Software Interrupt on line 19"]
    #[inline(always)]
    #[must_use]
    pub fn swier19(&mut self) -> SWIER19_W<SWIER1rs> {
        SWIER19_W::new(self, 19)
    }
    #[doc = "Bit 20 - Software Interrupt on line 20"]
    #[inline(always)]
    #[must_use]
    pub fn swier20(&mut self) -> SWIER20_W<SWIER1rs> {
        SWIER20_W::new(self, 20)
    }
    #[doc = "Bit 21 - Software Interrupt on line 21"]
    #[inline(always)]
    #[must_use]
    pub fn swier21(&mut self) -> SWIER21_W<SWIER1rs> {
        SWIER21_W::new(self, 21)
    }
    #[doc = "Bit 22 - Software Interrupt on line 22"]
    #[inline(always)]
    #[must_use]
    pub fn swier22(&mut self) -> SWIER22_W<SWIER1rs> {
        SWIER22_W::new(self, 22)
    }
    #[doc = "Bit 29 - Software Interrupt on line 29"]
    #[inline(always)]
    #[must_use]
    pub fn swier29(&mut self) -> SWIER29_W<SWIER1rs> {
        SWIER29_W::new(self, 29)
    }
    #[doc = "Bit 30 - Software Interrupt on line 309"]
    #[inline(always)]
    #[must_use]
    pub fn swier30(&mut self) -> SWIER30_W<SWIER1rs> {
        SWIER30_W::new(self, 30)
    }
    #[doc = "Bit 31 - Software Interrupt on line 319"]
    #[inline(always)]
    #[must_use]
    pub fn swier31(&mut self) -> SWIER31_W<SWIER1rs> {
        SWIER31_W::new(self, 31)
    }
}
#[doc = "Software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWIER1rs;
impl crate::RegisterSpec for SWIER1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swier1::R`](R) reader structure"]
impl crate::Readable for SWIER1rs {}
#[doc = "`write(|w| ..)` method takes [`swier1::W`](W) writer structure"]
impl crate::Writable for SWIER1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWIER1 to value 0"]
impl crate::Resettable for SWIER1rs {
    const RESET_VALUE: u32 = 0;
}
