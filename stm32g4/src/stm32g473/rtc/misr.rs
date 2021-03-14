#[doc = "Reader of register MISR"]
pub type R = crate::R<u32, super::MISR>;
#[doc = "Reader of field `ALRAMF`"]
pub type ALRAMF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ALRBMF`"]
pub type ALRBMF_R = crate::R<bool, bool>;
#[doc = "Reader of field `WUTMF`"]
pub type WUTMF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSMF`"]
pub type TSMF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSOVMF`"]
pub type TSOVMF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ITSMF`"]
pub type ITSMF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ALRAMF"]
    #[inline(always)]
    pub fn alramf(&self) -> ALRAMF_R {
        ALRAMF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ALRBMF"]
    #[inline(always)]
    pub fn alrbmf(&self) -> ALRBMF_R {
        ALRBMF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WUTMF"]
    #[inline(always)]
    pub fn wutmf(&self) -> WUTMF_R {
        WUTMF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TSMF"]
    #[inline(always)]
    pub fn tsmf(&self) -> TSMF_R {
        TSMF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TSOVMF"]
    #[inline(always)]
    pub fn tsovmf(&self) -> TSOVMF_R {
        TSOVMF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ITSMF"]
    #[inline(always)]
    pub fn itsmf(&self) -> ITSMF_R {
        ITSMF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
