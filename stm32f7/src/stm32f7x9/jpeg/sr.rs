#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Field `IFTF` reader - Input FIFO Threshold Flag"]
pub type IFTF_R = crate::BitReader;
#[doc = "Field `IFNFF` reader - Input FIFO Not Full Flag"]
pub type IFNFF_R = crate::BitReader;
#[doc = "Field `OFTF` reader - Output FIFO Threshold Flag"]
pub type OFTF_R = crate::BitReader;
#[doc = "Field `OFNEF` reader - Output FIFO Not Empty Flag"]
pub type OFNEF_R = crate::BitReader;
#[doc = "Field `EOCF` reader - End of Conversion Flag"]
pub type EOCF_R = crate::BitReader;
#[doc = "Field `HPDF` reader - Header Parsing Done Flag"]
pub type HPDF_R = crate::BitReader;
#[doc = "Field `COF` reader - Codec Operation Flag"]
pub type COF_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Input FIFO Threshold Flag"]
    #[inline(always)]
    pub fn iftf(&self) -> IFTF_R {
        IFTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input FIFO Not Full Flag"]
    #[inline(always)]
    pub fn ifnff(&self) -> IFNFF_R {
        IFNFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output FIFO Threshold Flag"]
    #[inline(always)]
    pub fn oftf(&self) -> OFTF_R {
        OFTF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output FIFO Not Empty Flag"]
    #[inline(always)]
    pub fn ofnef(&self) -> OFNEF_R {
        OFNEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Conversion Flag"]
    #[inline(always)]
    pub fn eocf(&self) -> EOCF_R {
        EOCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Header Parsing Done Flag"]
    #[inline(always)]
    pub fn hpdf(&self) -> HPDF_R {
        HPDF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Codec Operation Flag"]
    #[inline(always)]
    pub fn cof(&self) -> COF_R {
        COF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "JPEG status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
