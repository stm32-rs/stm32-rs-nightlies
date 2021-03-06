#[doc = "Reader of register TAMP_MISR"]
pub type R = crate::R<u32, super::TAMP_MISR>;
#[doc = "Reader of field `TAMP1MF`"]
pub type TAMP1MF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TAMP2MF`"]
pub type TAMP2MF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TAMP3MF`"]
pub type TAMP3MF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ITAMP1MF`"]
pub type ITAMP1MF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ITAMP2MF`"]
pub type ITAMP2MF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ITAMP3MF`"]
pub type ITAMP3MF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ITAMP4MF`"]
pub type ITAMP4MF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ITAMP5MF`"]
pub type ITAMP5MF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ITAMP8MF`"]
pub type ITAMP8MF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TAMP1MF"]
    #[inline(always)]
    pub fn tamp1mf(&self) -> TAMP1MF_R {
        TAMP1MF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TAMP2MF"]
    #[inline(always)]
    pub fn tamp2mf(&self) -> TAMP2MF_R {
        TAMP2MF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TAMP3MF"]
    #[inline(always)]
    pub fn tamp3mf(&self) -> TAMP3MF_R {
        TAMP3MF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ITAMP1MF"]
    #[inline(always)]
    pub fn itamp1mf(&self) -> ITAMP1MF_R {
        ITAMP1MF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ITAMP2MF"]
    #[inline(always)]
    pub fn itamp2mf(&self) -> ITAMP2MF_R {
        ITAMP2MF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ITAMP3MF"]
    #[inline(always)]
    pub fn itamp3mf(&self) -> ITAMP3MF_R {
        ITAMP3MF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ITAMP4MF"]
    #[inline(always)]
    pub fn itamp4mf(&self) -> ITAMP4MF_R {
        ITAMP4MF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ITAMP5MF"]
    #[inline(always)]
    pub fn itamp5mf(&self) -> ITAMP5MF_R {
        ITAMP5MF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ITAMP8MF"]
    #[inline(always)]
    pub fn itamp8mf(&self) -> ITAMP8MF_R {
        ITAMP8MF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
