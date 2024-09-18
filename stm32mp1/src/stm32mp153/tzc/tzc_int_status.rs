///Register `TZC_INT_STATUS` reader
pub type R = crate::R<TZC_INT_STATUSrs>;
///Field `STATUS` reader - STATUS
pub type STATUS_R = crate::FieldReader;
///Field `OVERRUN` reader - OVERRUN
pub type OVERRUN_R = crate::FieldReader;
///Field `OVERLAP` reader - OVERLAP
pub type OVERLAP_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - STATUS
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 3) as u8)
    }
    ///Bits 8:9 - OVERRUN
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:17 - OVERLAP
    #[inline(always)]
    pub fn overlap(&self) -> OVERLAP_R {
        OVERLAP_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZC_INT_STATUS")
            .field("status", &self.status())
            .field("overrun", &self.overrun())
            .field("overlap", &self.overlap())
            .finish()
    }
}
/**Contains the status of the interrupt signal, TZCINT, that reports access security violations or region overlap errors.

You can [`read`](crate::Reg::read) this register and get [`tzc_int_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TZC:TZC_INT_STATUS)*/
pub struct TZC_INT_STATUSrs;
impl crate::RegisterSpec for TZC_INT_STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`tzc_int_status::R`](R) reader structure
impl crate::Readable for TZC_INT_STATUSrs {}
///`reset()` method sets TZC_INT_STATUS to value 0
impl crate::Resettable for TZC_INT_STATUSrs {
    const RESET_VALUE: u32 = 0;
}
