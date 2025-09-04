///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `INITOK` reader - PKA initialization OK
pub type INITOK_R = crate::BitReader;
///Field `LMF` reader - Limited mode flag
pub type LMF_R = crate::BitReader;
///Field `BUSY` reader - PKA operation is in progress
pub type BUSY_R = crate::BitReader;
///Field `PROCENDF` reader - PKA End of Operation flag
pub type PROCENDF_R = crate::BitReader;
///Field `RAMERRF` reader - PKA RAM error flag
pub type RAMERRF_R = crate::BitReader;
///Field `ADDRERRF` reader - Address error flag
pub type ADDRERRF_R = crate::BitReader;
///Field `OPERRF` reader - Operation error flag
pub type OPERRF_R = crate::BitReader;
impl R {
    ///Bit 0 - PKA initialization OK
    #[inline(always)]
    pub fn initok(&self) -> INITOK_R {
        INITOK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Limited mode flag
    #[inline(always)]
    pub fn lmf(&self) -> LMF_R {
        LMF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 16 - PKA operation is in progress
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PKA End of Operation flag
    #[inline(always)]
    pub fn procendf(&self) -> PROCENDF_R {
        PROCENDF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - PKA RAM error flag
    #[inline(always)]
    pub fn ramerrf(&self) -> RAMERRF_R {
        RAMERRF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Address error flag
    #[inline(always)]
    pub fn addrerrf(&self) -> ADDRERRF_R {
        ADDRERRF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Operation error flag
    #[inline(always)]
    pub fn operrf(&self) -> OPERRF_R {
        OPERRF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("initok", &self.initok())
            .field("lmf", &self.lmf())
            .field("busy", &self.busy())
            .field("procendf", &self.procendf())
            .field("ramerrf", &self.ramerrf())
            .field("addrerrf", &self.addrerrf())
            .field("operrf", &self.operrf())
            .finish()
    }
}
/**PKA status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#PKA:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
