#[doc = "Reader of register HWCFGR2"]
pub type R = crate::R<u32, super::HWCFGR2>;
#[doc = "Reader of field `PTIONREG_OUT`"]
pub type PTIONREG_OUT_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRUST_ZONE`"]
pub type TRUST_ZONE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PTIONREG_OUT"]
    #[inline(always)]
    pub fn ptionreg_out(&self) -> PTIONREG_OUT_R {
        PTIONREG_OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - TRUST_ZONE"]
    #[inline(always)]
    pub fn trust_zone(&self) -> TRUST_ZONE_R {
        TRUST_ZONE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
