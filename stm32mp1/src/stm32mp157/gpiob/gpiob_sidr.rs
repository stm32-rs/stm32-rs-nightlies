#[doc = "Reader of register GPIOB_SIDR"]
pub type R = crate::R<u32, super::GPIOB_SIDR>;
#[doc = "Reader of field `SIDR`"]
pub type SIDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - SIDR"]
    #[inline(always)]
    pub fn sidr(&self) -> SIDR_R {
        SIDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
