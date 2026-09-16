///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `BUSY` reader - PKA operation is in progress This bit is set to '1' whenever START bit in the PKA_CR is set. It is automatically cleared when the computation is complete, meaning that PKA RAM can be safely accessed and a new operation can be started. 0: No operation is in progress (default) 1: An operation is in progress Nota: if PKA is started with a wrong opcode the IP will be busy for a couple of cycles then it will abort automatically the operation and go back to ready (BUSY bit is set to '0').
pub type BUSY_R = crate::BitReader;
///Field `PROCENDF` reader - PKA End of Operation flag 0: Operation in progress 1: PKA operation is completed. This flag is set when the BUSY bit is de-asserted.
pub type PROCENDF_R = crate::BitReader;
///Field `RAMERRF` reader - PKA RAM error flag 0: No PKA RAM access error 1: An AHB access to the PKA RAM occured while the PKA core was computing and using its internal RAM (AHB PKA_RAM access are not allowed while PKA operation is in progress).
pub type RAMERRF_R = crate::BitReader;
///Field `ADDRERRF` reader - Address error flag 0: No Address error 1: Address access is out of range (unmapped address)
pub type ADDRERRF_R = crate::BitReader;
///Field `FAULTFSMF` reader - Fault fsm error flag 0: No fault has been detected 1: A fault on fsm has been detected
pub type FAULTFSMF_R = crate::BitReader;
///Field `FAULTERRORCODEF` reader - Fault error code error flag 0: No fault has been detected 1: A fault has altered the execution of the operation and the internal fault check has been skipped
pub type FAULTERRORCODEF_R = crate::BitReader;
impl R {
    ///Bit 16 - PKA operation is in progress This bit is set to '1' whenever START bit in the PKA_CR is set. It is automatically cleared when the computation is complete, meaning that PKA RAM can be safely accessed and a new operation can be started. 0: No operation is in progress (default) 1: An operation is in progress Nota: if PKA is started with a wrong opcode the IP will be busy for a couple of cycles then it will abort automatically the operation and go back to ready (BUSY bit is set to '0').
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PKA End of Operation flag 0: Operation in progress 1: PKA operation is completed. This flag is set when the BUSY bit is de-asserted.
    #[inline(always)]
    pub fn procendf(&self) -> PROCENDF_R {
        PROCENDF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - PKA RAM error flag 0: No PKA RAM access error 1: An AHB access to the PKA RAM occured while the PKA core was computing and using its internal RAM (AHB PKA_RAM access are not allowed while PKA operation is in progress).
    #[inline(always)]
    pub fn ramerrf(&self) -> RAMERRF_R {
        RAMERRF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Address error flag 0: No Address error 1: Address access is out of range (unmapped address)
    #[inline(always)]
    pub fn addrerrf(&self) -> ADDRERRF_R {
        ADDRERRF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - Fault fsm error flag 0: No fault has been detected 1: A fault on fsm has been detected
    #[inline(always)]
    pub fn faultfsmf(&self) -> FAULTFSMF_R {
        FAULTFSMF_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Fault error code error flag 0: No fault has been detected 1: A fault has altered the execution of the operation and the internal fault check has been skipped
    #[inline(always)]
    pub fn faulterrorcodef(&self) -> FAULTERRORCODEF_R {
        FAULTERRORCODEF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("busy", &self.busy())
            .field("procendf", &self.procendf())
            .field("ramerrf", &self.ramerrf())
            .field("addrerrf", &self.addrerrf())
            .field("faultfsmf", &self.faultfsmf())
            .field("faulterrorcodef", &self.faulterrorcodef())
            .finish()
    }
}
/**PKA_SR register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PKA:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
