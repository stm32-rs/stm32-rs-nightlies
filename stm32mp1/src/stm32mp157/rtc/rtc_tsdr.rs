#[doc = "Reader of register RTC_TSDR"]
pub type R = crate::R<u32, super::RTC_TSDR>;
#[doc = "Reader of field `DU`"]
pub type DU_R = crate::R<u8, u8>;
#[doc = "Reader of field `DT`"]
pub type DT_R = crate::R<u8, u8>;
#[doc = "Reader of field `MU`"]
pub type MU_R = crate::R<u8, u8>;
#[doc = "Reader of field `MT`"]
pub type MT_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDU`"]
pub type WDU_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - DU"]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - DT"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - MU"]
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - MT"]
    #[inline(always)]
    pub fn mt(&self) -> MT_R {
        MT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - WDU"]
    #[inline(always)]
    pub fn wdu(&self) -> WDU_R {
        WDU_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
