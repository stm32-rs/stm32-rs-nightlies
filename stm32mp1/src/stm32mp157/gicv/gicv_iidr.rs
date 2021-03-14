#[doc = "Reader of register GICV_IIDR"]
pub type R = crate::R<u32, super::GICV_IIDR>;
#[doc = "Reader of field `IIDR`"]
pub type IIDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IIDR"]
    #[inline(always)]
    pub fn iidr(&self) -> IIDR_R {
        IIDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
