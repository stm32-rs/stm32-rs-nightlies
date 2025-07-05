///Register `BCHISR` reader
pub type R = crate::R<BCHISRrs>;
///Field `DUEF` reader - Decoder Uncorrectable Errors flag
pub type DUEF_R = crate::BitReader;
///Field `DERF` reader - Decoder Error Ready flag
pub type DERF_R = crate::BitReader;
///Field `DEFF` reader - Decoder Error Found flag
pub type DEFF_R = crate::BitReader;
///Field `DSRF` reader - Decoder Syndrome Ready flag
pub type DSRF_R = crate::BitReader;
///Field `EPBRF` reader - Encoder Parity Bits Ready flag
pub type EPBRF_R = crate::BitReader;
impl R {
    ///Bit 0 - Decoder Uncorrectable Errors flag
    #[inline(always)]
    pub fn duef(&self) -> DUEF_R {
        DUEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Decoder Error Ready flag
    #[inline(always)]
    pub fn derf(&self) -> DERF_R {
        DERF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Decoder Error Found flag
    #[inline(always)]
    pub fn deff(&self) -> DEFF_R {
        DEFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Decoder Syndrome Ready flag
    #[inline(always)]
    pub fn dsrf(&self) -> DSRF_R {
        DSRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Encoder Parity Bits Ready flag
    #[inline(always)]
    pub fn epbrf(&self) -> EPBRF_R {
        EPBRF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCHISR")
            .field("duef", &self.duef())
            .field("derf", &self.derf())
            .field("deff", &self.deff())
            .field("dsrf", &self.dsrf())
            .field("epbrf", &self.epbrf())
            .finish()
    }
}
/**FMC BCH interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`bchisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#FMC1:BCHISR)*/
pub struct BCHISRrs;
impl crate::RegisterSpec for BCHISRrs {
    type Ux = u32;
}
///`read()` method returns [`bchisr::R`](R) reader structure
impl crate::Readable for BCHISRrs {}
///`reset()` method sets BCHISR to value 0
impl crate::Resettable for BCHISRrs {}
