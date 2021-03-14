#[doc = "Reader of register MDIOS_WRFR"]
pub type R = crate::R<u32, super::MDIOS_WRFR>;
#[doc = "Reader of field `WRF`"]
pub type WRF_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - WRF"]
    #[inline(always)]
    pub fn wrf(&self) -> WRF_R {
        WRF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
