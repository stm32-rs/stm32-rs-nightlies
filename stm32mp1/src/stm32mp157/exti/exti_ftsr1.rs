#[doc = "Register `EXTI_FTSR1` reader"]
pub type R = crate::R<EXTI_FTSR1rs>;
#[doc = "Register `EXTI_FTSR1` writer"]
pub type W = crate::W<EXTI_FTSR1rs>;
#[doc = "Field `FT0` reader - FT0"]
pub type FT0_R = crate::BitReader;
#[doc = "Field `FT0` writer - FT0"]
pub type FT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT1` reader - FT1"]
pub type FT1_R = crate::BitReader;
#[doc = "Field `FT1` writer - FT1"]
pub type FT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT2` reader - FT2"]
pub type FT2_R = crate::BitReader;
#[doc = "Field `FT2` writer - FT2"]
pub type FT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT3` reader - FT3"]
pub type FT3_R = crate::BitReader;
#[doc = "Field `FT3` writer - FT3"]
pub type FT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT4` reader - FT4"]
pub type FT4_R = crate::BitReader;
#[doc = "Field `FT4` writer - FT4"]
pub type FT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT5` reader - FT5"]
pub type FT5_R = crate::BitReader;
#[doc = "Field `FT5` writer - FT5"]
pub type FT5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT6` reader - FT6"]
pub type FT6_R = crate::BitReader;
#[doc = "Field `FT6` writer - FT6"]
pub type FT6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT7` reader - FT7"]
pub type FT7_R = crate::BitReader;
#[doc = "Field `FT7` writer - FT7"]
pub type FT7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT8` reader - FT8"]
pub type FT8_R = crate::BitReader;
#[doc = "Field `FT8` writer - FT8"]
pub type FT8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT9` reader - FT9"]
pub type FT9_R = crate::BitReader;
#[doc = "Field `FT9` writer - FT9"]
pub type FT9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT10` reader - FT10"]
pub type FT10_R = crate::BitReader;
#[doc = "Field `FT10` writer - FT10"]
pub type FT10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT11` reader - FT11"]
pub type FT11_R = crate::BitReader;
#[doc = "Field `FT11` writer - FT11"]
pub type FT11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT12` reader - FT12"]
pub type FT12_R = crate::BitReader;
#[doc = "Field `FT12` writer - FT12"]
pub type FT12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT13` reader - FT13"]
pub type FT13_R = crate::BitReader;
#[doc = "Field `FT13` writer - FT13"]
pub type FT13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT14` reader - FT14"]
pub type FT14_R = crate::BitReader;
#[doc = "Field `FT14` writer - FT14"]
pub type FT14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT15` reader - FT15"]
pub type FT15_R = crate::BitReader;
#[doc = "Field `FT15` writer - FT15"]
pub type FT15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT16` reader - FT16"]
pub type FT16_R = crate::BitReader;
#[doc = "Field `FT16` writer - FT16"]
pub type FT16_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FT0"]
    #[inline(always)]
    pub fn ft0(&self) -> FT0_R {
        FT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FT1"]
    #[inline(always)]
    pub fn ft1(&self) -> FT1_R {
        FT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FT2"]
    #[inline(always)]
    pub fn ft2(&self) -> FT2_R {
        FT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FT3"]
    #[inline(always)]
    pub fn ft3(&self) -> FT3_R {
        FT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FT4"]
    #[inline(always)]
    pub fn ft4(&self) -> FT4_R {
        FT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FT5"]
    #[inline(always)]
    pub fn ft5(&self) -> FT5_R {
        FT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FT6"]
    #[inline(always)]
    pub fn ft6(&self) -> FT6_R {
        FT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FT7"]
    #[inline(always)]
    pub fn ft7(&self) -> FT7_R {
        FT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FT8"]
    #[inline(always)]
    pub fn ft8(&self) -> FT8_R {
        FT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FT9"]
    #[inline(always)]
    pub fn ft9(&self) -> FT9_R {
        FT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FT10"]
    #[inline(always)]
    pub fn ft10(&self) -> FT10_R {
        FT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FT11"]
    #[inline(always)]
    pub fn ft11(&self) -> FT11_R {
        FT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FT12"]
    #[inline(always)]
    pub fn ft12(&self) -> FT12_R {
        FT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FT13"]
    #[inline(always)]
    pub fn ft13(&self) -> FT13_R {
        FT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - FT14"]
    #[inline(always)]
    pub fn ft14(&self) -> FT14_R {
        FT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - FT15"]
    #[inline(always)]
    pub fn ft15(&self) -> FT15_R {
        FT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - FT16"]
    #[inline(always)]
    pub fn ft16(&self) -> FT16_R {
        FT16_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FT0"]
    #[inline(always)]
    #[must_use]
    pub fn ft0(&mut self) -> FT0_W<EXTI_FTSR1rs> {
        FT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - FT1"]
    #[inline(always)]
    #[must_use]
    pub fn ft1(&mut self) -> FT1_W<EXTI_FTSR1rs> {
        FT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - FT2"]
    #[inline(always)]
    #[must_use]
    pub fn ft2(&mut self) -> FT2_W<EXTI_FTSR1rs> {
        FT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - FT3"]
    #[inline(always)]
    #[must_use]
    pub fn ft3(&mut self) -> FT3_W<EXTI_FTSR1rs> {
        FT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - FT4"]
    #[inline(always)]
    #[must_use]
    pub fn ft4(&mut self) -> FT4_W<EXTI_FTSR1rs> {
        FT4_W::new(self, 4)
    }
    #[doc = "Bit 5 - FT5"]
    #[inline(always)]
    #[must_use]
    pub fn ft5(&mut self) -> FT5_W<EXTI_FTSR1rs> {
        FT5_W::new(self, 5)
    }
    #[doc = "Bit 6 - FT6"]
    #[inline(always)]
    #[must_use]
    pub fn ft6(&mut self) -> FT6_W<EXTI_FTSR1rs> {
        FT6_W::new(self, 6)
    }
    #[doc = "Bit 7 - FT7"]
    #[inline(always)]
    #[must_use]
    pub fn ft7(&mut self) -> FT7_W<EXTI_FTSR1rs> {
        FT7_W::new(self, 7)
    }
    #[doc = "Bit 8 - FT8"]
    #[inline(always)]
    #[must_use]
    pub fn ft8(&mut self) -> FT8_W<EXTI_FTSR1rs> {
        FT8_W::new(self, 8)
    }
    #[doc = "Bit 9 - FT9"]
    #[inline(always)]
    #[must_use]
    pub fn ft9(&mut self) -> FT9_W<EXTI_FTSR1rs> {
        FT9_W::new(self, 9)
    }
    #[doc = "Bit 10 - FT10"]
    #[inline(always)]
    #[must_use]
    pub fn ft10(&mut self) -> FT10_W<EXTI_FTSR1rs> {
        FT10_W::new(self, 10)
    }
    #[doc = "Bit 11 - FT11"]
    #[inline(always)]
    #[must_use]
    pub fn ft11(&mut self) -> FT11_W<EXTI_FTSR1rs> {
        FT11_W::new(self, 11)
    }
    #[doc = "Bit 12 - FT12"]
    #[inline(always)]
    #[must_use]
    pub fn ft12(&mut self) -> FT12_W<EXTI_FTSR1rs> {
        FT12_W::new(self, 12)
    }
    #[doc = "Bit 13 - FT13"]
    #[inline(always)]
    #[must_use]
    pub fn ft13(&mut self) -> FT13_W<EXTI_FTSR1rs> {
        FT13_W::new(self, 13)
    }
    #[doc = "Bit 14 - FT14"]
    #[inline(always)]
    #[must_use]
    pub fn ft14(&mut self) -> FT14_W<EXTI_FTSR1rs> {
        FT14_W::new(self, 14)
    }
    #[doc = "Bit 15 - FT15"]
    #[inline(always)]
    #[must_use]
    pub fn ft15(&mut self) -> FT15_W<EXTI_FTSR1rs> {
        FT15_W::new(self, 15)
    }
    #[doc = "Bit 16 - FT16"]
    #[inline(always)]
    #[must_use]
    pub fn ft16(&mut self) -> FT16_W<EXTI_FTSR1rs> {
        FT16_W::new(self, 16)
    }
}
#[doc = "Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_ftsr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_ftsr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_FTSR1rs;
impl crate::RegisterSpec for EXTI_FTSR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_ftsr1::R`](R) reader structure"]
impl crate::Readable for EXTI_FTSR1rs {}
#[doc = "`write(|w| ..)` method takes [`exti_ftsr1::W`](W) writer structure"]
impl crate::Writable for EXTI_FTSR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_FTSR1 to value 0"]
impl crate::Resettable for EXTI_FTSR1rs {
    const RESET_VALUE: u32 = 0;
}
