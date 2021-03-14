#[doc = "Reader of register SECR"]
pub type R = crate::R<u32, super::SECR>;
#[doc = "Reader of field `SEC_SIZE`"]
pub type SEC_SIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `BOOT_LOCK`"]
pub type BOOT_LOCK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:6 - Securable memory area size"]
    #[inline(always)]
    pub fn sec_size(&self) -> SEC_SIZE_R {
        SEC_SIZE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 16 - used to force boot from user area"]
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
