#[doc = "Reader of register UR9"]
pub type R = crate::R<u32, super::UR9>;
#[doc = "Reader of field `WRPN_2`"]
pub type WRPN_2_R = crate::R<u8, u8>;
#[doc = "Reader of field `PA_BEG_2`"]
pub type PA_BEG_2_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:7 - Write protection for flash bank 2"]
    #[inline(always)]
    pub fn wrpn_2(&self) -> WRPN_2_R {
        WRPN_2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:27 - Protected area start address for bank 2"]
    #[inline(always)]
    pub fn pa_beg_2(&self) -> PA_BEG_2_R {
        PA_BEG_2_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
