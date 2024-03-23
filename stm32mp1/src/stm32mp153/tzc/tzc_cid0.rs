#[doc = "Register `TZC_CID0` reader"]
pub type R = crate::R<TZC_CID0rs>;
#[doc = "Field `COMP_ID_0` reader - COMP_ID_0"]
pub type COMP_ID_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - COMP_ID_0"]
    #[inline(always)]
    pub fn comp_id_0(&self) -> COMP_ID_0_R {
        COMP_ID_0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_cid0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_CID0rs;
impl crate::RegisterSpec for TZC_CID0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_cid0::R`](R) reader structure"]
impl crate::Readable for TZC_CID0rs {}
#[doc = "`reset()` method sets TZC_CID0 to value 0x0d"]
impl crate::Resettable for TZC_CID0rs {
    const RESET_VALUE: u32 = 0x0d;
}
