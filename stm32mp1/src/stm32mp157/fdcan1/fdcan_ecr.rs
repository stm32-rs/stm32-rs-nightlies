#[doc = "Register `FDCAN_ECR` reader"]
pub type R = crate::R<FDCAN_ECRrs>;
#[doc = "Register `FDCAN_ECR` writer"]
pub type W = crate::W<FDCAN_ECRrs>;
#[doc = "Field `TEC` reader - TEC"]
pub type TEC_R = crate::FieldReader;
#[doc = "Field `TREC` reader - TREC"]
pub type TREC_R = crate::FieldReader;
#[doc = "Field `RP` reader - RP"]
pub type RP_R = crate::BitReader;
#[doc = "Field `CEL` reader - CEL"]
pub type CEL_R = crate::FieldReader;
#[doc = "Field `CEL` writer - CEL"]
pub type CEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TEC"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - TREC"]
    #[inline(always)]
    pub fn trec(&self) -> TREC_R {
        TREC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - RP"]
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - CEL"]
    #[inline(always)]
    pub fn cel(&self) -> CEL_R {
        CEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - CEL"]
    #[inline(always)]
    #[must_use]
    pub fn cel(&mut self) -> CEL_W<FDCAN_ECRrs> {
        CEL_W::new(self, 16)
    }
}
#[doc = "FDCAN error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_ECRrs;
impl crate::RegisterSpec for FDCAN_ECRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ecr::R`](R) reader structure"]
impl crate::Readable for FDCAN_ECRrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ecr::W`](W) writer structure"]
impl crate::Writable for FDCAN_ECRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_ECR to value 0"]
impl crate::Resettable for FDCAN_ECRrs {
    const RESET_VALUE: u32 = 0;
}
