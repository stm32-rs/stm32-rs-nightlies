#[doc = "Reader of register DSI_GPSR"]
pub type R = crate::R<u32, super::DSI_GPSR>;
#[doc = "Reader of field `CMDFE`"]
pub type CMDFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMDFF`"]
pub type CMDFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWRFE`"]
pub type PWRFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWRFF`"]
pub type PWRFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PRDFE`"]
pub type PRDFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PRDFF`"]
pub type PRDFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RCB`"]
pub type RCB_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - CMDFE"]
    #[inline(always)]
    pub fn cmdfe(&self) -> CMDFE_R {
        CMDFE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CMDFF"]
    #[inline(always)]
    pub fn cmdff(&self) -> CMDFF_R {
        CMDFF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWRFE"]
    #[inline(always)]
    pub fn pwrfe(&self) -> PWRFE_R {
        PWRFE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWRFF"]
    #[inline(always)]
    pub fn pwrff(&self) -> PWRFF_R {
        PWRFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PRDFE"]
    #[inline(always)]
    pub fn prdfe(&self) -> PRDFE_R {
        PRDFE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PRDFF"]
    #[inline(always)]
    pub fn prdff(&self) -> PRDFF_R {
        PRDFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RCB"]
    #[inline(always)]
    pub fn rcb(&self) -> RCB_R {
        RCB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
