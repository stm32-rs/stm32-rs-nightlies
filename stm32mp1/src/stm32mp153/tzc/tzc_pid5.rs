///Register `TZC_PID5` reader
pub type R = crate::R<TZC_PID5rs>;
///Field `PER_ID_5` reader - PER_ID_5
pub type PER_ID_5_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - PER_ID_5
    #[inline(always)]
    pub fn per_id_5(&self) -> PER_ID_5_R {
        PER_ID_5_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZC_PID5")
            .field("per_id_5", &self.per_id_5())
            .finish()
    }
}
/**Peripheral ID 5.

You can [`read`](crate::Reg::read) this register and get [`tzc_pid5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TZC:TZC_PID5)*/
pub struct TZC_PID5rs;
impl crate::RegisterSpec for TZC_PID5rs {
    type Ux = u32;
}
///`read()` method returns [`tzc_pid5::R`](R) reader structure
impl crate::Readable for TZC_PID5rs {}
///`reset()` method sets TZC_PID5 to value 0
impl crate::Resettable for TZC_PID5rs {
    const RESET_VALUE: u32 = 0;
}
