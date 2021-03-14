#[doc = "Reader of register WRPR"]
pub type R = crate::R<u32, super::WRPR>;
#[doc = "Reader of field `WRP`"]
pub type WRP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write protect"]
    #[inline(always)]
    pub fn wrp(&self) -> WRP_R {
        WRP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
