#[doc = "Register `FMC_BCHPBR4` reader"]
pub type R = crate::R<FMC_BCHPBR4rs>;
#[doc = "Field `BCHPB` reader - BCHPB"]
pub type BCHPB_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - BCHPB"]
    #[inline(always)]
    pub fn bchpb(&self) -> BCHPB_R {
        BCHPB_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "FMC BCH Parity Bits Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bchpbr4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_BCHPBR4rs;
impl crate::RegisterSpec for FMC_BCHPBR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc_bchpbr4::R`](R) reader structure"]
impl crate::Readable for FMC_BCHPBR4rs {}
#[doc = "`reset()` method sets FMC_BCHPBR4 to value 0"]
impl crate::Resettable for FMC_BCHPBR4rs {
    const RESET_VALUE: u32 = 0;
}
