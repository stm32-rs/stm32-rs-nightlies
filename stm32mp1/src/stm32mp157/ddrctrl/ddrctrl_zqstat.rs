#[doc = "Reader of register DDRCTRL_ZQSTAT"]
pub type R = crate::R<u32, super::DDRCTRL_ZQSTAT>;
#[doc = "Reader of field `ZQ_RESET_BUSY`"]
pub type ZQ_RESET_BUSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ZQ_RESET_BUSY"]
    #[inline(always)]
    pub fn zq_reset_busy(&self) -> ZQ_RESET_BUSY_R {
        ZQ_RESET_BUSY_R::new((self.bits & 0x01) != 0)
    }
}
