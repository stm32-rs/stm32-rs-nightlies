#[doc = "Reader of register RIR"]
pub type R = crate::R<u32, super::RIR>;
#[doc = "Reader of field `STID`"]
pub type STID_R = crate::R<u16, u16>;
#[doc = "Reader of field `EXID`"]
pub type EXID_R = crate::R<u32, u32>;
#[doc = "Reader of field `IDE`"]
pub type IDE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTR`"]
pub type RTR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 21:31 - STID"]
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
    #[doc = "Bits 3:20 - EXID"]
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new(((self.bits >> 3) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTR"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
