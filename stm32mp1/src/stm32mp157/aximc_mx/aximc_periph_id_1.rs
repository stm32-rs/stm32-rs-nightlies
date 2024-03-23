#[doc = "Register `AXIMC_PERIPH_ID_1` reader"]
pub type R = crate::R<AXIMC_PERIPH_ID_1rs>;
#[doc = "Field `PERIPH_ID_1` reader - PERIPH_ID_1"]
pub type PERIPH_ID_1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - PERIPH_ID_1"]
    #[inline(always)]
    pub fn periph_id_1(&self) -> PERIPH_ID_1_R {
        PERIPH_ID_1_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "AXIMC peripheral ID1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_periph_id_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXIMC_PERIPH_ID_1rs;
impl crate::RegisterSpec for AXIMC_PERIPH_ID_1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aximc_periph_id_1::R`](R) reader structure"]
impl crate::Readable for AXIMC_PERIPH_ID_1rs {}
#[doc = "`reset()` method sets AXIMC_PERIPH_ID_1 to value 0xb4"]
impl crate::Resettable for AXIMC_PERIPH_ID_1rs {
    const RESET_VALUE: u32 = 0xb4;
}
