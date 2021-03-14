#[doc = "Reader of register DDRCTRL_DFISTAT"]
pub type R = crate::R<u32, super::DDRCTRL_DFISTAT>;
#[doc = "Reader of field `DFI_INIT_COMPLETE`"]
pub type DFI_INIT_COMPLETE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DFI_LP_ACK`"]
pub type DFI_LP_ACK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - DFI_INIT_COMPLETE"]
    #[inline(always)]
    pub fn dfi_init_complete(&self) -> DFI_INIT_COMPLETE_R {
        DFI_INIT_COMPLETE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DFI_LP_ACK"]
    #[inline(always)]
    pub fn dfi_lp_ack(&self) -> DFI_LP_ACK_R {
        DFI_LP_ACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
