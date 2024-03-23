#[doc = "Register `TZC_CID3` reader"]
pub type R = crate::R<TZC_CID3rs>;
#[doc = "Field `COMP_ID_3` reader - COMP_ID_3"]
pub type COMP_ID_3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - COMP_ID_3"]
    #[inline(always)]
    pub fn comp_id_3(&self) -> COMP_ID_3_R {
        COMP_ID_3_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_cid3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_CID3rs;
impl crate::RegisterSpec for TZC_CID3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_cid3::R`](R) reader structure"]
impl crate::Readable for TZC_CID3rs {}
#[doc = "`reset()` method sets TZC_CID3 to value 0xb1"]
impl crate::Resettable for TZC_CID3rs {
    const RESET_VALUE: u32 = 0xb1;
}
