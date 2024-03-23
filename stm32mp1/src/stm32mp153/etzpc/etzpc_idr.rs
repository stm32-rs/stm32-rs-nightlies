#[doc = "Register `ETZPC_IDR` reader"]
pub type R = crate::R<ETZPC_IDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "ETZPC IP version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etzpc_idr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETZPC_IDRrs;
impl crate::RegisterSpec for ETZPC_IDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etzpc_idr::R`](R) reader structure"]
impl crate::Readable for ETZPC_IDRrs {}
#[doc = "`reset()` method sets ETZPC_IDR to value 0x0010_0061"]
impl crate::Resettable for ETZPC_IDRrs {
    const RESET_VALUE: u32 = 0x0010_0061;
}
