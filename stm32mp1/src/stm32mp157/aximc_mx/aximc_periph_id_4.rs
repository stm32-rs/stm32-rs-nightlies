#[doc = "Register `AXIMC_PERIPH_ID_4` reader"]
pub type R = crate::R<AXIMC_PERIPH_ID_4rs>;
#[doc = "Field `JEP106CON` reader - JEP106CON"]
pub type JEP106CON_R = crate::FieldReader;
#[doc = "Field `K4COUNT` reader - K4COUNT"]
pub type K4COUNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - JEP106CON"]
    #[inline(always)]
    pub fn jep106con(&self) -> JEP106CON_R {
        JEP106CON_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - K4COUNT"]
    #[inline(always)]
    pub fn k4count(&self) -> K4COUNT_R {
        K4COUNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "AXIMC peripheral ID4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aximc_periph_id_4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXIMC_PERIPH_ID_4rs;
impl crate::RegisterSpec for AXIMC_PERIPH_ID_4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aximc_periph_id_4::R`](R) reader structure"]
impl crate::Readable for AXIMC_PERIPH_ID_4rs {}
#[doc = "`reset()` method sets AXIMC_PERIPH_ID_4 to value 0x04"]
impl crate::Resettable for AXIMC_PERIPH_ID_4rs {
    const RESET_VALUE: u32 = 0x04;
}
