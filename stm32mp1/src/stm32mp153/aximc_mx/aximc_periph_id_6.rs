#[doc = "Register `AXIMC_PERIPH_ID_6` reader"]
pub type R = crate::R<AXIMC_PERIPH_ID_6rs>;
#[doc = "Field `PERIPH_ID_6` reader - PERIPH_ID_6"]
pub type PERIPH_ID_6_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - PERIPH_ID_6"]
    #[inline(always)]
    pub fn periph_id_6(&self) -> PERIPH_ID_6_R {
        PERIPH_ID_6_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "AXIMC peripheral ID6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_periph_id_6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXIMC_PERIPH_ID_6rs;
impl crate::RegisterSpec for AXIMC_PERIPH_ID_6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aximc_periph_id_6::R`](R) reader structure"]
impl crate::Readable for AXIMC_PERIPH_ID_6rs {}
#[doc = "`reset()` method sets AXIMC_PERIPH_ID_6 to value 0"]
impl crate::Resettable for AXIMC_PERIPH_ID_6rs {
    const RESET_VALUE: u32 = 0;
}
