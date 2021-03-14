#[doc = "Reader of register FDCAN_TXBCF"]
pub type R = crate::R<u32, super::FDCAN_TXBCF>;
#[doc = "Reader of field `CF`"]
pub type CF_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CF"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
