#[doc = "Reader of register FMC_HECCR"]
pub type R = crate::R<u32, super::FMC_HECCR>;
#[doc = "Reader of field `HECC`"]
pub type HECC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - HECC"]
    #[inline(always)]
    pub fn hecc(&self) -> HECC_R {
        HECC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
