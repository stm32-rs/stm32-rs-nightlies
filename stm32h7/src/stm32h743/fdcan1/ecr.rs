#[doc = "Register `ECR` reader"]
pub type R = crate::R<ECRrs>;
#[doc = "Register `ECR` writer"]
pub type W = crate::W<ECRrs>;
#[doc = "Field `TEC` reader - Transmit Error Counter"]
pub type TEC_R = crate::FieldReader;
#[doc = "Field `TEC` writer - Transmit Error Counter"]
pub type TEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REC` reader - Receive Error Counter"]
pub type REC_R = crate::FieldReader;
#[doc = "Field `REC` writer - Receive Error Counter"]
pub type REC_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RP` reader - Receive Error Passive"]
pub type RP_R = crate::BitReader;
#[doc = "Field `RP` writer - Receive Error Passive"]
pub type RP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEL` reader - AN Error Logging"]
pub type CEL_R = crate::FieldReader;
#[doc = "Field `CEL` writer - AN Error Logging"]
pub type CEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Receive Error Counter"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Receive Error Passive"]
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - AN Error Logging"]
    #[inline(always)]
    pub fn cel(&self) -> CEL_R {
        CEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    #[must_use]
    pub fn tec(&mut self) -> TEC_W<ECRrs> {
        TEC_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Receive Error Counter"]
    #[inline(always)]
    #[must_use]
    pub fn rec(&mut self) -> REC_W<ECRrs> {
        REC_W::new(self, 8)
    }
    #[doc = "Bit 15 - Receive Error Passive"]
    #[inline(always)]
    #[must_use]
    pub fn rp(&mut self) -> RP_W<ECRrs> {
        RP_W::new(self, 15)
    }
    #[doc = "Bits 16:23 - AN Error Logging"]
    #[inline(always)]
    #[must_use]
    pub fn cel(&mut self) -> CEL_W<ECRrs> {
        CEL_W::new(self, 16)
    }
}
#[doc = "FDCAN Error Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECRrs;
impl crate::RegisterSpec for ECRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecr::R`](R) reader structure"]
impl crate::Readable for ECRrs {}
#[doc = "`write(|w| ..)` method takes [`ecr::W`](W) writer structure"]
impl crate::Writable for ECRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for ECRrs {
    const RESET_VALUE: u32 = 0;
}
