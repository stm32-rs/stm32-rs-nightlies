#[doc = "Register `FMC_BCHPBR2` reader"]
pub type R = crate::R<FMC_BCHPBR2rs>;
#[doc = "Field `BCHPB` reader - BCHPB"]
pub type BCHPB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - BCHPB"]
    #[inline(always)]
    pub fn bchpb(&self) -> BCHPB_R {
        BCHPB_R::new(self.bits)
    }
}
#[doc = "FMC BCH Parity Bits Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bchpbr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_BCHPBR2rs;
impl crate::RegisterSpec for FMC_BCHPBR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc_bchpbr2::R`](R) reader structure"]
impl crate::Readable for FMC_BCHPBR2rs {}
#[doc = "`reset()` method sets FMC_BCHPBR2 to value 0"]
impl crate::Resettable for FMC_BCHPBR2rs {
    const RESET_VALUE: u32 = 0;
}
