#[doc = "Reader of register DFSDM2_CNVTIMR"]
pub type R = crate::R<u32, super::DFSDM2_CNVTIMR>;
#[doc = "Reader of field `CNVCNT`"]
pub type CNVCNT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 4:31 - 28-bit timer counting conversion time"]
    #[inline(always)]
    pub fn cnvcnt(&self) -> CNVCNT_R {
        CNVCNT_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
}
