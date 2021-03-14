#[doc = "Reader of register PWR_CSR1"]
pub type R = crate::R<u32, super::PWR_CSR1>;
#[doc = "Reader of field `PVDO`"]
pub type PVDO_R = crate::R<bool, bool>;
#[doc = "Reader of field `AVDO`"]
pub type AVDO_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 4 - PVDO"]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - AVDO"]
    #[inline(always)]
    pub fn avdo(&self) -> AVDO_R {
        AVDO_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
