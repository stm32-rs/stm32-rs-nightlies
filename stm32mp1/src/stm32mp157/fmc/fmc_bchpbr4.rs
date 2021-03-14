#[doc = "Reader of register FMC_BCHPBR4"]
pub type R = crate::R<u32, super::FMC_BCHPBR4>;
#[doc = "Reader of field `BCHPB`"]
pub type BCHPB_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - BCHPB"]
    #[inline(always)]
    pub fn bchpb(&self) -> BCHPB_R {
        BCHPB_R::new((self.bits & 0xff) as u8)
    }
}
