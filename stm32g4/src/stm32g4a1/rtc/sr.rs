#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `ALRAF`"]
pub type ALRAF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ALRBF`"]
pub type ALRBF_R = crate::R<bool, bool>;
#[doc = "Reader of field `WUTF`"]
pub type WUTF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSF`"]
pub type TSF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSOVF`"]
pub type TSOVF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ITSF`"]
pub type ITSF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ALRAF"]
    #[inline(always)]
    pub fn alraf(&self) -> ALRAF_R {
        ALRAF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ALRBF"]
    #[inline(always)]
    pub fn alrbf(&self) -> ALRBF_R {
        ALRBF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WUTF"]
    #[inline(always)]
    pub fn wutf(&self) -> WUTF_R {
        WUTF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TSF"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TSOVF"]
    #[inline(always)]
    pub fn tsovf(&self) -> TSOVF_R {
        TSOVF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ITSF"]
    #[inline(always)]
    pub fn itsf(&self) -> ITSF_R {
        ITSF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
