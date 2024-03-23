#[doc = "Register `AXIMC_COMP_ID_0` reader"]
pub type R = crate::R<AXIMC_COMP_ID_0rs>;
#[doc = "Field `PREAMBLE` reader - PREAMBLE"]
pub type PREAMBLE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - PREAMBLE"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "AXIMC component ID0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_comp_id_0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXIMC_COMP_ID_0rs;
impl crate::RegisterSpec for AXIMC_COMP_ID_0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aximc_comp_id_0::R`](R) reader structure"]
impl crate::Readable for AXIMC_COMP_ID_0rs {}
#[doc = "`reset()` method sets AXIMC_COMP_ID_0 to value 0x0d"]
impl crate::Resettable for AXIMC_COMP_ID_0rs {
    const RESET_VALUE: u32 = 0x0d;
}
