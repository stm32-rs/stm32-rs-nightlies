#[doc = "Register `DDRPHYC_RIDR` reader"]
pub type R = crate::R<DDRPHYC_RIDRrs>;
#[doc = "Field `PUBMNR` reader - PUBMNR"]
pub type PUBMNR_R = crate::FieldReader;
#[doc = "Field `PUBMDR` reader - PUBMDR"]
pub type PUBMDR_R = crate::FieldReader;
#[doc = "Field `PUBMJR` reader - PUBMJR"]
pub type PUBMJR_R = crate::FieldReader;
#[doc = "Field `PHYMNR` reader - PHYMNR"]
pub type PHYMNR_R = crate::FieldReader;
#[doc = "Field `PHYMDR` reader - PHYMDR"]
pub type PHYMDR_R = crate::FieldReader;
#[doc = "Field `PHYMJR` reader - PHYMJR"]
pub type PHYMJR_R = crate::FieldReader;
#[doc = "Field `UDRID` reader - UDRID"]
pub type UDRID_R = crate::FieldReader;
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
#[doc = "DDRPHYC revision ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_ridr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_RIDRrs;
impl crate::RegisterSpec for DDRPHYC_RIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_ridr::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_RIDRrs {}
#[doc = "`reset()` method sets DDRPHYC_RIDR to value 0x0041_0010"]
impl crate::Resettable for DDRPHYC_RIDRrs {
    const RESET_VALUE: u32 = 0x0041_0010;
}
