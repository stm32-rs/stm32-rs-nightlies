#[doc = "Reader of register FMC_BCHPBR2"]
pub type R = crate::R<u32, super::FMC_BCHPBR2>;
#[doc = "Reader of field `BCHPB`"]
pub type BCHPB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - BCHPB"]
    #[inline(always)]
    pub fn bchpb(&self) -> BCHPB_R {
        BCHPB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
