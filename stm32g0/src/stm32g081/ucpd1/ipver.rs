#[doc = "Reader of register IPVER"]
pub type R = crate::R<u32, super::IPVER>;
#[doc = "Reader of field `IPVER`"]
pub type IPVER_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPVER"]
    #[inline(always)]
    pub fn ipver(&self) -> IPVER_R {
        IPVER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
