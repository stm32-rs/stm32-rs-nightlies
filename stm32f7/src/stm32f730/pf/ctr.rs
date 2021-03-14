#[doc = "Reader of register CTR"]
pub type R = crate::R<u32, super::CTR>;
#[doc = "Reader of field `_IminLine`"]
pub type _IMINLINE_R = crate::R<u8, u8>;
#[doc = "Reader of field `DMinLine`"]
pub type DMINLINE_R = crate::R<u8, u8>;
#[doc = "Reader of field `ERG`"]
pub type ERG_R = crate::R<u8, u8>;
#[doc = "Reader of field `CWG`"]
pub type CWG_R = crate::R<u8, u8>;
#[doc = "Reader of field `Format`"]
pub type FORMAT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - IminLine"]
    #[inline(always)]
    pub fn _imin_line(&self) -> _IMINLINE_R {
        _IMINLINE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DMinLine"]
    #[inline(always)]
    pub fn dmin_line(&self) -> DMINLINE_R {
        DMINLINE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - ERG"]
    #[inline(always)]
    pub fn erg(&self) -> ERG_R {
        ERG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CWG"]
    #[inline(always)]
    pub fn cwg(&self) -> CWG_R {
        CWG_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 29:31 - Format"]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
