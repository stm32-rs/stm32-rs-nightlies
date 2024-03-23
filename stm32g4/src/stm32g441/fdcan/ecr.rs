#[doc = "Register `ECR` reader"]
pub type R = crate::R<ECRrs>;
#[doc = "Field `TEC` reader - TEC"]
pub type TEC_R = crate::FieldReader;
#[doc = "Field `REC` reader - TREC"]
pub type REC_R = crate::FieldReader;
#[doc = "Field `RP` reader - RP"]
pub type RP_R = crate::BitReader;
#[doc = "Field `CEL` reader - CEL"]
pub type CEL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - TEC"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - TREC"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 8) & 0x7f) as u8)
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
#[doc = "FDCAN Error Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECRrs;
impl crate::RegisterSpec for ECRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecr::R`](R) reader structure"]
impl crate::Readable for ECRrs {}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for ECRrs {
    const RESET_VALUE: u32 = 0;
}
