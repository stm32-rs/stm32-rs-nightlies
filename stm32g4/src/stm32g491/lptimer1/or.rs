#[doc = "Register `OR` reader"]
pub type R = crate::R<ORrs>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<ORrs>;
#[doc = "Field `IN1` reader - IN1"]
pub type IN1_R = crate::BitReader;
#[doc = "Field `IN1` writer - IN1"]
pub type IN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN2` reader - IN2"]
pub type IN2_R = crate::BitReader;
#[doc = "Field `IN2` writer - IN2"]
pub type IN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN1_2_1` reader - IN1_2_1"]
pub type IN1_2_1_R = crate::FieldReader;
#[doc = "Field `IN1_2_1` writer - IN1_2_1"]
pub type IN1_2_1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IN2_2_1` reader - IN2_2_1"]
pub type IN2_2_1_R = crate::FieldReader;
#[doc = "Field `IN2_2_1` writer - IN2_2_1"]
pub type IN2_2_1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - IN1"]
    #[inline(always)]
    pub fn in1(&self) -> IN1_R {
        IN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IN2"]
    #[inline(always)]
    pub fn in2(&self) -> IN2_R {
        IN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - IN1_2_1"]
    #[inline(always)]
    pub fn in1_2_1(&self) -> IN1_2_1_R {
        IN1_2_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - IN2_2_1"]
    #[inline(always)]
    pub fn in2_2_1(&self) -> IN2_2_1_R {
        IN2_2_1_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IN1"]
    #[inline(always)]
    #[must_use]
    pub fn in1(&mut self) -> IN1_W<ORrs> {
        IN1_W::new(self, 0)
    }
    #[doc = "Bit 1 - IN2"]
    #[inline(always)]
    #[must_use]
    pub fn in2(&mut self) -> IN2_W<ORrs> {
        IN2_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - IN1_2_1"]
    #[inline(always)]
    #[must_use]
    pub fn in1_2_1(&mut self) -> IN1_2_1_W<ORrs> {
        IN1_2_1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - IN2_2_1"]
    #[inline(always)]
    #[must_use]
    pub fn in2_2_1(&mut self) -> IN2_2_1_W<ORrs> {
        IN2_2_1_W::new(self, 4)
    }
}
#[doc = "option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or::R`](R) reader structure"]
impl crate::Readable for ORrs {}
#[doc = "`write(|w| ..)` method takes [`or::W`](W) writer structure"]
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for ORrs {
    const RESET_VALUE: u32 = 0;
}
