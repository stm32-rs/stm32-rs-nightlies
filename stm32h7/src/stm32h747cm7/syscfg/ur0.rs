#[doc = "Reader of register UR0"]
pub type R = crate::R<u32, super::UR0>;
#[doc = "Reader of field `BKS`"]
pub type BKS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RDP`"]
pub type RDP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Bank Swap"]
    #[inline(always)]
    pub fn bks(&self) -> BKS_R {
        BKS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Readout protection"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
