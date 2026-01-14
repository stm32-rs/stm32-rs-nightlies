///Register `ECR` reader
pub type R = crate::R<ECRrs>;
///Field `TEC` reader - TEC
pub type TEC_R = crate::FieldReader;
///Field `REC` reader - TREC
pub type REC_R = crate::FieldReader;
///Field `RP` reader - RP
pub type RP_R = crate::BitReader;
///Field `CEL` reader - CEL
pub type CEL_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - TEC
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:14 - TREC
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 15 - RP
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - CEL
    #[inline(always)]
    pub fn cel(&self) -> CEL_R {
        CEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECR")
            .field("tec", &self.tec())
            .field("rec", &self.rec())
            .field("rp", &self.rp())
            .field("cel", &self.cel())
            .finish()
    }
}
/**FDCAN Error Counter Register

You can [`read`](crate::Reg::read) this register and get [`ecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473.html#FDCAN1:ECR)*/
pub struct ECRrs;
impl crate::RegisterSpec for ECRrs {
    type Ux = u32;
}
///`read()` method returns [`ecr::R`](R) reader structure
impl crate::Readable for ECRrs {}
///`reset()` method sets ECR to value 0
impl crate::Resettable for ECRrs {}
