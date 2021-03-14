#[doc = "Reader of register DDRCTRL_MRSTAT"]
pub type R = crate::R<u32, super::DDRCTRL_MRSTAT>;
#[doc = "Reader of field `MR_WR_BUSY`"]
pub type MR_WR_BUSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - MR_WR_BUSY"]
    #[inline(always)]
    pub fn mr_wr_busy(&self) -> MR_WR_BUSY_R {
        MR_WR_BUSY_R::new((self.bits & 0x01) != 0)
    }
}
