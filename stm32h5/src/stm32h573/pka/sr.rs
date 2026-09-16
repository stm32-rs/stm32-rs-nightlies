///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `INITOK` reader - PKA initialization OK This bit is asserted when PKA initialization is complete. When RNG is not able to output proper random numbers INITOK stays at 0.
pub type INITOK_R = crate::BitReader;
///Field `LMF` reader - Limited mode flag This bit is updated when EN bit in PKA_CR is set
pub type LMF_R = crate::BitReader;
///Field `BUSY` reader - PKA operation is in progress This bit is set to 1 whenever START bit in the PKA_CR is set. It is automatically cleared when the computation is complete, meaning that PKA RAM can be safely accessed and a new operation can be started. If PKA is started with a wrong opcode, it is busy for a couple of cycles, then it aborts automatically the operation and go back to ready (BUSY bit is set to 0).
pub type BUSY_R = crate::BitReader;
///Field `PROCENDF` reader - PKA End of Operation flag
pub type PROCENDF_R = crate::BitReader;
///Field `RAMERRF` reader - PKA RAM error flag This bit is cleared using RAMERRFC bit in PKA_CLRFR.
pub type RAMERRF_R = crate::BitReader;
///Field `ADDRERRF` reader - Address error flag This bit is cleared using ADDRERRFC bit in PKA_CLRFR.
pub type ADDRERRF_R = crate::BitReader;
///Field `OPERRF` reader - Operation error flag This bit is cleared using OPERRFC bit in PKA_CLRFR.
pub type OPERRF_R = crate::BitReader;
impl R {
    ///Bit 0 - PKA initialization OK This bit is asserted when PKA initialization is complete. When RNG is not able to output proper random numbers INITOK stays at 0.
    #[inline(always)]
    pub fn initok(&self) -> INITOK_R {
        INITOK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Limited mode flag This bit is updated when EN bit in PKA_CR is set
    #[inline(always)]
    pub fn lmf(&self) -> LMF_R {
        LMF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 16 - PKA operation is in progress This bit is set to 1 whenever START bit in the PKA_CR is set. It is automatically cleared when the computation is complete, meaning that PKA RAM can be safely accessed and a new operation can be started. If PKA is started with a wrong opcode, it is busy for a couple of cycles, then it aborts automatically the operation and go back to ready (BUSY bit is set to 0).
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PKA End of Operation flag
    #[inline(always)]
    pub fn procendf(&self) -> PROCENDF_R {
        PROCENDF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - PKA RAM error flag This bit is cleared using RAMERRFC bit in PKA_CLRFR.
    #[inline(always)]
    pub fn ramerrf(&self) -> RAMERRF_R {
        RAMERRF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Address error flag This bit is cleared using ADDRERRFC bit in PKA_CLRFR.
    #[inline(always)]
    pub fn addrerrf(&self) -> ADDRERRF_R {
        ADDRERRF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Operation error flag This bit is cleared using OPERRFC bit in PKA_CLRFR.
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#PKA:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
