#[doc = "Register `PERIPH_ID_1` reader"]
pub type R = crate::R<PERIPH_ID_1rs>;
#[doc = "Field `PARTNUM` reader - Peripheral part number bits 8 to 11"]
pub type PARTNUM_R = crate::FieldReader;
#[doc = "Field `JEP106I` reader - JEP106 identity bits 0 to 3"]
pub type JEP106I_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Peripheral part number bits 8 to 11"]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - JEP106 identity bits 0 to 3"]
    #[inline(always)]
    pub fn jep106i(&self) -> JEP106I_R {
        JEP106I_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "AXI interconnect - peripheral ID1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`periph_id_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERIPH_ID_1rs;
impl crate::RegisterSpec for PERIPH_ID_1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periph_id_1::R`](R) reader structure"]
impl crate::Readable for PERIPH_ID_1rs {}
#[doc = "`reset()` method sets PERIPH_ID_1 to value 0x04"]
impl crate::Resettable for PERIPH_ID_1rs {
    const RESET_VALUE: u32 = 0x04;
}
