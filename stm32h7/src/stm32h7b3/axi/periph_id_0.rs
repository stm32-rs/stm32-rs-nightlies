#[doc = "Register `PERIPH_ID_0` reader"]
pub type R = crate::R<PERIPH_ID_0rs>;
#[doc = "Field `PARTNUM` reader - Peripheral part number bits 0 to 7"]
pub type PARTNUM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Peripheral part number bits 0 to 7"]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "AXI interconnect - peripheral ID0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`periph_id_0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERIPH_ID_0rs;
impl crate::RegisterSpec for PERIPH_ID_0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periph_id_0::R`](R) reader structure"]
impl crate::Readable for PERIPH_ID_0rs {}
#[doc = "`reset()` method sets PERIPH_ID_0 to value 0x04"]
impl crate::Resettable for PERIPH_ID_0rs {
    const RESET_VALUE: u32 = 0x04;
}
