///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `PERF` reader - PERF
pub type PERF_R = crate::BitReader;
///Field `SERF` reader - SERF
pub type SERF_R = crate::BitReader;
///Field `TERF` reader - TERF
pub type TERF_R = crate::BitReader;
impl R {
    ///Bit 0 - PERF
    #[inline(always)]
    pub fn perf(&self) -> PERF_R {
        PERF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SERF
    #[inline(always)]
    pub fn serf(&self) -> SERF_R {
        SERF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TERF
    #[inline(always)]
    pub fn terf(&self) -> TERF_R {
        TERF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("perf", &self.perf())
            .field("serf", &self.serf())
            .field("terf", &self.terf())
            .finish()
    }
}
/**MDIOS status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDIOS:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
