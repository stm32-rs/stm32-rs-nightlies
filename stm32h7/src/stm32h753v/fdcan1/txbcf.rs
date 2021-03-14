#[doc = "Reader of register TXBCF"]
pub type R = crate::R<u32, super::TXBCF>;
#[doc = "Reader of field `CF`"]
pub type CF_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
