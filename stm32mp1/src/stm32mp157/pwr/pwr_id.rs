#[doc = "Reader of register PWR_ID"]
pub type R = crate::R<u32, super::PWR_ID>;
#[doc = "Reader of field `IPID`"]
pub type IPID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPID"]
    #[inline(always)]
    pub fn ipid(&self) -> IPID_R {
        IPID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
