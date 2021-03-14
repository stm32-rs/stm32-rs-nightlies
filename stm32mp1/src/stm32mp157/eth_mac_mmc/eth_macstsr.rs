#[doc = "Reader of register ETH_MACSTSR"]
pub type R = crate::R<u32, super::ETH_MACSTSR>;
#[doc = "Reader of field `TSS`"]
pub type TSS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - TSS"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
