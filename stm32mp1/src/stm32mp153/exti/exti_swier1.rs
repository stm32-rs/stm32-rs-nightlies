#[doc = "Register `EXTI_SWIER1` reader"]
pub type R = crate::R<EXTI_SWIER1rs>;
#[doc = "Register `EXTI_SWIER1` writer"]
pub type W = crate::W<EXTI_SWIER1rs>;
#[doc = "Field `SWI0` reader - SWI0"]
pub type SWI0_R = crate::BitReader;
#[doc = "Field `SWI0` writer - SWI0"]
pub type SWI0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI1` reader - SWI1"]
pub type SWI1_R = crate::BitReader;
#[doc = "Field `SWI1` writer - SWI1"]
pub type SWI1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI2` reader - SWI2"]
pub type SWI2_R = crate::BitReader;
#[doc = "Field `SWI2` writer - SWI2"]
pub type SWI2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI3` reader - SWI3"]
pub type SWI3_R = crate::BitReader;
#[doc = "Field `SWI3` writer - SWI3"]
pub type SWI3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI4` reader - SWI4"]
pub type SWI4_R = crate::BitReader;
#[doc = "Field `SWI4` writer - SWI4"]
pub type SWI4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI5` reader - SWI5"]
pub type SWI5_R = crate::BitReader;
#[doc = "Field `SWI5` writer - SWI5"]
pub type SWI5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI6` reader - SWI6"]
pub type SWI6_R = crate::BitReader;
#[doc = "Field `SWI6` writer - SWI6"]
pub type SWI6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI7` reader - SWI7"]
pub type SWI7_R = crate::BitReader;
#[doc = "Field `SWI7` writer - SWI7"]
pub type SWI7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI8` reader - SWI8"]
pub type SWI8_R = crate::BitReader;
#[doc = "Field `SWI8` writer - SWI8"]
pub type SWI8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI9` reader - SWI9"]
pub type SWI9_R = crate::BitReader;
#[doc = "Field `SWI9` writer - SWI9"]
pub type SWI9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI10` reader - SWI10"]
pub type SWI10_R = crate::BitReader;
#[doc = "Field `SWI10` writer - SWI10"]
pub type SWI10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI11` reader - SWI11"]
pub type SWI11_R = crate::BitReader;
#[doc = "Field `SWI11` writer - SWI11"]
pub type SWI11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI12` reader - SWI12"]
pub type SWI12_R = crate::BitReader;
#[doc = "Field `SWI12` writer - SWI12"]
pub type SWI12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI13` reader - SWI13"]
pub type SWI13_R = crate::BitReader;
#[doc = "Field `SWI13` writer - SWI13"]
pub type SWI13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI14` reader - SWI14"]
pub type SWI14_R = crate::BitReader;
#[doc = "Field `SWI14` writer - SWI14"]
pub type SWI14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI15` reader - SWI15"]
pub type SWI15_R = crate::BitReader;
#[doc = "Field `SWI15` writer - SWI15"]
pub type SWI15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI16` reader - SWI16"]
pub type SWI16_R = crate::BitReader;
#[doc = "Field `SWI16` writer - SWI16"]
pub type SWI16_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SWI0"]
    #[inline(always)]
    pub fn swi0(&self) -> SWI0_R {
        SWI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SWI1"]
    #[inline(always)]
    pub fn swi1(&self) -> SWI1_R {
        SWI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SWI2"]
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SWI3"]
    #[inline(always)]
    pub fn swi3(&self) -> SWI3_R {
        SWI3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SWI4"]
    #[inline(always)]
    pub fn swi4(&self) -> SWI4_R {
        SWI4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SWI5"]
    #[inline(always)]
    pub fn swi5(&self) -> SWI5_R {
        SWI5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SWI6"]
    #[inline(always)]
    pub fn swi6(&self) -> SWI6_R {
        SWI6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SWI7"]
    #[inline(always)]
    pub fn swi7(&self) -> SWI7_R {
        SWI7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SWI8"]
    #[inline(always)]
    pub fn swi8(&self) -> SWI8_R {
        SWI8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SWI9"]
    #[inline(always)]
    pub fn swi9(&self) -> SWI9_R {
        SWI9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SWI10"]
    #[inline(always)]
    pub fn swi10(&self) -> SWI10_R {
        SWI10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SWI11"]
    #[inline(always)]
    pub fn swi11(&self) -> SWI11_R {
        SWI11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SWI12"]
    #[inline(always)]
    pub fn swi12(&self) -> SWI12_R {
        SWI12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SWI13"]
    #[inline(always)]
    pub fn swi13(&self) -> SWI13_R {
        SWI13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SWI14"]
    #[inline(always)]
    pub fn swi14(&self) -> SWI14_R {
        SWI14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SWI15"]
    #[inline(always)]
    pub fn swi15(&self) -> SWI15_R {
        SWI15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SWI16"]
    #[inline(always)]
    pub fn swi16(&self) -> SWI16_R {
        SWI16_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SWI0"]
    #[inline(always)]
    #[must_use]
    pub fn swi0(&mut self) -> SWI0_W<EXTI_SWIER1rs> {
        SWI0_W::new(self, 0)
    }
    #[doc = "Bit 1 - SWI1"]
    #[inline(always)]
    #[must_use]
    pub fn swi1(&mut self) -> SWI1_W<EXTI_SWIER1rs> {
        SWI1_W::new(self, 1)
    }
    #[doc = "Bit 2 - SWI2"]
    #[inline(always)]
    #[must_use]
    pub fn swi2(&mut self) -> SWI2_W<EXTI_SWIER1rs> {
        SWI2_W::new(self, 2)
    }
    #[doc = "Bit 3 - SWI3"]
    #[inline(always)]
    #[must_use]
    pub fn swi3(&mut self) -> SWI3_W<EXTI_SWIER1rs> {
        SWI3_W::new(self, 3)
    }
    #[doc = "Bit 4 - SWI4"]
    #[inline(always)]
    #[must_use]
    pub fn swi4(&mut self) -> SWI4_W<EXTI_SWIER1rs> {
        SWI4_W::new(self, 4)
    }
    #[doc = "Bit 5 - SWI5"]
    #[inline(always)]
    #[must_use]
    pub fn swi5(&mut self) -> SWI5_W<EXTI_SWIER1rs> {
        SWI5_W::new(self, 5)
    }
    #[doc = "Bit 6 - SWI6"]
    #[inline(always)]
    #[must_use]
    pub fn swi6(&mut self) -> SWI6_W<EXTI_SWIER1rs> {
        SWI6_W::new(self, 6)
    }
    #[doc = "Bit 7 - SWI7"]
    #[inline(always)]
    #[must_use]
    pub fn swi7(&mut self) -> SWI7_W<EXTI_SWIER1rs> {
        SWI7_W::new(self, 7)
    }
    #[doc = "Bit 8 - SWI8"]
    #[inline(always)]
    #[must_use]
    pub fn swi8(&mut self) -> SWI8_W<EXTI_SWIER1rs> {
        SWI8_W::new(self, 8)
    }
    #[doc = "Bit 9 - SWI9"]
    #[inline(always)]
    #[must_use]
    pub fn swi9(&mut self) -> SWI9_W<EXTI_SWIER1rs> {
        SWI9_W::new(self, 9)
    }
    #[doc = "Bit 10 - SWI10"]
    #[inline(always)]
    #[must_use]
    pub fn swi10(&mut self) -> SWI10_W<EXTI_SWIER1rs> {
        SWI10_W::new(self, 10)
    }
    #[doc = "Bit 11 - SWI11"]
    #[inline(always)]
    #[must_use]
    pub fn swi11(&mut self) -> SWI11_W<EXTI_SWIER1rs> {
        SWI11_W::new(self, 11)
    }
    #[doc = "Bit 12 - SWI12"]
    #[inline(always)]
    #[must_use]
    pub fn swi12(&mut self) -> SWI12_W<EXTI_SWIER1rs> {
        SWI12_W::new(self, 12)
    }
    #[doc = "Bit 13 - SWI13"]
    #[inline(always)]
    #[must_use]
    pub fn swi13(&mut self) -> SWI13_W<EXTI_SWIER1rs> {
        SWI13_W::new(self, 13)
    }
    #[doc = "Bit 14 - SWI14"]
    #[inline(always)]
    #[must_use]
    pub fn swi14(&mut self) -> SWI14_W<EXTI_SWIER1rs> {
        SWI14_W::new(self, 14)
    }
    #[doc = "Bit 15 - SWI15"]
    #[inline(always)]
    #[must_use]
    pub fn swi15(&mut self) -> SWI15_W<EXTI_SWIER1rs> {
        SWI15_W::new(self, 15)
    }
    #[doc = "Bit 16 - SWI16"]
    #[inline(always)]
    #[must_use]
    pub fn swi16(&mut self) -> SWI16_W<EXTI_SWIER1rs> {
        SWI16_W::new(self, 16)
    }
}
#[doc = "Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_swier1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_swier1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_SWIER1rs;
impl crate::RegisterSpec for EXTI_SWIER1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_swier1::R`](R) reader structure"]
impl crate::Readable for EXTI_SWIER1rs {}
#[doc = "`write(|w| ..)` method takes [`exti_swier1::W`](W) writer structure"]
impl crate::Writable for EXTI_SWIER1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_SWIER1 to value 0"]
impl crate::Resettable for EXTI_SWIER1rs {
    const RESET_VALUE: u32 = 0;
}
