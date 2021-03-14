#[doc = "Reader of register UR12"]
pub type R = crate::R<u32, super::UR12>;
#[doc = "Reader of field `SECURE`"]
pub type SECURE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 16 - Secure mode"]
    #[inline(always)]
    pub fn secure(&self) -> SECURE_R {
        SECURE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
