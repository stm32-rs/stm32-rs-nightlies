///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `TS1_ITEF` reader - TS1_ITEF
pub type TS1_ITEF_R = crate::BitReader;
///Field `TS1_ITLF` reader - TS1_ITLF
pub type TS1_ITLF_R = crate::BitReader;
///Field `TS1_ITHF` reader - TS1_ITHF
pub type TS1_ITHF_R = crate::BitReader;
///Field `TS1_AITEF` reader - TS1_AITEF
pub type TS1_AITEF_R = crate::BitReader;
///Field `TS1_AITLF` reader - TS1_AITLF
pub type TS1_AITLF_R = crate::BitReader;
///Field `TS1_AITHF` reader - TS1_AITHF
pub type TS1_AITHF_R = crate::BitReader;
///Field `TS1_RDY` reader - TS1_RDY
pub type TS1_RDY_R = crate::BitReader;
impl R {
    ///Bit 0 - TS1_ITEF
    #[inline(always)]
    pub fn ts1_itef(&self) -> TS1_ITEF_R {
        TS1_ITEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TS1_ITLF
    #[inline(always)]
    pub fn ts1_itlf(&self) -> TS1_ITLF_R {
        TS1_ITLF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TS1_ITHF
    #[inline(always)]
    pub fn ts1_ithf(&self) -> TS1_ITHF_R {
        TS1_ITHF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - TS1_AITEF
    #[inline(always)]
    pub fn ts1_aitef(&self) -> TS1_AITEF_R {
        TS1_AITEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TS1_AITLF
    #[inline(always)]
    pub fn ts1_aitlf(&self) -> TS1_AITLF_R {
        TS1_AITLF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TS1_AITHF
    #[inline(always)]
    pub fn ts1_aithf(&self) -> TS1_AITHF_R {
        TS1_AITHF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 15 - TS1_RDY
    #[inline(always)]
    pub fn ts1_rdy(&self) -> TS1_RDY_R {
        TS1_RDY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("ts1_itef", &self.ts1_itef())
            .field("ts1_itlf", &self.ts1_itlf())
            .field("ts1_ithf", &self.ts1_ithf())
            .field("ts1_aitef", &self.ts1_aitef())
            .field("ts1_aitlf", &self.ts1_aitlf())
            .field("ts1_aithf", &self.ts1_aithf())
            .field("ts1_rdy", &self.ts1_rdy())
            .finish()
    }
}
/**Temperature sensor status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DTS:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
