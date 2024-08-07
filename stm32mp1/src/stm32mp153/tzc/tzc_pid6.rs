///Register `TZC_PID6` reader
pub type R = crate::R<TZC_PID6rs>;
///Field `PER_ID_6` reader - PER_ID_6
pub type PER_ID_6_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - PER_ID_6
    #[inline(always)]
    pub fn per_id_6(&self) -> PER_ID_6_R {
        PER_ID_6_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZC_PID6")
            .field("per_id_6", &self.per_id_6())
            .finish()
    }
}
/**Peripheral ID 6.

You can [`read`](crate::Reg::read) this register and get [`tzc_pid6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TZC:TZC_PID6)*/
pub struct TZC_PID6rs;
impl crate::RegisterSpec for TZC_PID6rs {
    type Ux = u32;
}
///`read()` method returns [`tzc_pid6::R`](R) reader structure
impl crate::Readable for TZC_PID6rs {}
///`reset()` method sets TZC_PID6 to value 0
impl crate::Resettable for TZC_PID6rs {
    const RESET_VALUE: u32 = 0;
}
