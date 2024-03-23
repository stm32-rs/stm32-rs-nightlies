#[doc = "Register `OR` reader"]
pub type R = crate::R<ORrs>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<ORrs>;
#[doc = "Field `OUT3_RMP` reader - TAMP_OUT3 mapping"]
pub type OUT3_RMP_R = crate::FieldReader;
#[doc = "Field `OUT3_RMP` writer - TAMP_OUT3 mapping"]
pub type OUT3_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OUT5_RMP` reader - TAMP_OUT5 mapping"]
pub type OUT5_RMP_R = crate::BitReader;
#[doc = "Field `OUT5_RMP` writer - TAMP_OUT5 mapping"]
pub type OUT5_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN2_RMP` reader - TAMP_IN2 mapping"]
pub type IN2_RMP_R = crate::BitReader;
#[doc = "Field `IN2_RMP` writer - TAMP_IN2 mapping"]
pub type IN2_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN3_RMP` reader - TAMP_IN3 mapping"]
pub type IN3_RMP_R = crate::BitReader;
#[doc = "Field `IN3_RMP` writer - TAMP_IN3 mapping"]
pub type IN3_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN4_RMP` reader - TAMP_IN4 mapping"]
pub type IN4_RMP_R = crate::BitReader;
#[doc = "Field `IN4_RMP` writer - TAMP_IN4 mapping"]
pub type IN4_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 1:2 - TAMP_OUT3 mapping"]
    #[inline(always)]
    pub fn out3_rmp(&self) -> OUT3_RMP_R {
        OUT3_RMP_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - TAMP_OUT5 mapping"]
    #[inline(always)]
    pub fn out5_rmp(&self) -> OUT5_RMP_R {
        OUT5_RMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - TAMP_IN2 mapping"]
    #[inline(always)]
    pub fn in2_rmp(&self) -> IN2_RMP_R {
        IN2_RMP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TAMP_IN3 mapping"]
    #[inline(always)]
    pub fn in3_rmp(&self) -> IN3_RMP_R {
        IN3_RMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TAMP_IN4 mapping"]
    #[inline(always)]
    pub fn in4_rmp(&self) -> IN4_RMP_R {
        IN4_RMP_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:2 - TAMP_OUT3 mapping"]
    #[inline(always)]
    #[must_use]
    pub fn out3_rmp(&mut self) -> OUT3_RMP_W<ORrs> {
        OUT3_RMP_W::new(self, 1)
    }
    #[doc = "Bit 3 - TAMP_OUT5 mapping"]
    #[inline(always)]
    #[must_use]
    pub fn out5_rmp(&mut self) -> OUT5_RMP_W<ORrs> {
        OUT5_RMP_W::new(self, 3)
    }
    #[doc = "Bit 8 - TAMP_IN2 mapping"]
    #[inline(always)]
    #[must_use]
    pub fn in2_rmp(&mut self) -> IN2_RMP_W<ORrs> {
        IN2_RMP_W::new(self, 8)
    }
    #[doc = "Bit 9 - TAMP_IN3 mapping"]
    #[inline(always)]
    #[must_use]
    pub fn in3_rmp(&mut self) -> IN3_RMP_W<ORrs> {
        IN3_RMP_W::new(self, 9)
    }
    #[doc = "Bit 10 - TAMP_IN4 mapping"]
    #[inline(always)]
    #[must_use]
    pub fn in4_rmp(&mut self) -> IN4_RMP_W<ORrs> {
        IN4_RMP_W::new(self, 10)
    }
}
#[doc = "TAMP option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
