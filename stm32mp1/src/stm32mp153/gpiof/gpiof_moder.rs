#[doc = "Register `GPIOF_MODER` reader"]
pub type R = crate::R<GPIOF_MODERrs>;
#[doc = "Register `GPIOF_MODER` writer"]
pub type W = crate::W<GPIOF_MODERrs>;
#[doc = "Field `MODER0` reader - MODER0"]
pub type MODER0_R = crate::FieldReader;
#[doc = "Field `MODER0` writer - MODER0"]
pub type MODER0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER1` reader - MODER1"]
pub type MODER1_R = crate::FieldReader;
#[doc = "Field `MODER1` writer - MODER1"]
pub type MODER1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER2` reader - MODER2"]
pub type MODER2_R = crate::FieldReader;
#[doc = "Field `MODER2` writer - MODER2"]
pub type MODER2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER3` reader - MODER3"]
pub type MODER3_R = crate::FieldReader;
#[doc = "Field `MODER3` writer - MODER3"]
pub type MODER3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER4` reader - MODER4"]
pub type MODER4_R = crate::FieldReader;
#[doc = "Field `MODER4` writer - MODER4"]
pub type MODER4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER5` reader - MODER5"]
pub type MODER5_R = crate::FieldReader;
#[doc = "Field `MODER5` writer - MODER5"]
pub type MODER5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER6` reader - MODER6"]
pub type MODER6_R = crate::FieldReader;
#[doc = "Field `MODER6` writer - MODER6"]
pub type MODER6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER7` reader - MODER7"]
pub type MODER7_R = crate::FieldReader;
#[doc = "Field `MODER7` writer - MODER7"]
pub type MODER7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER8` reader - MODER8"]
pub type MODER8_R = crate::FieldReader;
#[doc = "Field `MODER8` writer - MODER8"]
pub type MODER8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER9` reader - MODER9"]
pub type MODER9_R = crate::FieldReader;
#[doc = "Field `MODER9` writer - MODER9"]
pub type MODER9_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER10` reader - MODER10"]
pub type MODER10_R = crate::FieldReader;
#[doc = "Field `MODER10` writer - MODER10"]
pub type MODER10_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER11` reader - MODER11"]
pub type MODER11_R = crate::FieldReader;
#[doc = "Field `MODER11` writer - MODER11"]
pub type MODER11_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER12` reader - MODER12"]
pub type MODER12_R = crate::FieldReader;
#[doc = "Field `MODER12` writer - MODER12"]
pub type MODER12_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER13` reader - MODER13"]
pub type MODER13_R = crate::FieldReader;
#[doc = "Field `MODER13` writer - MODER13"]
pub type MODER13_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER14` reader - MODER14"]
pub type MODER14_R = crate::FieldReader;
#[doc = "Field `MODER14` writer - MODER14"]
pub type MODER14_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODER15` reader - MODER15"]
pub type MODER15_R = crate::FieldReader;
#[doc = "Field `MODER15` writer - MODER15"]
pub type MODER15_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - MODER0"]
    #[inline(always)]
    pub fn moder0(&self) -> MODER0_R {
        MODER0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - MODER1"]
    #[inline(always)]
    pub fn moder1(&self) -> MODER1_R {
        MODER1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - MODER2"]
    #[inline(always)]
    pub fn moder2(&self) -> MODER2_R {
        MODER2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - MODER3"]
    #[inline(always)]
    pub fn moder3(&self) -> MODER3_R {
        MODER3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - MODER4"]
    #[inline(always)]
    pub fn moder4(&self) -> MODER4_R {
        MODER4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - MODER5"]
    #[inline(always)]
    pub fn moder5(&self) -> MODER5_R {
        MODER5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - MODER6"]
    #[inline(always)]
    pub fn moder6(&self) -> MODER6_R {
        MODER6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - MODER7"]
    #[inline(always)]
    pub fn moder7(&self) -> MODER7_R {
        MODER7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - MODER8"]
    #[inline(always)]
    pub fn moder8(&self) -> MODER8_R {
        MODER8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - MODER9"]
    #[inline(always)]
    pub fn moder9(&self) -> MODER9_R {
        MODER9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - MODER10"]
    #[inline(always)]
    pub fn moder10(&self) -> MODER10_R {
        MODER10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - MODER11"]
    #[inline(always)]
    pub fn moder11(&self) -> MODER11_R {
        MODER11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - MODER12"]
    #[inline(always)]
    pub fn moder12(&self) -> MODER12_R {
        MODER12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - MODER13"]
    #[inline(always)]
    pub fn moder13(&self) -> MODER13_R {
        MODER13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - MODER14"]
    #[inline(always)]
    pub fn moder14(&self) -> MODER14_R {
        MODER14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - MODER15"]
    #[inline(always)]
    pub fn moder15(&self) -> MODER15_R {
        MODER15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - MODER0"]
    #[inline(always)]
    #[must_use]
    pub fn moder0(&mut self) -> MODER0_W<GPIOF_MODERrs> {
        MODER0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - MODER1"]
    #[inline(always)]
    #[must_use]
    pub fn moder1(&mut self) -> MODER1_W<GPIOF_MODERrs> {
        MODER1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - MODER2"]
    #[inline(always)]
    #[must_use]
    pub fn moder2(&mut self) -> MODER2_W<GPIOF_MODERrs> {
        MODER2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - MODER3"]
    #[inline(always)]
    #[must_use]
    pub fn moder3(&mut self) -> MODER3_W<GPIOF_MODERrs> {
        MODER3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - MODER4"]
    #[inline(always)]
    #[must_use]
    pub fn moder4(&mut self) -> MODER4_W<GPIOF_MODERrs> {
        MODER4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - MODER5"]
    #[inline(always)]
    #[must_use]
    pub fn moder5(&mut self) -> MODER5_W<GPIOF_MODERrs> {
        MODER5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - MODER6"]
    #[inline(always)]
    #[must_use]
    pub fn moder6(&mut self) -> MODER6_W<GPIOF_MODERrs> {
        MODER6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - MODER7"]
    #[inline(always)]
    #[must_use]
    pub fn moder7(&mut self) -> MODER7_W<GPIOF_MODERrs> {
        MODER7_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - MODER8"]
    #[inline(always)]
    #[must_use]
    pub fn moder8(&mut self) -> MODER8_W<GPIOF_MODERrs> {
        MODER8_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - MODER9"]
    #[inline(always)]
    #[must_use]
    pub fn moder9(&mut self) -> MODER9_W<GPIOF_MODERrs> {
        MODER9_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - MODER10"]
    #[inline(always)]
    #[must_use]
    pub fn moder10(&mut self) -> MODER10_W<GPIOF_MODERrs> {
        MODER10_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - MODER11"]
    #[inline(always)]
    #[must_use]
    pub fn moder11(&mut self) -> MODER11_W<GPIOF_MODERrs> {
        MODER11_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - MODER12"]
    #[inline(always)]
    #[must_use]
    pub fn moder12(&mut self) -> MODER12_W<GPIOF_MODERrs> {
        MODER12_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - MODER13"]
    #[inline(always)]
    #[must_use]
    pub fn moder13(&mut self) -> MODER13_W<GPIOF_MODERrs> {
        MODER13_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - MODER14"]
    #[inline(always)]
    #[must_use]
    pub fn moder14(&mut self) -> MODER14_W<GPIOF_MODERrs> {
        MODER14_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - MODER15"]
    #[inline(always)]
    #[must_use]
    pub fn moder15(&mut self) -> MODER15_W<GPIOF_MODERrs> {
        MODER15_W::new(self, 30)
    }
}
#[doc = "GPIO port mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiof_moder::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiof_moder::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOF_MODERrs;
impl crate::RegisterSpec for GPIOF_MODERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiof_moder::R`](R) reader structure"]
impl crate::Readable for GPIOF_MODERrs {}
#[doc = "`write(|w| ..)` method takes [`gpiof_moder::W`](W) writer structure"]
impl crate::Writable for GPIOF_MODERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIOF_MODER to value 0xffff_ffff"]
impl crate::Resettable for GPIOF_MODERrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
