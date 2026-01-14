///Register `RIDR` reader
pub type R = crate::R<RIDRrs>;
///Field `PUBMNR` reader - PUBMNR
pub type PUBMNR_R = crate::FieldReader;
///Field `PUBMDR` reader - PUBMDR
pub type PUBMDR_R = crate::FieldReader;
///Field `PUBMJR` reader - PUBMJR
pub type PUBMJR_R = crate::FieldReader;
///Field `PHYMNR` reader - PHYMNR
pub type PHYMNR_R = crate::FieldReader;
///Field `PHYMDR` reader - PHYMDR
pub type PHYMDR_R = crate::FieldReader;
///Field `PHYMJR` reader - PHYMJR
pub type PHYMJR_R = crate::FieldReader;
///Field `UDRID` reader - UDRID
pub type UDRID_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - PUBMNR
    #[inline(always)]
    pub fn pubmnr(&self) -> PUBMNR_R {
        PUBMNR_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - PUBMDR
    #[inline(always)]
    pub fn pubmdr(&self) -> PUBMDR_R {
        PUBMDR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - PUBMJR
    #[inline(always)]
    pub fn pubmjr(&self) -> PUBMJR_R {
        PUBMJR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - PHYMNR
    #[inline(always)]
    pub fn phymnr(&self) -> PHYMNR_R {
        PHYMNR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - PHYMDR
    #[inline(always)]
    pub fn phymdr(&self) -> PHYMDR_R {
        PHYMDR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - PHYMJR
    #[inline(always)]
    pub fn phymjr(&self) -> PHYMJR_R {
        PHYMJR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:31 - UDRID
    #[inline(always)]
    pub fn udrid(&self) -> UDRID_R {
        UDRID_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIDR")
            .field("pubmnr", &self.pubmnr())
            .field("pubmdr", &self.pubmdr())
            .field("pubmjr", &self.pubmjr())
            .field("phymnr", &self.phymnr())
            .field("phymdr", &self.phymdr())
            .field("phymjr", &self.phymjr())
            .field("udrid", &self.udrid())
            .finish()
    }
}
/**DDRPHYC revision ID register

You can [`read`](crate::Reg::read) this register and get [`ridr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRPHYC:RIDR)*/
pub struct RIDRrs;
impl crate::RegisterSpec for RIDRrs {
    type Ux = u32;
}
///`read()` method returns [`ridr::R`](R) reader structure
impl crate::Readable for RIDRrs {}
///`reset()` method sets RIDR to value 0x0041_0010
impl crate::Resettable for RIDRrs {
    const RESET_VALUE: u32 = 0x0041_0010;
}
