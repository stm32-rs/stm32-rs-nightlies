#[doc = "Register `PUPDR` reader"]
pub type R = crate::R<PUPDRrs>;
#[doc = "Register `PUPDR` writer"]
pub type W = crate::W<PUPDRrs>;
#[doc = "Field `PUPDR0` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR0_R = crate::FieldReader;
#[doc = "Field `PUPDR0` writer - Port x configuration bits (y = 0..15)"]
pub type PUPDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR1` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR1_R = crate::FieldReader;
#[doc = "Field `PUPDR1` writer - Port x configuration bits (y = 0..15)"]
pub type PUPDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR2` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR2_R = crate::FieldReader;
#[doc = "Field `PUPDR2` writer - Port x configuration bits (y = 0..15)"]
pub type PUPDR2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR3` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR3_R = crate::FieldReader;
#[doc = "Field `PUPDR3` writer - Port x configuration bits (y = 0..15)"]
pub type PUPDR3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR4` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR4_R = crate::FieldReader;
#[doc = "Field `PUPDR4` writer - Port x configuration bits (y = 0..15)"]
pub type PUPDR4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR5` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR5_R = crate::FieldReader;
#[doc = "Field `PUPDR5` writer - Port x configuration bits (y = 0..15)"]
pub type PUPDR5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR6` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR6_R = crate::FieldReader;
#[doc = "Field `PUPDR6` writer - Port x configuration bits (y = 0..15)"]
pub type PUPDR6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR7` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR7_R = crate::FieldReader;
#[doc = "Field `PUPDR7` writer - Port x configuration bits (y = 0..15)"]
pub type PUPDR7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR8` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR8_R = crate::FieldReader;
#[doc = "Field `PUPDR8` writer - Port x configuration bits (y = 0..15)"]
pub type PUPDR8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR9` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR9_R = crate::FieldReader;
#[doc = "Field `PUPDR9` writer - Port x configuration bits (y = 0..15)"]
pub type PUPDR9_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR10` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR10_R = crate::FieldReader;
#[doc = "Field `PUPDR10` writer - Port x configuration bits (y = 0..15)"]
pub type PUPDR10_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR11` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR11_R = crate::FieldReader;
#[doc = "Field `PUPDR11` writer - Port x configuration bits (y = 0..15)"]
pub type PUPDR11_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR12` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR12_R = crate::FieldReader;
#[doc = "Field `PUPDR12` writer - Port x configuration bits (y = 0..15)"]
pub type PUPDR12_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR13` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR13_R = crate::FieldReader;
#[doc = "Field `PUPDR13` writer - Port x configuration bits (y = 0..15)"]
pub type PUPDR13_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR14` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR14_R = crate::FieldReader;
#[doc = "Field `PUPDR14` writer - Port x configuration bits (y = 0..15)"]
pub type PUPDR14_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPDR15` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR15_R = crate::FieldReader;
#[doc = "Field `PUPDR15` writer - Port x configuration bits (y = 0..15)"]
pub type PUPDR15_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr0(&self) -> PUPDR0_R {
        PUPDR0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr1(&self) -> PUPDR1_R {
        PUPDR1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr2(&self) -> PUPDR2_R {
        PUPDR2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr3(&self) -> PUPDR3_R {
        PUPDR3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr4(&self) -> PUPDR4_R {
        PUPDR4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr5(&self) -> PUPDR5_R {
        PUPDR5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr6(&self) -> PUPDR6_R {
        PUPDR6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr7(&self) -> PUPDR7_R {
        PUPDR7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr8(&self) -> PUPDR8_R {
        PUPDR8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr9(&self) -> PUPDR9_R {
        PUPDR9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr10(&self) -> PUPDR10_R {
        PUPDR10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr11(&self) -> PUPDR11_R {
        PUPDR11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr12(&self) -> PUPDR12_R {
        PUPDR12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr13(&self) -> PUPDR13_R {
        PUPDR13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr14(&self) -> PUPDR14_R {
        PUPDR14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr15(&self) -> PUPDR15_R {
        PUPDR15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr0(&mut self) -> PUPDR0_W<PUPDRrs> {
        PUPDR0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr1(&mut self) -> PUPDR1_W<PUPDRrs> {
        PUPDR1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr2(&mut self) -> PUPDR2_W<PUPDRrs> {
        PUPDR2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr3(&mut self) -> PUPDR3_W<PUPDRrs> {
        PUPDR3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr4(&mut self) -> PUPDR4_W<PUPDRrs> {
        PUPDR4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr5(&mut self) -> PUPDR5_W<PUPDRrs> {
        PUPDR5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr6(&mut self) -> PUPDR6_W<PUPDRrs> {
        PUPDR6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr7(&mut self) -> PUPDR7_W<PUPDRrs> {
        PUPDR7_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr8(&mut self) -> PUPDR8_W<PUPDRrs> {
        PUPDR8_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr9(&mut self) -> PUPDR9_W<PUPDRrs> {
        PUPDR9_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr10(&mut self) -> PUPDR10_W<PUPDRrs> {
        PUPDR10_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr11(&mut self) -> PUPDR11_W<PUPDRrs> {
        PUPDR11_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr12(&mut self) -> PUPDR12_W<PUPDRrs> {
        PUPDR12_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr13(&mut self) -> PUPDR13_W<PUPDRrs> {
        PUPDR13_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr14(&mut self) -> PUPDR14_W<PUPDRrs> {
        PUPDR14_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr15(&mut self) -> PUPDR15_W<PUPDRrs> {
        PUPDR15_W::new(self, 30)
    }
}
#[doc = "GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pupdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pupdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUPDRrs;
impl crate::RegisterSpec for PUPDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pupdr::R`](R) reader structure"]
impl crate::Readable for PUPDRrs {}
#[doc = "`write(|w| ..)` method takes [`pupdr::W`](W) writer structure"]
impl crate::Writable for PUPDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUPDR to value 0x0100"]
impl crate::Resettable for PUPDRrs {
    const RESET_VALUE: u32 = 0x0100;
}
