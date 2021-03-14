#[doc = "Reader of register TSSSR"]
pub type R = crate::R<u32, super::TSSSR>;
#[doc = "Reader of field `SS`"]
pub type SS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Sub second value"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
