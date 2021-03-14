#[doc = "Reader of register JPEG_SR"]
pub type R = crate::R<u32, super::JPEG_SR>;
#[doc = "Reader of field `IFTF`"]
pub type IFTF_R = crate::R<bool, bool>;
#[doc = "Reader of field `IFNFF`"]
pub type IFNFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `OFTF`"]
pub type OFTF_R = crate::R<bool, bool>;
#[doc = "Reader of field `OFNEF`"]
pub type OFNEF_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOCF`"]
pub type EOCF_R = crate::R<bool, bool>;
#[doc = "Reader of field `HPDF`"]
pub type HPDF_R = crate::R<bool, bool>;
#[doc = "Reader of field `COF`"]
pub type COF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - Input FIFO Threshold Flag"]
    #[inline(always)]
    pub fn iftf(&self) -> IFTF_R {
        IFTF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Input FIFO Not Full Flag"]
    #[inline(always)]
    pub fn ifnff(&self) -> IFNFF_R {
        IFNFF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output FIFO Threshold Flag"]
    #[inline(always)]
    pub fn oftf(&self) -> OFTF_R {
        OFTF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Output FIFO Not Empty Flag"]
    #[inline(always)]
    pub fn ofnef(&self) -> OFNEF_R {
        OFNEF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End of Conversion Flag"]
    #[inline(always)]
    pub fn eocf(&self) -> EOCF_R {
        EOCF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Header Parsing Done Flag"]
    #[inline(always)]
    pub fn hpdf(&self) -> HPDF_R {
        HPDF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Codec Operation Flag"]
    #[inline(always)]
    pub fn cof(&self) -> COF_R {
        COF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
