#[doc = "Reader of register PKGR"]
pub type R = crate::R<u32, super::PKGR>;
#[doc = "Reader of field `PKG`"]
pub type PKG_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Package"]
    #[inline(always)]
    pub fn pkg(&self) -> PKG_R {
        PKG_R::new((self.bits & 0x0f) as u8)
    }
}
