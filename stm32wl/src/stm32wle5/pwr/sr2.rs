#[doc = "Reader of register SR2"]
pub type R = crate::R<u32, super::SR2>;
#[doc = "Reader of field `PVMO3`"]
pub type PVMO3_R = crate::R<bool, bool>;
#[doc = "Reader of field `PVDO`"]
pub type PVDO_R = crate::R<bool, bool>;
#[doc = "Reader of field `VOSF`"]
pub type VOSF_R = crate::R<bool, bool>;
#[doc = "Reader of field `REGLPF`"]
pub type REGLPF_R = crate::R<bool, bool>;
#[doc = "Reader of field `REGLPS`"]
pub type REGLPS_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLASHRDY`"]
pub type FLASHRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `REGMRS`"]
pub type REGMRS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RFEOLF`"]
pub type RFEOLF_R = crate::R<bool, bool>;
#[doc = "Reader of field `LDORDY`"]
pub type LDORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `SMPSRDY`"]
pub type SMPSRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RFBUSYMS`"]
pub type RFBUSYMS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RFBUSYS`"]
pub type RFBUSYS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 14 - Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
    #[inline(always)]
    pub fn pvmo3(&self) -> PVMO3_R {
        PVMO3_R::new(((self.bits >> 14) & 0x01) != 0)
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
    #[doc = "Bit 9 - regulator1 low power flag"]
    #[inline(always)]
    pub fn reglpf(&self) -> REGLPF_R {
        REGLPF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - regulator1 started"]
    #[inline(always)]
    pub fn reglps(&self) -> REGLPS_R {
        REGLPS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Flash ready"]
    #[inline(always)]
    pub fn flashrdy(&self) -> FLASHRDY_R {
        FLASHRDY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - regulator2 low power flag"]
    #[inline(always)]
    pub fn regmrs(&self) -> REGMRS_R {
        REGMRS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Radio end of life flag"]
    #[inline(always)]
    pub fn rfeolf(&self) -> RFEOLF_R {
        RFEOLF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LDO ready flag"]
    #[inline(always)]
    pub fn ldordy(&self) -> LDORDY_R {
        LDORDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SMPS ready flag"]
    #[inline(always)]
    pub fn smpsrdy(&self) -> SMPSRDY_R {
        SMPSRDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Radio BUSY masked signal status"]
    #[inline(always)]
    pub fn rfbusyms(&self) -> RFBUSYMS_R {
        RFBUSYMS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Radio BUSY signal status"]
    #[inline(always)]
    pub fn rfbusys(&self) -> RFBUSYS_R {
        RFBUSYS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
