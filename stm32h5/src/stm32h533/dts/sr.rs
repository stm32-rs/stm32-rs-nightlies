///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `TS1_ITEF` reader - Interrupt flag for end of measurement on temperature sensor 1, synchronized on PCLK.
pub type TS1_ITEF_R = crate::BitReader;
///Field `TS1_ITLF` reader - Interrupt flag for low threshold on temperature sensor 1, synchronized on PCLK.
pub type TS1_ITLF_R = crate::BitReader;
///Field `TS1_ITHF` reader - Interrupt flag for high threshold on temperature sensor 1, synchronized on PCLK
pub type TS1_ITHF_R = crate::BitReader;
///Field `TS1_AITEF` reader - Asynchronous interrupt flag for end of measure on temperature sensor 1
pub type TS1_AITEF_R = crate::BitReader;
///Field `TS1_AITLF` reader - Asynchronous interrupt flag for low threshold on temperature sensor 1
pub type TS1_AITLF_R = crate::BitReader;
///Field `TS1_AITHF` reader - Asynchronous interrupt flag for high threshold on temperature sensor 1
pub type TS1_AITHF_R = crate::BitReader;
///Field `TS1_RDY` reader - Temperature sensor 1 ready flag
pub type TS1_RDY_R = crate::BitReader;
impl R {
    ///Bit 0 - Interrupt flag for end of measurement on temperature sensor 1, synchronized on PCLK.
    #[inline(always)]
    pub fn ts1_itef(&self) -> TS1_ITEF_R {
        TS1_ITEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt flag for low threshold on temperature sensor 1, synchronized on PCLK.
    #[inline(always)]
    pub fn ts1_itlf(&self) -> TS1_ITLF_R {
        TS1_ITLF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt flag for high threshold on temperature sensor 1, synchronized on PCLK
    #[inline(always)]
    pub fn ts1_ithf(&self) -> TS1_ITHF_R {
        TS1_ITHF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Asynchronous interrupt flag for end of measure on temperature sensor 1
    #[inline(always)]
    pub fn ts1_aitef(&self) -> TS1_AITEF_R {
        TS1_AITEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Asynchronous interrupt flag for low threshold on temperature sensor 1
    #[inline(always)]
    pub fn ts1_aitlf(&self) -> TS1_AITLF_R {
        TS1_AITLF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Asynchronous interrupt flag for high threshold on temperature sensor 1
    #[inline(always)]
    pub fn ts1_aithf(&self) -> TS1_AITHF_R {
        TS1_AITHF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 15 - Temperature sensor 1 ready flag
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#DTS:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
