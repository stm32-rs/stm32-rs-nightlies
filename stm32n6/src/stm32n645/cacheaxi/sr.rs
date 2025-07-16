///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `BUSYF` reader - full invalidate busy flag
pub type BUSYF_R = crate::BitReader;
///Field `BSYENDF` reader - full invalidate busy end flag
pub type BSYENDF_R = crate::BitReader;
///Field `ERRF` reader - cache error flag
pub type ERRF_R = crate::BitReader;
///Field `BUSYCMDF` reader - command busy flag
pub type BUSYCMDF_R = crate::BitReader;
///Field `CMDENDF` reader - command end flag
pub type CMDENDF_R = crate::BitReader;
impl R {
    ///Bit 0 - full invalidate busy flag
    #[inline(always)]
    pub fn busyf(&self) -> BUSYF_R {
        BUSYF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - full invalidate busy end flag
    #[inline(always)]
    pub fn bsyendf(&self) -> BSYENDF_R {
        BSYENDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - cache error flag
    #[inline(always)]
    pub fn errf(&self) -> ERRF_R {
        ERRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - command busy flag
    #[inline(always)]
    pub fn busycmdf(&self) -> BUSYCMDF_R {
        BUSYCMDF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - command end flag
    #[inline(always)]
    pub fn cmdendf(&self) -> CMDENDF_R {
        CMDENDF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("busyf", &self.busyf())
            .field("bsyendf", &self.bsyendf())
            .field("errf", &self.errf())
            .field("busycmdf", &self.busycmdf())
            .field("cmdendf", &self.cmdendf())
            .finish()
    }
}
/**CACHEAXI status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#CACHEAXI:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0x01
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x01;
}
