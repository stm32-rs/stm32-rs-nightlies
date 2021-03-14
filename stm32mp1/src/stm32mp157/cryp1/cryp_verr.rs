#[doc = "Reader of register CRYP_VERR"]
pub type R = crate::R<u32, super::CRYP_VERR>;
#[doc = "Reader of field `VER`"]
pub type VER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - VER"]
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new((self.bits & 0xff) as u8)
    }
}
