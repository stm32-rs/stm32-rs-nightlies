#[doc = "Reader of register SR1"]
pub type R = crate::R<u32, super::SR1>;
#[doc = "Reader of field `WUFI`"]
pub type WUFI_R = crate::R<bool, bool>;
#[doc = "Reader of field `C2HF`"]
pub type C2HF_R = crate::R<bool, bool>;
#[doc = "Reader of field `AF802`"]
pub type AF802_R = crate::R<bool, bool>;
#[doc = "Reader of field `BLEAF`"]
pub type BLEAF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRPEF`"]
pub type CRPEF_R = crate::R<bool, bool>;
#[doc = "Reader of field `_802WUF`"]
pub type _802WUF_R = crate::R<bool, bool>;
#[doc = "Reader of field `BLEWUF`"]
pub type BLEWUF_R = crate::R<bool, bool>;
#[doc = "Reader of field `BORHF`"]
pub type BORHF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SDFBF`"]
pub type SDFBF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CWUF5`"]
pub type CWUF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `CWUF4`"]
pub type CWUF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CWUF3`"]
pub type CWUF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CWUF2`"]
pub type CWUF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CWUF1`"]
pub type CWUF1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 15 - Internal Wakeup interrupt flag"]
    #[inline(always)]
    pub fn wufi(&self) -> WUFI_R {
        WUFI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CPU2 Hold interrupt flag"]
    #[inline(always)]
    pub fn c2hf(&self) -> C2HF_R {
        C2HF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 802.15.4 end of activity interrupt flag"]
    #[inline(always)]
    pub fn af802(&self) -> AF802_R {
        AF802_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - BLE end of activity interrupt flag"]
    #[inline(always)]
    pub fn bleaf(&self) -> BLEAF_R {
        BLEAF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable critical radio phase end of activity interrupt flag"]
    #[inline(always)]
    pub fn crpef(&self) -> CRPEF_R {
        CRPEF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 802.15.4 wakeup interrupt flag"]
    #[inline(always)]
    pub fn _802wuf(&self) -> _802WUF_R {
        _802WUF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BLE wakeup interrupt flag"]
    #[inline(always)]
    pub fn blewuf(&self) -> BLEWUF_R {
        BLEWUF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BORH interrupt flag"]
    #[inline(always)]
    pub fn borhf(&self) -> BORHF_R {
        BORHF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Step Down converter forced in Bypass interrupt flag"]
    #[inline(always)]
    pub fn sdfbf(&self) -> SDFBF_R {
        SDFBF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wakeup flag 5"]
    #[inline(always)]
    pub fn cwuf5(&self) -> CWUF5_R {
        CWUF5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wakeup flag 4"]
    #[inline(always)]
    pub fn cwuf4(&self) -> CWUF4_R {
        CWUF4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup flag 3"]
    #[inline(always)]
    pub fn cwuf3(&self) -> CWUF3_R {
        CWUF3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup flag 2"]
    #[inline(always)]
    pub fn cwuf2(&self) -> CWUF2_R {
        CWUF2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Wakeup flag 1"]
    #[inline(always)]
    pub fn cwuf1(&self) -> CWUF1_R {
        CWUF1_R::new((self.bits & 0x01) != 0)
    }
}
