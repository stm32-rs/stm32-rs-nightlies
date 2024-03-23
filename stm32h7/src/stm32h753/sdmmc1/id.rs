#[doc = "Register `ID` reader"]
pub type R = crate::R<IDrs>;
#[doc = "Field `IP_ID` reader - SDMMC IP identification."]
pub type IP_ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SDMMC IP identification."]
    #[inline(always)]
    pub fn ip_id(&self) -> IP_ID_R {
        IP_ID_R::new(self.bits)
    }
}
#[doc = "SDMMC IP identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDrs;
impl crate::RegisterSpec for IDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id::R`](R) reader structure"]
impl crate::Readable for IDrs {}
#[doc = "`reset()` method sets ID to value 0x0014_0022"]
impl crate::Resettable for IDrs {
    const RESET_VALUE: u32 = 0x0014_0022;
}
