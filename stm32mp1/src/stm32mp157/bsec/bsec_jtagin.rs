#[doc = "Reader of register BSEC_JTAGIN"]
pub type R = crate::R<u32, super::BSEC_JTAGIN>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - DATA"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
