#[doc = "Reader of register DDRPHYC_RIDR"]
pub type R = crate::R<u32, super::DDRPHYC_RIDR>;
#[doc = "Reader of field `PUBMNR`"]
pub type PUBMNR_R = crate::R<u8, u8>;
#[doc = "Reader of field `PUBMDR`"]
pub type PUBMDR_R = crate::R<u8, u8>;
#[doc = "Reader of field `PUBMJR`"]
pub type PUBMJR_R = crate::R<u8, u8>;
#[doc = "Reader of field `PHYMNR`"]
pub type PHYMNR_R = crate::R<u8, u8>;
#[doc = "Reader of field `PHYMDR`"]
pub type PHYMDR_R = crate::R<u8, u8>;
#[doc = "Reader of field `PHYMJR`"]
pub type PHYMJR_R = crate::R<u8, u8>;
#[doc = "Reader of field `UDRID`"]
pub type UDRID_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - PUBMNR"]
    #[inline(always)]
    pub fn pubmnr(&self) -> PUBMNR_R {
        PUBMNR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PUBMDR"]
    #[inline(always)]
    pub fn pubmdr(&self) -> PUBMDR_R {
        PUBMDR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PUBMJR"]
    #[inline(always)]
    pub fn pubmjr(&self) -> PUBMJR_R {
        PUBMJR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PHYMNR"]
    #[inline(always)]
    pub fn phymnr(&self) -> PHYMNR_R {
        PHYMNR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PHYMDR"]
    #[inline(always)]
    pub fn phymdr(&self) -> PHYMDR_R {
        PHYMDR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PHYMJR"]
    #[inline(always)]
    pub fn phymjr(&self) -> PHYMJR_R {
        PHYMJR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - UDRID"]
    #[inline(always)]
    pub fn udrid(&self) -> UDRID_R {
        UDRID_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
