#[doc = "Reader of register GPIOA_HWCFGR10"]
pub type R = crate::R<u32, super::GPIOA_HWCFGR10>;
#[doc = "Reader of field `AHB_IOP`"]
pub type AHB_IOP_R = crate::R<u8, u8>;
#[doc = "Reader of field `AF_SIZE`"]
pub type AF_SIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `SPEED_CFG`"]
pub type SPEED_CFG_R = crate::R<u8, u8>;
#[doc = "Reader of field `LOCK_CFG`"]
pub type LOCK_CFG_R = crate::R<u8, u8>;
#[doc = "Reader of field `SEC_CFG`"]
pub type SEC_CFG_R = crate::R<u8, u8>;
#[doc = "Reader of field `OR_CFG`"]
pub type OR_CFG_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - AHB_IOP"]
    #[inline(always)]
    pub fn ahb_iop(&self) -> AHB_IOP_R {
        AHB_IOP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AF_SIZE"]
    #[inline(always)]
    pub fn af_size(&self) -> AF_SIZE_R {
        AF_SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SPEED_CFG"]
    #[inline(always)]
    pub fn speed_cfg(&self) -> SPEED_CFG_R {
        SPEED_CFG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - LOCK_CFG"]
    #[inline(always)]
    pub fn lock_cfg(&self) -> LOCK_CFG_R {
        LOCK_CFG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SEC_CFG"]
    #[inline(always)]
    pub fn sec_cfg(&self) -> SEC_CFG_R {
        SEC_CFG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - OR_CFG"]
    #[inline(always)]
    pub fn or_cfg(&self) -> OR_CFG_R {
        OR_CFG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
