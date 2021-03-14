#[doc = "Reader of register UR5"]
pub type R = crate::R<u32, super::UR5>;
#[doc = "Reader of field `MESAD_1`"]
pub type MESAD_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRPN_1`"]
pub type WRPN_1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Mass erase secured area disabled for bank 1"]
    #[inline(always)]
    pub fn mesad_1(&self) -> MESAD_1_R {
        MESAD_1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Write protection for flash bank 1"]
    #[inline(always)]
    pub fn wrpn_1(&self) -> WRPN_1_R {
        WRPN_1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
