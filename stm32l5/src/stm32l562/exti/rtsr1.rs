#[doc = "Register `RTSR1` reader"]
pub type R = crate::R<RTSR1rs>;
#[doc = "Register `RTSR1` writer"]
pub type W = crate::W<RTSR1rs>;
#[doc = "Field `RT0` reader - Rising trigger event configuration bit of configurable event input x"]
pub type RT0_R = crate::BitReader;
#[doc = "Field `RT0` writer - Rising trigger event configuration bit of configurable event input x"]
pub type RT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT1` reader - Rising trigger event configuration bit of configurable event input x"]
pub type RT1_R = crate::BitReader;
#[doc = "Field `RT1` writer - Rising trigger event configuration bit of configurable event input x"]
pub type RT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT2` reader - Rising trigger event configuration bit of configurable event input x"]
pub type RT2_R = crate::BitReader;
#[doc = "Field `RT2` writer - Rising trigger event configuration bit of configurable event input x"]
pub type RT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT3` reader - Rising trigger event configuration bit of configurable event input x"]
pub type RT3_R = crate::BitReader;
#[doc = "Field `RT3` writer - Rising trigger event configuration bit of configurable event input x"]
pub type RT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT4` reader - Rising trigger event configuration bit of configurable event input x"]
pub type RT4_R = crate::BitReader;
#[doc = "Field `RT4` writer - Rising trigger event configuration bit of configurable event input x"]
pub type RT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT5` reader - Rising trigger event configuration bit of configurable event input x"]
pub type RT5_R = crate::BitReader;
#[doc = "Field `RT5` writer - Rising trigger event configuration bit of configurable event input x"]
pub type RT5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT6` reader - Rising trigger event configuration bit of configurable event input x"]
pub type RT6_R = crate::BitReader;
#[doc = "Field `RT6` writer - Rising trigger event configuration bit of configurable event input x"]
pub type RT6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT7` reader - Rising trigger event configuration bit of configurable event input x"]
pub type RT7_R = crate::BitReader;
#[doc = "Field `RT7` writer - Rising trigger event configuration bit of configurable event input x"]
pub type RT7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT8` reader - Rising trigger event configuration bit of configurable event input x"]
pub type RT8_R = crate::BitReader;
#[doc = "Field `RT8` writer - Rising trigger event configuration bit of configurable event input x"]
pub type RT8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT9` reader - Rising trigger event configuration bit of configurable event input x"]
pub type RT9_R = crate::BitReader;
#[doc = "Field `RT9` writer - Rising trigger event configuration bit of configurable event input x"]
pub type RT9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT10` reader - Rising trigger event configuration bit of configurable event input x"]
pub type RT10_R = crate::BitReader;
#[doc = "Field `RT10` writer - Rising trigger event configuration bit of configurable event input x"]
pub type RT10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT11` reader - Rising trigger event configuration bit of configurable event input x"]
pub type RT11_R = crate::BitReader;
#[doc = "Field `RT11` writer - Rising trigger event configuration bit of configurable event input x"]
pub type RT11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT12` reader - Rising trigger event configuration bit of configurable event input x"]
pub type RT12_R = crate::BitReader;
#[doc = "Field `RT12` writer - Rising trigger event configuration bit of configurable event input x"]
pub type RT12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT13` reader - Rising trigger event configuration bit of configurable event input x"]
pub type RT13_R = crate::BitReader;
#[doc = "Field `RT13` writer - Rising trigger event configuration bit of configurable event input x"]
pub type RT13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT14` reader - Rising trigger event configuration bit of configurable event input x"]
pub type RT14_R = crate::BitReader;
#[doc = "Field `RT14` writer - Rising trigger event configuration bit of configurable event input x"]
pub type RT14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT15` reader - Rising trigger event configuration bit of configurable event input x"]
pub type RT15_R = crate::BitReader;
#[doc = "Field `RT15` writer - Rising trigger event configuration bit of configurable event input x"]
pub type RT15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT16` reader - Rising trigger event configuration bit of configurable event input x"]
pub type RT16_R = crate::BitReader;
#[doc = "Field `RT16` writer - Rising trigger event configuration bit of configurable event input x"]
pub type RT16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT21` reader - Rising trigger event configuration bit of configurable event input x"]
pub type RT21_R = crate::BitReader;
#[doc = "Field `RT21` writer - Rising trigger event configuration bit of configurable event input x"]
pub type RT21_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT22` reader - Rising trigger event configuration bit of configurable event input x"]
pub type RT22_R = crate::BitReader;
#[doc = "Field `RT22` writer - Rising trigger event configuration bit of configurable event input x"]
pub type RT22_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt0(&self) -> RT0_R {
        RT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt1(&self) -> RT1_R {
        RT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt2(&self) -> RT2_R {
        RT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt3(&self) -> RT3_R {
        RT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt4(&self) -> RT4_R {
        RT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt5(&self) -> RT5_R {
        RT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt6(&self) -> RT6_R {
        RT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt7(&self) -> RT7_R {
        RT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt8(&self) -> RT8_R {
        RT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt9(&self) -> RT9_R {
        RT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt10(&self) -> RT10_R {
        RT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt11(&self) -> RT11_R {
        RT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt12(&self) -> RT12_R {
        RT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt13(&self) -> RT13_R {
        RT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt14(&self) -> RT14_R {
        RT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt15(&self) -> RT15_R {
        RT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt16(&self) -> RT16_R {
        RT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt21(&self) -> RT21_R {
        RT21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    pub fn rt22(&self) -> RT22_R {
        RT22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    #[must_use]
    pub fn rt0(&mut self) -> RT0_W<RTSR1rs> {
        RT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    #[must_use]
    pub fn rt1(&mut self) -> RT1_W<RTSR1rs> {
        RT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    #[must_use]
    pub fn rt2(&mut self) -> RT2_W<RTSR1rs> {
        RT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    #[must_use]
    pub fn rt3(&mut self) -> RT3_W<RTSR1rs> {
        RT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    #[must_use]
    pub fn rt4(&mut self) -> RT4_W<RTSR1rs> {
        RT4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    #[must_use]
    pub fn rt5(&mut self) -> RT5_W<RTSR1rs> {
        RT5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    #[must_use]
    pub fn rt6(&mut self) -> RT6_W<RTSR1rs> {
        RT6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    #[must_use]
    pub fn rt7(&mut self) -> RT7_W<RTSR1rs> {
        RT7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    #[must_use]
    pub fn rt8(&mut self) -> RT8_W<RTSR1rs> {
        RT8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    #[must_use]
    pub fn rt9(&mut self) -> RT9_W<RTSR1rs> {
        RT9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    #[must_use]
    pub fn rt10(&mut self) -> RT10_W<RTSR1rs> {
        RT10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    #[must_use]
    pub fn rt11(&mut self) -> RT11_W<RTSR1rs> {
        RT11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    #[must_use]
    pub fn rt12(&mut self) -> RT12_W<RTSR1rs> {
        RT12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    #[must_use]
    pub fn rt13(&mut self) -> RT13_W<RTSR1rs> {
        RT13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    #[must_use]
    pub fn rt14(&mut self) -> RT14_W<RTSR1rs> {
        RT14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    #[must_use]
    pub fn rt15(&mut self) -> RT15_W<RTSR1rs> {
        RT15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    #[must_use]
    pub fn rt16(&mut self) -> RT16_W<RTSR1rs> {
        RT16_W::new(self, 16)
    }
    #[doc = "Bit 21 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    #[must_use]
    pub fn rt21(&mut self) -> RT21_W<RTSR1rs> {
        RT21_W::new(self, 21)
    }
    #[doc = "Bit 22 - Rising trigger event configuration bit of configurable event input x"]
    #[inline(always)]
    #[must_use]
    pub fn rt22(&mut self) -> RT22_W<RTSR1rs> {
        RT22_W::new(self, 22)
    }
}
#[doc = "EXTI rising trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTSR1rs;
impl crate::RegisterSpec for RTSR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtsr1::R`](R) reader structure"]
impl crate::Readable for RTSR1rs {}
#[doc = "`write(|w| ..)` method takes [`rtsr1::W`](W) writer structure"]
impl crate::Writable for RTSR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTSR1 to value 0"]
impl crate::Resettable for RTSR1rs {
    const RESET_VALUE: u32 = 0;
}
