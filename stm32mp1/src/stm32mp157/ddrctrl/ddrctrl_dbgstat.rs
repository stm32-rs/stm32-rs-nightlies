#[doc = "Reader of register DDRCTRL_DBGSTAT"]
pub type R = crate::R<u32, super::DDRCTRL_DBGSTAT>;
#[doc = "Reader of field `RANK0_REFRESH_BUSY`"]
pub type RANK0_REFRESH_BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `ZQ_CALIB_SHORT_BUSY`"]
pub type ZQ_CALIB_SHORT_BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTRLUPD_BUSY`"]
pub type CTRLUPD_BUSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - RANK0_REFRESH_BUSY"]
    #[inline(always)]
    pub fn rank0_refresh_busy(&self) -> RANK0_REFRESH_BUSY_R {
        RANK0_REFRESH_BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - ZQ_CALIB_SHORT_BUSY"]
    #[inline(always)]
    pub fn zq_calib_short_busy(&self) -> ZQ_CALIB_SHORT_BUSY_R {
        ZQ_CALIB_SHORT_BUSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CTRLUPD_BUSY"]
    #[inline(always)]
    pub fn ctrlupd_busy(&self) -> CTRLUPD_BUSY_R {
        CTRLUPD_BUSY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
