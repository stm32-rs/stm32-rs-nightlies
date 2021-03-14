#[doc = "Reader of register SR1"]
pub type R = crate::R<u32, super::SR1>;
#[doc = "Reader of field `WUFI`"]
pub type WUFI_R = crate::R<bool, bool>;
#[doc = "Reader of field `CSBF`"]
pub type CSBF_R = crate::R<bool, bool>;
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
    #[doc = "Bit 15 - Wakeup flag internal"]
    #[inline(always)]
    pub fn wufi(&self) -> WUFI_R {
        WUFI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Standby flag"]
    #[inline(always)]
    pub fn csbf(&self) -> CSBF_R {
        CSBF_R::new(((self.bits >> 8) & 0x01) != 0)
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
