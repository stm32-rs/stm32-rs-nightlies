#[doc = "Reader of register SR2"]
pub type R = crate::R<u32, super::SR2>;
#[doc = "Reader of field `PVDO`"]
pub type PVDO_R = crate::R<bool, bool>;
#[doc = "Reader of field `VOSF`"]
pub type VOSF_R = crate::R<bool, bool>;
#[doc = "Reader of field `REGLPF`"]
pub type REGLPF_R = crate::R<bool, bool>;
#[doc = "Reader of field `REGLPS`"]
pub type REGLPS_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLASH_RDY`"]
pub type FLASH_RDY_R = crate::R<bool, bool>;
impl R {
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
    #[doc = "Bit 7 - Flash ready flag"]
    #[inline(always)]
    pub fn flash_rdy(&self) -> FLASH_RDY_R {
        FLASH_RDY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
