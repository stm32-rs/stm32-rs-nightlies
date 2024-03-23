#[doc = "Register `TZC_CID2` reader"]
pub type R = crate::R<TZC_CID2rs>;
#[doc = "Field `COMP_ID_2` reader - COMP_ID_2"]
pub type COMP_ID_2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - COMP_ID_2"]
    #[inline(always)]
    pub fn comp_id_2(&self) -> COMP_ID_2_R {
        COMP_ID_2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_cid2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_CID2rs;
impl crate::RegisterSpec for TZC_CID2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_cid2::R`](R) reader structure"]
impl crate::Readable for TZC_CID2rs {}
#[doc = "`reset()` method sets TZC_CID2 to value 0x05"]
impl crate::Resettable for TZC_CID2rs {
    const RESET_VALUE: u32 = 0x05;
}
