#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
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
    #[doc = "Bit 1 - Input FIFO Threshold Flag This bit is set when the input FIFO is not full and is bellow its threshold."]
    #[inline(always)]
    pub fn iftf(&self) -> IFTF_R {
        IFTF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Input FIFO Not Full Flag This bit is set when the input FIFO is not full (a data can be written)."]
    #[inline(always)]
    pub fn ifnff(&self) -> IFNFF_R {
        IFNFF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output FIFO Threshold Flag This bit is set when the output FIFO is not empty and has reach its threshold."]
    #[inline(always)]
    pub fn oftf(&self) -> OFTF_R {
        OFTF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Output FIFO Not Empty Flag This bit is set when the output FIFO is not empty (a data is available)."]
    #[inline(always)]
    pub fn ofnef(&self) -> OFNEF_R {
        OFNEF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End of Conversion Flag This bit is set when the JPEG codec core has finished the encoding or the decoding process and than last data has been sent to the output FIFO."]
    #[inline(always)]
    pub fn eocf(&self) -> EOCF_R {
        EOCF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Header Parsing Done Flag This bit is set in decode mode when the JPEG codec has finished the parsing of the headers and the internal registers have been updated."]
    #[inline(always)]
    pub fn hpdf(&self) -> HPDF_R {
        HPDF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Codec Operation Flag This bit is set when when a JPEG codec operation is on going (encoding or decoding)."]
    #[inline(always)]
    pub fn cof(&self) -> COF_R {
        COF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
