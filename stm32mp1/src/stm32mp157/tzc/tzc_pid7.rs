#[doc = "Reader of register TZC_PID7"]
pub type R = crate::R<u32, super::TZC_PID7>;
#[doc = "Reader of field `PER_ID_7`"]
pub type PER_ID_7_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PER_ID_7"]
    #[inline(always)]
    pub fn per_id_7(&self) -> PER_ID_7_R {
        PER_ID_7_R::new((self.bits & 0xff) as u8)
    }
}
