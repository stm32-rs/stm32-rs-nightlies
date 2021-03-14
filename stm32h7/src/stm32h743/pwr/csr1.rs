#[doc = "Reader of register CSR1"]
pub type R = crate::R<u32, super::CSR1>;
#[doc = "Reader of field `PVDO`"]
pub type PVDO_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTVOSRDY`"]
pub type ACTVOSRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACTVOS`"]
pub type ACTVOS_R = crate::R<u8, u8>;
#[doc = "Reader of field `AVDO`"]
pub type AVDO_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 4 - Programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. Note: since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set."]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Voltage levels ready bit for currently used VOS and SDLEVEL This bit is set to 1 by hardware when the voltage regulator and the SD converter are both disabled and Bypass mode is selected in PWR control register 3 (PWR_CR3)."]
    #[inline(always)]
    pub fn actvosrdy(&self) -> ACTVOSRDY_R {
        ACTVOSRDY_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - VOS currently applied for VCORE voltage scaling selection. These bits reflect the last VOS value applied to the PMU."]
    #[inline(always)]
    pub fn actvos(&self) -> ACTVOS_R {
        ACTVOS_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Analog voltage detector output on VDDA This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the AVDEN bit is set."]
    #[inline(always)]
    pub fn avdo(&self) -> AVDO_R {
        AVDO_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
