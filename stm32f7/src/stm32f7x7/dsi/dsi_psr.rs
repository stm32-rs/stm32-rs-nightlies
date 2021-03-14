#[doc = "Reader of register DSI_PSR"]
pub type R = crate::R<u32, super::DSI_PSR>;
#[doc = "Reader of field `PD`"]
pub type PD_R = crate::R<bool, bool>;
#[doc = "Reader of field `PSSC`"]
pub type PSSC_R = crate::R<bool, bool>;
#[doc = "Reader of field `UANC`"]
pub type UANC_R = crate::R<bool, bool>;
#[doc = "Reader of field `PSS0`"]
pub type PSS0_R = crate::R<bool, bool>;
#[doc = "Reader of field `UAN0`"]
pub type UAN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `RUE0`"]
pub type RUE0_R = crate::R<bool, bool>;
#[doc = "Reader of field `PSS1`"]
pub type PSS1_R = crate::R<bool, bool>;
#[doc = "Reader of field `UAN1`"]
pub type UAN1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - PHY Direction"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PHY Stop State Clock lane"]
    #[inline(always)]
    pub fn pssc(&self) -> PSSC_R {
        PSSC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ULPS Active Not Clock lane"]
    #[inline(always)]
    pub fn uanc(&self) -> UANC_R {
        UANC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PHY Stop State lane 0"]
    #[inline(always)]
    pub fn pss0(&self) -> PSS0_R {
        PSS0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ULPS Active Not lane 1"]
    #[inline(always)]
    pub fn uan0(&self) -> UAN0_R {
        UAN0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RX ULPS Escape lane 0"]
    #[inline(always)]
    pub fn rue0(&self) -> RUE0_R {
        RUE0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PHY Stop State lane 1"]
    #[inline(always)]
    pub fn pss1(&self) -> PSS1_R {
        PSS1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ULPS Active Not lane 1"]
    #[inline(always)]
    pub fn uan1(&self) -> UAN1_R {
        UAN1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
