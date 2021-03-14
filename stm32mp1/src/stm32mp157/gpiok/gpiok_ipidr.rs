#[doc = "Reader of register GPIOK_IPIDR"]
pub type R = crate::R<u32, super::GPIOK_IPIDR>;
#[doc = "Reader of field `IPIDR`"]
pub type IPIDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPIDR"]
    #[inline(always)]
    pub fn ipidr(&self) -> IPIDR_R {
        IPIDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
