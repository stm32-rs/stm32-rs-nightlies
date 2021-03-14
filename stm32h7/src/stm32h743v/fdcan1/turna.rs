#[doc = "Reader of register TURNA"]
pub type R = crate::R<u32, super::TURNA>;
#[doc = "Reader of field `NAV`"]
pub type NAV_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:17 - Numerator Actual Value"]
    #[inline(always)]
    pub fn nav(&self) -> NAV_R {
        NAV_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
