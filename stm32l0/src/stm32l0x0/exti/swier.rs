#[doc = "Register `SWIER` reader"]
pub type R = crate::R<SWIERrs>;
#[doc = "Register `SWIER` writer"]
pub type W = crate::W<SWIERrs>;
#[doc = "Software Interrupt on line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI0W {
    #[doc = "1: Generates an interrupt request"]
    Pend = 1,
}
impl From<SWI0W> for bool {
    #[inline(always)]
    fn from(variant: SWI0W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI0` reader - Software Interrupt on line 0"]
pub type SWI0_R = crate::BitReader<SWI0W>;
impl SWI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWI0W> {
        match self.bits {
            true => Some(SWI0W::Pend),
            _ => None,
        }
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWI0W::Pend
    }
}
#[doc = "Field `SWI0` writer - Software Interrupt on line 0"]
pub type SWI0_W<'a, REG> = crate::BitWriter<'a, REG, SWI0W>;
impl<'a, REG> SWI0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut crate::W<REG> {
        self.variant(SWI0W::Pend)
    }
}
#[doc = "Field `SWI1` reader - Software Interrupt on line 1"]
pub use SWI0_R as SWI1_R;
#[doc = "Field `SWI2` reader - Software Interrupt on line 2"]
pub use SWI0_R as SWI2_R;
#[doc = "Field `SWI3` reader - Software Interrupt on line 3"]
pub use SWI0_R as SWI3_R;
#[doc = "Field `SWI4` reader - Software Interrupt on line 4"]
pub use SWI0_R as SWI4_R;
#[doc = "Field `SWI5` reader - Software Interrupt on line 5"]
pub use SWI0_R as SWI5_R;
#[doc = "Field `SWI6` reader - Software Interrupt on line 6"]
pub use SWI0_R as SWI6_R;
#[doc = "Field `SWI7` reader - Software Interrupt on line 7"]
pub use SWI0_R as SWI7_R;
#[doc = "Field `SWI8` reader - Software Interrupt on line 8"]
pub use SWI0_R as SWI8_R;
#[doc = "Field `SWI9` reader - Software Interrupt on line 9"]
pub use SWI0_R as SWI9_R;
#[doc = "Field `SWI10` reader - Software Interrupt on line 10"]
pub use SWI0_R as SWI10_R;
#[doc = "Field `SWI11` reader - Software Interrupt on line 11"]
pub use SWI0_R as SWI11_R;
#[doc = "Field `SWI12` reader - Software Interrupt on line 12"]
pub use SWI0_R as SWI12_R;
#[doc = "Field `SWI13` reader - Software Interrupt on line 13"]
pub use SWI0_R as SWI13_R;
#[doc = "Field `SWI14` reader - Software Interrupt on line 14"]
pub use SWI0_R as SWI14_R;
#[doc = "Field `SWI15` reader - Software Interrupt on line 15"]
pub use SWI0_R as SWI15_R;
#[doc = "Field `SWI16` reader - Software Interrupt on line 16"]
pub use SWI0_R as SWI16_R;
#[doc = "Field `SWI17` reader - Software Interrupt on line 17"]
pub use SWI0_R as SWI17_R;
#[doc = "Field `SWI19` reader - Software Interrupt on line 19"]
pub use SWI0_R as SWI19_R;
#[doc = "Field `SWI20` reader - Software Interrupt on line 20"]
pub use SWI0_R as SWI20_R;
#[doc = "Field `SWI21` reader - Software Interrupt on line 21"]
pub use SWI0_R as SWI21_R;
#[doc = "Field `SWI22` reader - Software Interrupt on line 22"]
pub use SWI0_R as SWI22_R;
#[doc = "Field `SWI1` writer - Software Interrupt on line 1"]
pub use SWI0_W as SWI1_W;
#[doc = "Field `SWI2` writer - Software Interrupt on line 2"]
pub use SWI0_W as SWI2_W;
#[doc = "Field `SWI3` writer - Software Interrupt on line 3"]
pub use SWI0_W as SWI3_W;
#[doc = "Field `SWI4` writer - Software Interrupt on line 4"]
pub use SWI0_W as SWI4_W;
#[doc = "Field `SWI5` writer - Software Interrupt on line 5"]
pub use SWI0_W as SWI5_W;
#[doc = "Field `SWI6` writer - Software Interrupt on line 6"]
pub use SWI0_W as SWI6_W;
#[doc = "Field `SWI7` writer - Software Interrupt on line 7"]
pub use SWI0_W as SWI7_W;
#[doc = "Field `SWI8` writer - Software Interrupt on line 8"]
pub use SWI0_W as SWI8_W;
#[doc = "Field `SWI9` writer - Software Interrupt on line 9"]
pub use SWI0_W as SWI9_W;
#[doc = "Field `SWI10` writer - Software Interrupt on line 10"]
pub use SWI0_W as SWI10_W;
#[doc = "Field `SWI11` writer - Software Interrupt on line 11"]
pub use SWI0_W as SWI11_W;
#[doc = "Field `SWI12` writer - Software Interrupt on line 12"]
pub use SWI0_W as SWI12_W;
#[doc = "Field `SWI13` writer - Software Interrupt on line 13"]
pub use SWI0_W as SWI13_W;
#[doc = "Field `SWI14` writer - Software Interrupt on line 14"]
pub use SWI0_W as SWI14_W;
#[doc = "Field `SWI15` writer - Software Interrupt on line 15"]
pub use SWI0_W as SWI15_W;
#[doc = "Field `SWI16` writer - Software Interrupt on line 16"]
pub use SWI0_W as SWI16_W;
#[doc = "Field `SWI17` writer - Software Interrupt on line 17"]
pub use SWI0_W as SWI17_W;
#[doc = "Field `SWI19` writer - Software Interrupt on line 19"]
pub use SWI0_W as SWI19_W;
#[doc = "Field `SWI20` writer - Software Interrupt on line 20"]
pub use SWI0_W as SWI20_W;
#[doc = "Field `SWI21` writer - Software Interrupt on line 21"]
pub use SWI0_W as SWI21_W;
#[doc = "Field `SWI22` writer - Software Interrupt on line 22"]
pub use SWI0_W as SWI22_W;
impl R {
    #[doc = "Bit 0 - Software Interrupt on line 0"]
    #[inline(always)]
    pub fn swi0(&self) -> SWI0_R {
        SWI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Interrupt on line 1"]
    #[inline(always)]
    pub fn swi1(&self) -> SWI1_R {
        SWI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Interrupt on line 2"]
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software Interrupt on line 3"]
    #[inline(always)]
    pub fn swi3(&self) -> SWI3_R {
        SWI3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software Interrupt on line 4"]
    #[inline(always)]
    pub fn swi4(&self) -> SWI4_R {
        SWI4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software Interrupt on line 5"]
    #[inline(always)]
    pub fn swi5(&self) -> SWI5_R {
        SWI5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software Interrupt on line 6"]
    #[inline(always)]
    pub fn swi6(&self) -> SWI6_R {
        SWI6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software Interrupt on line 7"]
    #[inline(always)]
    pub fn swi7(&self) -> SWI7_R {
        SWI7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software Interrupt on line 8"]
    #[inline(always)]
    pub fn swi8(&self) -> SWI8_R {
        SWI8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software Interrupt on line 9"]
    #[inline(always)]
    pub fn swi9(&self) -> SWI9_R {
        SWI9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software Interrupt on line 10"]
    #[inline(always)]
    pub fn swi10(&self) -> SWI10_R {
        SWI10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software Interrupt on line 11"]
    #[inline(always)]
    pub fn swi11(&self) -> SWI11_R {
        SWI11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software Interrupt on line 12"]
    #[inline(always)]
    pub fn swi12(&self) -> SWI12_R {
        SWI12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Software Interrupt on line 13"]
    #[inline(always)]
    pub fn swi13(&self) -> SWI13_R {
        SWI13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software Interrupt on line 14"]
    #[inline(always)]
    pub fn swi14(&self) -> SWI14_R {
        SWI14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Software Interrupt on line 15"]
    #[inline(always)]
    pub fn swi15(&self) -> SWI15_R {
        SWI15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Software Interrupt on line 16"]
    #[inline(always)]
    pub fn swi16(&self) -> SWI16_R {
        SWI16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Software Interrupt on line 17"]
    #[inline(always)]
    pub fn swi17(&self) -> SWI17_R {
        SWI17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Software Interrupt on line 19"]
    #[inline(always)]
    pub fn swi19(&self) -> SWI19_R {
        SWI19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Software Interrupt on line 20"]
    #[inline(always)]
    pub fn swi20(&self) -> SWI20_R {
        SWI20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Software Interrupt on line 21"]
    #[inline(always)]
    pub fn swi21(&self) -> SWI21_R {
        SWI21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Software Interrupt on line 22"]
    #[inline(always)]
    pub fn swi22(&self) -> SWI22_R {
        SWI22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Interrupt on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn swi0(&mut self) -> SWI0_W<SWIERrs> {
        SWI0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Software Interrupt on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn swi1(&mut self) -> SWI1_W<SWIERrs> {
        SWI1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Software Interrupt on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn swi2(&mut self) -> SWI2_W<SWIERrs> {
        SWI2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Software Interrupt on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn swi3(&mut self) -> SWI3_W<SWIERrs> {
        SWI3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Software Interrupt on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn swi4(&mut self) -> SWI4_W<SWIERrs> {
        SWI4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Software Interrupt on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn swi5(&mut self) -> SWI5_W<SWIERrs> {
        SWI5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Software Interrupt on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn swi6(&mut self) -> SWI6_W<SWIERrs> {
        SWI6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Software Interrupt on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn swi7(&mut self) -> SWI7_W<SWIERrs> {
        SWI7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Software Interrupt on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn swi8(&mut self) -> SWI8_W<SWIERrs> {
        SWI8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Software Interrupt on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn swi9(&mut self) -> SWI9_W<SWIERrs> {
        SWI9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Software Interrupt on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn swi10(&mut self) -> SWI10_W<SWIERrs> {
        SWI10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Software Interrupt on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn swi11(&mut self) -> SWI11_W<SWIERrs> {
        SWI11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Software Interrupt on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn swi12(&mut self) -> SWI12_W<SWIERrs> {
        SWI12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Software Interrupt on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn swi13(&mut self) -> SWI13_W<SWIERrs> {
        SWI13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Software Interrupt on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn swi14(&mut self) -> SWI14_W<SWIERrs> {
        SWI14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Software Interrupt on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn swi15(&mut self) -> SWI15_W<SWIERrs> {
        SWI15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Software Interrupt on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn swi16(&mut self) -> SWI16_W<SWIERrs> {
        SWI16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Software Interrupt on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn swi17(&mut self) -> SWI17_W<SWIERrs> {
        SWI17_W::new(self, 17)
    }
    #[doc = "Bit 19 - Software Interrupt on line 19"]
    #[inline(always)]
    #[must_use]
    pub fn swi19(&mut self) -> SWI19_W<SWIERrs> {
        SWI19_W::new(self, 19)
    }
    #[doc = "Bit 20 - Software Interrupt on line 20"]
    #[inline(always)]
    #[must_use]
    pub fn swi20(&mut self) -> SWI20_W<SWIERrs> {
        SWI20_W::new(self, 20)
    }
    #[doc = "Bit 21 - Software Interrupt on line 21"]
    #[inline(always)]
    #[must_use]
    pub fn swi21(&mut self) -> SWI21_W<SWIERrs> {
        SWI21_W::new(self, 21)
    }
    #[doc = "Bit 22 - Software Interrupt on line 22"]
    #[inline(always)]
    #[must_use]
    pub fn swi22(&mut self) -> SWI22_W<SWIERrs> {
        SWI22_W::new(self, 22)
    }
}
#[doc = "Software interrupt event register (EXTI_SWIER)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWIERrs;
impl crate::RegisterSpec for SWIERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swier::R`](R) reader structure"]
impl crate::Readable for SWIERrs {}
#[doc = "`write(|w| ..)` method takes [`swier::W`](W) writer structure"]
impl crate::Writable for SWIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWIER to value 0"]
impl crate::Resettable for SWIERrs {
    const RESET_VALUE: u32 = 0;
}
