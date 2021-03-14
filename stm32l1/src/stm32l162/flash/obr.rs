#[doc = "Reader of register OBR"]
pub type R = crate::R<u32, super::OBR>;
#[doc = "Reader of field `RDPRT`"]
pub type RDPRT_R = crate::R<u8, u8>;
#[doc = "Reader of field `BOR_LEV`"]
pub type BOR_LEV_R = crate::R<u8, u8>;
#[doc = "Reader of field `IWDG_SW`"]
pub type IWDG_SW_R = crate::R<bool, bool>;
#[doc = "Reader of field `nRTS_STOP`"]
pub type NRTS_STOP_R = crate::R<bool, bool>;
#[doc = "Reader of field `nRST_STDBY`"]
pub type NRST_STDBY_R = crate::R<bool, bool>;
#[doc = "Reader of field `BFB2`"]
pub type BFB2_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:7 - Read protection"]
    #[inline(always)]
    pub fn rdprt(&self) -> RDPRT_R {
        RDPRT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - BOR_LEV"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - IWDG_SW"]
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - nRTS_STOP"]
    #[inline(always)]
    pub fn n_rts_stop(&self) -> NRTS_STOP_R {
        NRTS_STOP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Boot From Bank 2"]
    #[inline(always)]
    pub fn bfb2(&self) -> BFB2_R {
        BFB2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
