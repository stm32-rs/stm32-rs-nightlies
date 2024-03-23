#[doc = "Register `COMP_ID_0` reader"]
pub type R = crate::R<COMP_ID_0rs>;
#[doc = "Field `PREAMBLE` reader - Preamble bits 0 to 7"]
pub type PREAMBLE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Preamble bits 0 to 7"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "AXI interconnect - component ID0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_id_0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP_ID_0rs;
impl crate::RegisterSpec for COMP_ID_0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_id_0::R`](R) reader structure"]
impl crate::Readable for COMP_ID_0rs {}
#[doc = "`reset()` method sets COMP_ID_0 to value 0x04"]
impl crate::Resettable for COMP_ID_0rs {
    const RESET_VALUE: u32 = 0x04;
}
