#[doc = "Reader of register SR1"]
pub type R = crate::R<u32, super::SR1>;
#[doc = "Reader of field `WUFI`"]
pub type WUFI_R = crate::R<bool, bool>;
#[doc = "Reader of field `C2HF`"]
pub type C2HF_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRFBUSYF`"]
pub type WRFBUSYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `WPVDF`"]
pub type WPVDF_R = crate::R<bool, bool>;
#[doc = "Reader of field `WUF3`"]
pub type WUF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `WUF2`"]
pub type WUF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `WUF1`"]
pub type WUF1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 15 - Internal wakeup interrupt flag"]
    #[inline(always)]
    pub fn wufi(&self) -> WUFI_R {
        WUFI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PU2 Hold interrupt flag"]
    #[inline(always)]
    pub fn c2hf(&self) -> C2HF_R {
        C2HF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Radio BUSY wakeup flag"]
    #[inline(always)]
    pub fn wrfbusyf(&self) -> WRFBUSYF_R {
        WRFBUSYF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Wakeup PVD flag"]
    #[inline(always)]
    pub fn wpvdf(&self) -> WPVDF_R {
        WPVDF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup flag 3"]
    #[inline(always)]
    pub fn wuf3(&self) -> WUF3_R {
        WUF3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup flag 2"]
    #[inline(always)]
    pub fn wuf2(&self) -> WUF2_R {
        WUF2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Wakeup flag 1"]
    #[inline(always)]
    pub fn wuf1(&self) -> WUF1_R {
        WUF1_R::new((self.bits & 0x01) != 0)
    }
}
