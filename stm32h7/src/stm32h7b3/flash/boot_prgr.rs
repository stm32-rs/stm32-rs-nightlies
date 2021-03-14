#[doc = "Reader of register BOOT_PRGR"]
pub type R = crate::R<u32, super::BOOT_PRGR>;
#[doc = "Reader of field `BOOT_ADD0`"]
pub type BOOT_ADD0_R = crate::R<u16, u16>;
#[doc = "Reader of field `BOOT_ADD1`"]
pub type BOOT_ADD1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Boot address 0"]
    #[inline(always)]
    pub fn boot_add0(&self) -> BOOT_ADD0_R {
        BOOT_ADD0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Boot address 1"]
    #[inline(always)]
    pub fn boot_add1(&self) -> BOOT_ADD1_R {
        BOOT_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
