#[doc = "Reader of register DDRCTRL_SWSTAT"]
pub type R = crate::R<u32, super::DDRCTRL_SWSTAT>;
#[doc = "Reader of field `SW_DONE_ACK`"]
pub type SW_DONE_ACK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SW_DONE_ACK"]
    #[inline(always)]
    pub fn sw_done_ack(&self) -> SW_DONE_ACK_R {
        SW_DONE_ACK_R::new((self.bits & 0x01) != 0)
    }
}
