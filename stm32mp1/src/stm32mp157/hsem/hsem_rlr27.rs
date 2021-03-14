#[doc = "Reader of register HSEM_RLR27"]
pub type R = crate::R<u32, super::HSEM_RLR27>;
#[doc = "Reader of field `PROCID`"]
pub type PROCID_R = crate::R<u8, u8>;
#[doc = "Reader of field `COREID`"]
pub type COREID_R = crate::R<u8, u8>;
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:7 - PROCID"]
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - COREID"]
    #[inline(always)]
    pub fn coreid(&self) -> COREID_R {
        COREID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
