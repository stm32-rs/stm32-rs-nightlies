#[doc = "Register `MID` reader"]
pub type R = crate::R<MIDrs>;
#[doc = "Field `MID` reader - Magic ID"]
pub type MID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Magic ID"]
    #[inline(always)]
    pub fn mid(&self) -> MID_R {
        MID_R::new(self.bits)
    }
}
#[doc = "magic ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIDrs;
impl crate::RegisterSpec for MIDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mid::R`](R) reader structure"]
impl crate::Readable for MIDrs {}
#[doc = "`reset()` method sets MID to value 0xa3c5_dd01"]
impl crate::Resettable for MIDrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
