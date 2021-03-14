#[doc = "Reader of register HWCFGR1"]
pub type R = crate::R<u32, super::HWCFGR1>;
#[doc = "Reader of field `BACKUP_REGS`"]
pub type BACKUP_REGS_R = crate::R<u8, u8>;
#[doc = "Reader of field `TAMPER`"]
pub type TAMPER_R = crate::R<u8, u8>;
#[doc = "Reader of field `ACTIVE_TAMPER`"]
pub type ACTIVE_TAMPER_R = crate::R<u8, u8>;
#[doc = "Reader of field `INT_TAMPER`"]
pub type INT_TAMPER_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:7 - BACKUP_REGS"]
    #[inline(always)]
    pub fn backup_regs(&self) -> BACKUP_REGS_R {
        BACKUP_REGS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - TAMPER"]
    #[inline(always)]
    pub fn tamper(&self) -> TAMPER_R {
        TAMPER_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - ACTIVE_TAMPER"]
    #[inline(always)]
    pub fn active_tamper(&self) -> ACTIVE_TAMPER_R {
        ACTIVE_TAMPER_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - INT_TAMPER"]
    #[inline(always)]
    pub fn int_tamper(&self) -> INT_TAMPER_R {
        INT_TAMPER_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
