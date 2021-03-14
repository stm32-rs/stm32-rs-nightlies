#[doc = "Reader of register DMACCARxDR"]
pub type R = crate::R<u32, super::DMACCARXDR>;
#[doc = "Reader of field `CURRDESAPTR`"]
pub type CURRDESAPTR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Application Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
