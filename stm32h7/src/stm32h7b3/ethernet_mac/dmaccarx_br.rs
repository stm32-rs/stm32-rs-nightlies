#[doc = "Reader of register DMACCARxBR"]
pub type R = crate::R<u32, super::DMACCARXBR>;
#[doc = "Reader of field `CURRBUFAPTR`"]
pub type CURRBUFAPTR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Application Receive Buffer Address Pointer"]
    #[inline(always)]
    pub fn currbufaptr(&self) -> CURRBUFAPTR_R {
        CURRBUFAPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
