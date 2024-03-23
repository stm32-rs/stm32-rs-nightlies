#[doc = "Register `HASH_MID` reader"]
pub type R = crate::R<HASH_MIDrs>;
#[doc = "Field `MID` reader - MID"]
pub type MID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - MID"]
    #[inline(always)]
    pub fn mid(&self) -> MID_R {
        MID_R::new(self.bits)
    }
}
#[doc = "HASH Hardware Magic ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_mid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_MIDrs;
impl crate::RegisterSpec for HASH_MIDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_mid::R`](R) reader structure"]
impl crate::Readable for HASH_MIDrs {}
#[doc = "`reset()` method sets HASH_MID to value 0xa3c5_dd01"]
impl crate::Resettable for HASH_MIDrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
