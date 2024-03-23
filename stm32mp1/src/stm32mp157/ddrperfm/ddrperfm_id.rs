#[doc = "Register `DDRPERFM_ID` reader"]
pub type R = crate::R<DDRPERFM_IDrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "DDRPERFM ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrperfm_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPERFM_IDrs;
impl crate::RegisterSpec for DDRPERFM_IDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrperfm_id::R`](R) reader structure"]
impl crate::Readable for DDRPERFM_IDrs {}
#[doc = "`reset()` method sets DDRPERFM_ID to value 0x0014_0061"]
impl crate::Resettable for DDRPERFM_IDrs {
    const RESET_VALUE: u32 = 0x0014_0061;
}
