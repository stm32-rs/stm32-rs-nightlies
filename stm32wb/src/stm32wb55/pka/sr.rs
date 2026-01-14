///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `BUSY` reader - PKA Operation in progress
pub type BUSY_R = crate::BitReader;
///Field `PROCENDF` reader - PKA End of Operation flag
pub type PROCENDF_R = crate::BitReader;
///Field `RAMERRF` reader - RAM error flag
pub type RAMERRF_R = crate::BitReader;
///Field `ADDRERRF` reader - Address error flag
pub type ADDRERRF_R = crate::BitReader;
impl R {
    ///Bit 16 - PKA Operation in progress
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PKA End of Operation flag
    #[inline(always)]
    pub fn procendf(&self) -> PROCENDF_R {
        PROCENDF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - RAM error flag
    #[inline(always)]
    pub fn ramerrf(&self) -> RAMERRF_R {
        RAMERRF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Address error flag
    #[inline(always)]
    pub fn addrerrf(&self) -> ADDRERRF_R {
        ADDRERRF_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("addrerrf", &self.addrerrf())
            .field("ramerrf", &self.ramerrf())
            .field("procendf", &self.procendf())
            .field("busy", &self.busy())
            .finish()
    }
}
/**PKA status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#PKA:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
