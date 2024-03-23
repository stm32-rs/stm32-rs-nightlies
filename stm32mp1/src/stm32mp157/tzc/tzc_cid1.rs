#[doc = "Register `TZC_CID1` reader"]
pub type R = crate::R<TZC_CID1rs>;
#[doc = "Field `COMP_ID_1` reader - COMP_ID_1"]
pub type COMP_ID_1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - COMP_ID_1"]
    #[inline(always)]
    pub fn comp_id_1(&self) -> COMP_ID_1_R {
        COMP_ID_1_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_cid1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_CID1rs;
impl crate::RegisterSpec for TZC_CID1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_cid1::R`](R) reader structure"]
impl crate::Readable for TZC_CID1rs {}
#[doc = "`reset()` method sets TZC_CID1 to value 0xf0"]
impl crate::Resettable for TZC_CID1rs {
    const RESET_VALUE: u32 = 0xf0;
}
