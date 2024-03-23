#[doc = "Register `PERIPH_ID_4` reader"]
pub type R = crate::R<PERIPH_ID_4rs>;
#[doc = "Field `JEP106CON` reader - JEP106 continuation code"]
pub type JEP106CON_R = crate::FieldReader;
#[doc = "Field `KCOUNT4` reader - Register file size"]
pub type KCOUNT4_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - JEP106 continuation code"]
    #[inline(always)]
    pub fn jep106con(&self) -> JEP106CON_R {
        JEP106CON_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Register file size"]
    #[inline(always)]
    pub fn kcount4(&self) -> KCOUNT4_R {
        KCOUNT4_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "AXI interconnect - peripheral ID4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`periph_id_4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERIPH_ID_4rs;
impl crate::RegisterSpec for PERIPH_ID_4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periph_id_4::R`](R) reader structure"]
impl crate::Readable for PERIPH_ID_4rs {}
#[doc = "`reset()` method sets PERIPH_ID_4 to value 0x04"]
impl crate::Resettable for PERIPH_ID_4rs {
    const RESET_VALUE: u32 = 0x04;
}
