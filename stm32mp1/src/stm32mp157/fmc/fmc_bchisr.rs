#[doc = "Reader of register FMC_BCHISR"]
pub type R = crate::R<u32, super::FMC_BCHISR>;
#[doc = "Reader of field `DUEF`"]
pub type DUEF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DERF`"]
pub type DERF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DEFF`"]
pub type DEFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DSRF`"]
pub type DSRF_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPBRF`"]
pub type EPBRF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - DUEF"]
    #[inline(always)]
    pub fn duef(&self) -> DUEF_R {
        DUEF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DERF"]
    #[inline(always)]
    pub fn derf(&self) -> DERF_R {
        DERF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DEFF"]
    #[inline(always)]
    pub fn deff(&self) -> DEFF_R {
        DEFF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DSRF"]
    #[inline(always)]
    pub fn dsrf(&self) -> DSRF_R {
        DSRF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EPBRF"]
    #[inline(always)]
    pub fn epbrf(&self) -> EPBRF_R {
        EPBRF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
