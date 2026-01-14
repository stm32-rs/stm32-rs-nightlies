///Register `IASR` reader
pub type R = crate::R<IASRrs>;
///Field `CAEF` reader - Configuration access error flag This bit is set when an illegal access to any MCE configuration register is detected. Bit is cleared by setting corresponding bit in MCE_IACR register. No additional details on the error is available.
pub type CAEF_R = crate::BitReader;
///Field `IAEF` reader - Illegal access error flag This bit is set when an illegal access is detected on the system bus. More details on the error can be found in MCE_IAESR and MCE_IADDR registers. This bit is cleared by setting corresponding bit in MCE_IACR register.
pub type IAEF_R = crate::BitReader;
impl R {
    ///Bit 0 - Configuration access error flag This bit is set when an illegal access to any MCE configuration register is detected. Bit is cleared by setting corresponding bit in MCE_IACR register. No additional details on the error is available.
    #[inline(always)]
    pub fn caef(&self) -> CAEF_R {
        CAEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Illegal access error flag This bit is set when an illegal access is detected on the system bus. More details on the error can be found in MCE_IAESR and MCE_IADDR registers. This bit is cleared by setting corresponding bit in MCE_IACR register.
    #[inline(always)]
    pub fn iaef(&self) -> IAEF_R {
        IAEF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IASR")
            .field("caef", &self.caef())
            .field("iaef", &self.iaef())
            .finish()
    }
}
/**MCE illegal access status register

You can [`read`](crate::Reg::read) this register and get [`iasr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#MCE1:IASR)*/
pub struct IASRrs;
impl crate::RegisterSpec for IASRrs {
    type Ux = u32;
}
///`read()` method returns [`iasr::R`](R) reader structure
impl crate::Readable for IASRrs {}
///`reset()` method sets IASR to value 0
impl crate::Resettable for IASRrs {}
