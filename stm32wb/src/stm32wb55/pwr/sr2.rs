#[doc = "Reader of register SR2"]
pub type R = crate::R<u32, super::SR2>;
#[doc = "Reader of field `PVMO3`"]
pub type PVMO3_R = crate::R<bool, bool>;
#[doc = "Reader of field `PVMO1`"]
pub type PVMO1_R = crate::R<bool, bool>;
#[doc = "Reader of field `PVDO`"]
pub type PVDO_R = crate::R<bool, bool>;
#[doc = "Reader of field `VOSF`"]
pub type VOSF_R = crate::R<bool, bool>;
#[doc = "Reader of field `REGLPF`"]
pub type REGLPF_R = crate::R<bool, bool>;
#[doc = "Reader of field `REGLPS`"]
pub type REGLPS_R = crate::R<bool, bool>;
#[doc = "Reader of field `SDSMPSF`"]
pub type SDSMPSF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SDBF`"]
pub type SDBF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 14 - Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
    #[inline(always)]
    pub fn pvmo3(&self) -> PVMO3_R {
        PVMO3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Peripheral voltage monitoring output: VDDUSB vs. 1.2 V"]
    #[inline(always)]
    pub fn pvmo1(&self) -> PVMO1_R {
        PVMO1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Power voltage detector output"]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Voltage scaling flag"]
    #[inline(always)]
    pub fn vosf(&self) -> VOSF_R {
        VOSF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Low-power regulator flag"]
    #[inline(always)]
    pub fn reglpf(&self) -> REGLPF_R {
        REGLPF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Low-power regulator started"]
    #[inline(always)]
    pub fn reglps(&self) -> REGLPS_R {
        REGLPS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Step Down converter SMPS mode flag"]
    #[inline(always)]
    pub fn sdsmpsf(&self) -> SDSMPSF_R {
        SDSMPSF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Step Down converter Bypass mode flag"]
    #[inline(always)]
    pub fn sdbf(&self) -> SDBF_R {
        SDBF_R::new((self.bits & 0x01) != 0)
    }
}
