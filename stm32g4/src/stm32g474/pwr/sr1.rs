#[doc = "Reader of register SR1"]
pub type R = crate::R<u32, super::SR1>;
#[doc = "Reader of field `WUFI`"]
pub type WUFI_R = crate::R<bool, bool>;
#[doc = "Reader of field `SBF`"]
pub type SBF_R = crate::R<bool, bool>;
#[doc = "Reader of field `WUF5`"]
pub type WUF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `WUF4`"]
pub type WUF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `WUF3`"]
pub type WUF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `WUF2`"]
pub type WUF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `WUF1`"]
pub type WUF1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 15 - Wakeup flag internal"]
    #[inline(always)]
    pub fn wufi(&self) -> WUFI_R {
        WUFI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Standby flag"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wakeup flag 5"]
    #[inline(always)]
    pub fn wuf5(&self) -> WUF5_R {
        WUF5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wakeup flag 4"]
    #[inline(always)]
    pub fn wuf4(&self) -> WUF4_R {
        WUF4_R::new(((self.bits >> 3) & 0x01) != 0)
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
