#[doc = "Register `PWR_PUCRJ` reader"]
pub type R = crate::R<PWR_PUCRJrs>;
#[doc = "Register `PWR_PUCRJ` writer"]
pub type W = crate::W<PWR_PUCRJrs>;
#[doc = "Field `PU0` reader - "]
pub type PU0_R = crate::BitReader;
#[doc = "Field `PU0` writer - "]
pub type PU0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU1` reader - "]
pub type PU1_R = crate::BitReader;
#[doc = "Field `PU1` writer - "]
pub type PU1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU2` reader - "]
pub type PU2_R = crate::BitReader;
#[doc = "Field `PU2` writer - "]
pub type PU2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU3` reader - "]
pub type PU3_R = crate::BitReader;
#[doc = "Field `PU3` writer - "]
pub type PU3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU4` reader - "]
pub type PU4_R = crate::BitReader;
#[doc = "Field `PU4` writer - "]
pub type PU4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU5` reader - "]
pub type PU5_R = crate::BitReader;
#[doc = "Field `PU5` writer - "]
pub type PU5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU6` reader - "]
pub type PU6_R = crate::BitReader;
#[doc = "Field `PU6` writer - "]
pub type PU6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU7` reader - "]
pub type PU7_R = crate::BitReader;
#[doc = "Field `PU7` writer - "]
pub type PU7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU8` reader - "]
pub type PU8_R = crate::BitReader;
#[doc = "Field `PU8` writer - "]
pub type PU8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU9` reader - "]
pub type PU9_R = crate::BitReader;
#[doc = "Field `PU9` writer - "]
pub type PU9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU10` reader - "]
pub type PU10_R = crate::BitReader;
#[doc = "Field `PU10` writer - "]
pub type PU10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU11` reader - "]
pub type PU11_R = crate::BitReader;
#[doc = "Field `PU11` writer - "]
pub type PU11_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu4(&self) -> PU4_R {
        PU4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu5(&self) -> PU5_R {
        PU5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pu7(&self) -> PU7_R {
        PU7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu8(&self) -> PU8_R {
        PU8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu9(&self) -> PU9_R {
        PU9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu10(&self) -> PU10_R {
        PU10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu11(&self) -> PU11_R {
        PU11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pu0(&mut self) -> PU0_W<PWR_PUCRJrs> {
        PU0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pu1(&mut self) -> PU1_W<PWR_PUCRJrs> {
        PU1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pu2(&mut self) -> PU2_W<PWR_PUCRJrs> {
        PU2_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pu3(&mut self) -> PU3_W<PWR_PUCRJrs> {
        PU3_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pu4(&mut self) -> PU4_W<PWR_PUCRJrs> {
        PU4_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pu5(&mut self) -> PU5_W<PWR_PUCRJrs> {
        PU5_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pu6(&mut self) -> PU6_W<PWR_PUCRJrs> {
        PU6_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pu7(&mut self) -> PU7_W<PWR_PUCRJrs> {
        PU7_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pu8(&mut self) -> PU8_W<PWR_PUCRJrs> {
        PU8_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pu9(&mut self) -> PU9_W<PWR_PUCRJrs> {
        PU9_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pu10(&mut self) -> PU10_W<PWR_PUCRJrs> {
        PU10_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pu11(&mut self) -> PU11_W<PWR_PUCRJrs> {
        PU11_W::new(self, 11)
    }
}
#[doc = "PWR port J pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pucrj::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pucrj::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_PUCRJrs;
impl crate::RegisterSpec for PWR_PUCRJrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_pucrj::R`](R) reader structure"]
impl crate::Readable for PWR_PUCRJrs {}
#[doc = "`write(|w| ..)` method takes [`pwr_pucrj::W`](W) writer structure"]
impl crate::Writable for PWR_PUCRJrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_PUCRJ to value 0"]
impl crate::Resettable for PWR_PUCRJrs {
    const RESET_VALUE: u32 = 0;
}
