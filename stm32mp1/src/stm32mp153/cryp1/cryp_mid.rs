#[doc = "Register `CRYP_MID` reader"]
pub type R = crate::R<CRYP_MIDrs>;
#[doc = "Field `MID` reader - MID"]
pub type MID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - MID"]
    #[inline(always)]
    pub fn mid(&self) -> MID_R {
        MID_R::new(self.bits)
    }
}
#[doc = "CRYP HW Magic ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_mid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYP_MIDrs;
impl crate::RegisterSpec for CRYP_MIDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cryp_mid::R`](R) reader structure"]
impl crate::Readable for CRYP_MIDrs {}
#[doc = "`reset()` method sets CRYP_MID to value 0xa3c5_dd01"]
impl crate::Resettable for CRYP_MIDrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
