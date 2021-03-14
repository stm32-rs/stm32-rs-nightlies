#[doc = "Reader of register TZC_PID1"]
pub type R = crate::R<u32, super::TZC_PID1>;
#[doc = "Reader of field `PER_ID_1`"]
pub type PER_ID_1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PER_ID_1"]
    #[inline(always)]
    pub fn per_id_1(&self) -> PER_ID_1_R {
        PER_ID_1_R::new((self.bits & 0xff) as u8)
    }
}
