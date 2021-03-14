#[doc = "Reader of register DOR"]
pub type R = crate::R<u32, super::DOR>;
#[doc = "Reader of field `DATAOUT`"]
pub type DATAOUT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Output FIFO Output FIFO data register."]
    #[inline(always)]
    pub fn dataout(&self) -> DATAOUT_R {
        DATAOUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
