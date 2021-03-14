#[doc = "Reader of register DOUTR"]
pub type R = crate::R<u32, super::DOUTR>;
#[doc = "Reader of field `DOUT`"]
pub type DOUT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data output register"]
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
