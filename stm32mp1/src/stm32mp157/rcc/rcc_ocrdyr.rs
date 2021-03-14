#[doc = "Reader of register RCC_OCRDYR"]
pub type R = crate::R<u32, super::RCC_OCRDYR>;
#[doc = "Reader of field `HSIRDY`"]
pub type HSIRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSIDIVRDY`"]
pub type HSIDIVRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `CSIRDY`"]
pub type CSIRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSERDY`"]
pub type HSERDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `MPUCKRDY`"]
pub type MPUCKRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `AXICKRDY`"]
pub type AXICKRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `CKREST`"]
pub type CKREST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - HSIRDY"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSIDIVRDY"]
    #[inline(always)]
    pub fn hsidivrdy(&self) -> HSIDIVRDY_R {
        HSIDIVRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CSIRDY"]
    #[inline(always)]
    pub fn csirdy(&self) -> CSIRDY_R {
        CSIRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HSERDY"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 23 - MPUCKRDY"]
    #[inline(always)]
    pub fn mpuckrdy(&self) -> MPUCKRDY_R {
        MPUCKRDY_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - AXICKRDY"]
    #[inline(always)]
    pub fn axickrdy(&self) -> AXICKRDY_R {
        AXICKRDY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CKREST"]
    #[inline(always)]
    pub fn ckrest(&self) -> CKREST_R {
        CKREST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
