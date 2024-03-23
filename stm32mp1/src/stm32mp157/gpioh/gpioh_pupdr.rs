#[doc = "Register `GPIOH_PUPDR` reader"]
pub type R = crate::R<GPIOH_PUPDRrs>;
#[doc = "Register `GPIOH_PUPDR` writer"]
pub type W = crate::W<GPIOH_PUPDRrs>;
#[doc = "Field `PUPDR0` reader - PUPDR0"]
pub type PUPDR0_R = crate::FieldReader;
#[doc = "Field `PUPDR0` writer - PUPDR0"]
pub type PUPDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR1` reader - PUPDR1"]
pub type PUPDR1_R = crate::FieldReader;
#[doc = "Field `PUPDR1` writer - PUPDR1"]
pub type PUPDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR2` reader - PUPDR2"]
pub type PUPDR2_R = crate::FieldReader;
#[doc = "Field `PUPDR2` writer - PUPDR2"]
pub type PUPDR2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR3` reader - PUPDR3"]
pub type PUPDR3_R = crate::FieldReader;
#[doc = "Field `PUPDR3` writer - PUPDR3"]
pub type PUPDR3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR4` reader - PUPDR4"]
pub type PUPDR4_R = crate::FieldReader;
#[doc = "Field `PUPDR4` writer - PUPDR4"]
pub type PUPDR4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR5` reader - PUPDR5"]
pub type PUPDR5_R = crate::FieldReader;
#[doc = "Field `PUPDR5` writer - PUPDR5"]
pub type PUPDR5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR6` reader - PUPDR6"]
pub type PUPDR6_R = crate::FieldReader;
#[doc = "Field `PUPDR6` writer - PUPDR6"]
pub type PUPDR6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR7` reader - PUPDR7"]
pub type PUPDR7_R = crate::FieldReader;
#[doc = "Field `PUPDR7` writer - PUPDR7"]
pub type PUPDR7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR8` reader - PUPDR8"]
pub type PUPDR8_R = crate::FieldReader;
#[doc = "Field `PUPDR8` writer - PUPDR8"]
pub type PUPDR8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR9` reader - PUPDR9"]
pub type PUPDR9_R = crate::FieldReader;
#[doc = "Field `PUPDR9` writer - PUPDR9"]
pub type PUPDR9_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR10` reader - PUPDR10"]
pub type PUPDR10_R = crate::FieldReader;
#[doc = "Field `PUPDR10` writer - PUPDR10"]
pub type PUPDR10_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR11` reader - PUPDR11"]
pub type PUPDR11_R = crate::FieldReader;
#[doc = "Field `PUPDR11` writer - PUPDR11"]
pub type PUPDR11_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR12` reader - PUPDR12"]
pub type PUPDR12_R = crate::FieldReader;
#[doc = "Field `PUPDR12` writer - PUPDR12"]
pub type PUPDR12_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR13` reader - PUPDR13"]
pub type PUPDR13_R = crate::FieldReader;
#[doc = "Field `PUPDR13` writer - PUPDR13"]
pub type PUPDR13_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR14` reader - PUPDR14"]
pub type PUPDR14_R = crate::FieldReader;
#[doc = "Field `PUPDR14` writer - PUPDR14"]
pub type PUPDR14_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR15` reader - PUPDR15"]
pub type PUPDR15_R = crate::FieldReader;
#[doc = "Field `PUPDR15` writer - PUPDR15"]
pub type PUPDR15_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - PUPDR0"]
    #[inline(always)]
    pub fn pupdr0(&self) -> PUPDR0_R {
        PUPDR0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PUPDR1"]
    #[inline(always)]
    pub fn pupdr1(&self) -> PUPDR1_R {
        PUPDR1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PUPDR2"]
    #[inline(always)]
    pub fn pupdr2(&self) -> PUPDR2_R {
        PUPDR2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PUPDR3"]
    #[inline(always)]
    pub fn pupdr3(&self) -> PUPDR3_R {
        PUPDR3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PUPDR4"]
    #[inline(always)]
    pub fn pupdr4(&self) -> PUPDR4_R {
        PUPDR4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PUPDR5"]
    #[inline(always)]
    pub fn pupdr5(&self) -> PUPDR5_R {
        PUPDR5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PUPDR6"]
    #[inline(always)]
    pub fn pupdr6(&self) -> PUPDR6_R {
        PUPDR6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PUPDR7"]
    #[inline(always)]
    pub fn pupdr7(&self) -> PUPDR7_R {
        PUPDR7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PUPDR8"]
    #[inline(always)]
    pub fn pupdr8(&self) -> PUPDR8_R {
        PUPDR8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PUPDR9"]
    #[inline(always)]
    pub fn pupdr9(&self) -> PUPDR9_R {
        PUPDR9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PUPDR10"]
    #[inline(always)]
    pub fn pupdr10(&self) -> PUPDR10_R {
        PUPDR10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PUPDR11"]
    #[inline(always)]
    pub fn pupdr11(&self) -> PUPDR11_R {
        PUPDR11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PUPDR12"]
    #[inline(always)]
    pub fn pupdr12(&self) -> PUPDR12_R {
        PUPDR12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PUPDR13"]
    #[inline(always)]
    pub fn pupdr13(&self) -> PUPDR13_R {
        PUPDR13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PUPDR14"]
    #[inline(always)]
    pub fn pupdr14(&self) -> PUPDR14_R {
        PUPDR14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PUPDR15"]
    #[inline(always)]
    pub fn pupdr15(&self) -> PUPDR15_R {
        PUPDR15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PUPDR0"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr0(&mut self) -> PUPDR0_W<GPIOH_PUPDRrs> {
        PUPDR0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - PUPDR1"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr1(&mut self) -> PUPDR1_W<GPIOH_PUPDRrs> {
        PUPDR1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - PUPDR2"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr2(&mut self) -> PUPDR2_W<GPIOH_PUPDRrs> {
        PUPDR2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - PUPDR3"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr3(&mut self) -> PUPDR3_W<GPIOH_PUPDRrs> {
        PUPDR3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - PUPDR4"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr4(&mut self) -> PUPDR4_W<GPIOH_PUPDRrs> {
        PUPDR4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - PUPDR5"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr5(&mut self) -> PUPDR5_W<GPIOH_PUPDRrs> {
        PUPDR5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - PUPDR6"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr6(&mut self) -> PUPDR6_W<GPIOH_PUPDRrs> {
        PUPDR6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - PUPDR7"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr7(&mut self) -> PUPDR7_W<GPIOH_PUPDRrs> {
        PUPDR7_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - PUPDR8"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr8(&mut self) -> PUPDR8_W<GPIOH_PUPDRrs> {
        PUPDR8_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - PUPDR9"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr9(&mut self) -> PUPDR9_W<GPIOH_PUPDRrs> {
        PUPDR9_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - PUPDR10"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr10(&mut self) -> PUPDR10_W<GPIOH_PUPDRrs> {
        PUPDR10_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - PUPDR11"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr11(&mut self) -> PUPDR11_W<GPIOH_PUPDRrs> {
        PUPDR11_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - PUPDR12"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr12(&mut self) -> PUPDR12_W<GPIOH_PUPDRrs> {
        PUPDR12_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - PUPDR13"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr13(&mut self) -> PUPDR13_W<GPIOH_PUPDRrs> {
        PUPDR13_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - PUPDR14"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr14(&mut self) -> PUPDR14_W<GPIOH_PUPDRrs> {
        PUPDR14_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - PUPDR15"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr15(&mut self) -> PUPDR15_W<GPIOH_PUPDRrs> {
        PUPDR15_W::new(self, 30)
    }
}
#[doc = "GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioh_pupdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioh_pupdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOH_PUPDRrs;
impl crate::RegisterSpec for GPIOH_PUPDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioh_pupdr::R`](R) reader structure"]
impl crate::Readable for GPIOH_PUPDRrs {}
#[doc = "`write(|w| ..)` method takes [`gpioh_pupdr::W`](W) writer structure"]
impl crate::Writable for GPIOH_PUPDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIOH_PUPDR to value 0"]
impl crate::Resettable for GPIOH_PUPDRrs {
    const RESET_VALUE: u32 = 0;
}
