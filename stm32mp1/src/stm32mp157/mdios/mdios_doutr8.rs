#[doc = "Reader of register MDIOS_DOUTR8"]
pub type R = crate::R<u32, super::MDIOS_DOUTR8>;
#[doc = "Reader of field `DOUT`"]
pub type DOUT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - DOUT"]
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0xffff) as u16)
    }
}
