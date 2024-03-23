#[doc = "Register `AXIMC_COMP_ID_1` reader"]
pub type R = crate::R<AXIMC_COMP_ID_1rs>;
#[doc = "Field `PREAMBLE` reader - PREAMBLE"]
pub type PREAMBLE_R = crate::FieldReader;
#[doc = "Field `CLASS` reader - CLASS"]
pub type CLASS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - PREAMBLE"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CLASS"]
    #[inline(always)]
    pub fn class(&self) -> CLASS_R {
        CLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "AXIMC component ID1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_comp_id_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXIMC_COMP_ID_1rs;
impl crate::RegisterSpec for AXIMC_COMP_ID_1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aximc_comp_id_1::R`](R) reader structure"]
impl crate::Readable for AXIMC_COMP_ID_1rs {}
#[doc = "`reset()` method sets AXIMC_COMP_ID_1 to value 0xf0"]
impl crate::Resettable for AXIMC_COMP_ID_1rs {
    const RESET_VALUE: u32 = 0xf0;
}
