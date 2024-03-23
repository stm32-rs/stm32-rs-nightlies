#[doc = "Register `FMC_BCHPBR1` reader"]
pub type R = crate::R<FMC_BCHPBR1rs>;
#[doc = "Field `BCHPB` reader - BCHPB"]
pub type BCHPB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - BCHPB"]
    #[inline(always)]
    pub fn bchpb(&self) -> BCHPB_R {
        BCHPB_R::new(self.bits)
    }
}
#[doc = "These registers contain the BCH parity bits (BCHPB). For the BCH 4-bit, only BCHPB\\[51:0\\]
are significant and for the BCH 8-bit BCHPB\\[103:0\\]
are significant.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bchpbr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_BCHPBR1rs;
impl crate::RegisterSpec for FMC_BCHPBR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc_bchpbr1::R`](R) reader structure"]
impl crate::Readable for FMC_BCHPBR1rs {}
#[doc = "`reset()` method sets FMC_BCHPBR1 to value 0"]
impl crate::Resettable for FMC_BCHPBR1rs {
    const RESET_VALUE: u32 = 0;
}
