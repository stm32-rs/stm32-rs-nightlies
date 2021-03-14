#[doc = "Reader of register TAMP_HWCFGR2"]
pub type R = crate::R<u32, super::TAMP_HWCFGR2>;
#[doc = "Reader of field `OPTIONREG_OUT`"]
pub type OPTIONREG_OUT_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRUST_ZONE`"]
pub type TRUST_ZONE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - OPTIONREG_OUT"]
    #[inline(always)]
    pub fn optionreg_out(&self) -> OPTIONREG_OUT_R {
        OPTIONREG_OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - TRUST_ZONE"]
    #[inline(always)]
    pub fn trust_zone(&self) -> TRUST_ZONE_R {
        TRUST_ZONE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
