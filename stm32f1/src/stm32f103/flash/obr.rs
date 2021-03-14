#[doc = "Reader of register OBR"]
pub type R = crate::R<u32, super::OBR>;
#[doc = "Reader of field `OPTERR`"]
pub type OPTERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RDPRT`"]
pub type RDPRT_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDG_SW`"]
pub type WDG_SW_R = crate::R<bool, bool>;
#[doc = "Reader of field `nRST_STOP`"]
pub type NRST_STOP_R = crate::R<bool, bool>;
#[doc = "Reader of field `nRST_STDBY`"]
pub type NRST_STDBY_R = crate::R<bool, bool>;
#[doc = "Reader of field `Data0`"]
pub type DATA0_R = crate::R<u8, u8>;
#[doc = "Reader of field `Data1`"]
pub type DATA1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Option byte error"]
    #[inline(always)]
    pub fn opterr(&self) -> OPTERR_R {
        OPTERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read protection"]
    #[inline(always)]
    pub fn rdprt(&self) -> RDPRT_R {
        RDPRT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WDG_SW"]
    #[inline(always)]
    pub fn wdg_sw(&self) -> WDG_SW_R {
        WDG_SW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 10:17 - Data0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 18:25 - Data1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 18) & 0xff) as u8)
    }
}
