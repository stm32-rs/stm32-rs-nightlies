#[doc = "Reader of register DDRCTRL_DBGCAM"]
pub type R = crate::R<u32, super::DDRCTRL_DBGCAM>;
#[doc = "Reader of field `DBG_HPR_Q_DEPTH`"]
pub type DBG_HPR_Q_DEPTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `DBG_LPR_Q_DEPTH`"]
pub type DBG_LPR_Q_DEPTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `DBG_W_Q_DEPTH`"]
pub type DBG_W_Q_DEPTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `DBG_STALL`"]
pub type DBG_STALL_R = crate::R<bool, bool>;
#[doc = "Reader of field `DBG_RD_Q_EMPTY`"]
pub type DBG_RD_Q_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `DBG_WR_Q_EMPTY`"]
pub type DBG_WR_Q_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RD_DATA_PIPELINE_EMPTY`"]
pub type RD_DATA_PIPELINE_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `WR_DATA_PIPELINE_EMPTY`"]
pub type WR_DATA_PIPELINE_EMPTY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:4 - DBG_HPR_Q_DEPTH"]
    #[inline(always)]
    pub fn dbg_hpr_q_depth(&self) -> DBG_HPR_Q_DEPTH_R {
        DBG_HPR_Q_DEPTH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DBG_LPR_Q_DEPTH"]
    #[inline(always)]
    pub fn dbg_lpr_q_depth(&self) -> DBG_LPR_Q_DEPTH_R {
        DBG_LPR_Q_DEPTH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - DBG_W_Q_DEPTH"]
    #[inline(always)]
    pub fn dbg_w_q_depth(&self) -> DBG_W_Q_DEPTH_R {
        DBG_W_Q_DEPTH_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - DBG_STALL"]
    #[inline(always)]
    pub fn dbg_stall(&self) -> DBG_STALL_R {
        DBG_STALL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DBG_RD_Q_EMPTY"]
    #[inline(always)]
    pub fn dbg_rd_q_empty(&self) -> DBG_RD_Q_EMPTY_R {
        DBG_RD_Q_EMPTY_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DBG_WR_Q_EMPTY"]
    #[inline(always)]
    pub fn dbg_wr_q_empty(&self) -> DBG_WR_Q_EMPTY_R {
        DBG_WR_Q_EMPTY_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - RD_DATA_PIPELINE_EMPTY"]
    #[inline(always)]
    pub fn rd_data_pipeline_empty(&self) -> RD_DATA_PIPELINE_EMPTY_R {
        RD_DATA_PIPELINE_EMPTY_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - WR_DATA_PIPELINE_EMPTY"]
    #[inline(always)]
    pub fn wr_data_pipeline_empty(&self) -> WR_DATA_PIPELINE_EMPTY_R {
        WR_DATA_PIPELINE_EMPTY_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
