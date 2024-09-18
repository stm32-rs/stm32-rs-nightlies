///Register `TZC_PID3` reader
pub type R = crate::R<TZC_PID3rs>;
///Field `PER_ID_3` reader - PER_ID_3
pub type PER_ID_3_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - PER_ID_3
    #[inline(always)]
    pub fn per_id_3(&self) -> PER_ID_3_R {
        PER_ID_3_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZC_PID3")
            .field("per_id_3", &self.per_id_3())
            .finish()
    }
}
/**Peripheral ID 3.

You can [`read`](crate::Reg::read) this register and get [`tzc_pid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TZC:TZC_PID3)*/
pub struct TZC_PID3rs;
impl crate::RegisterSpec for TZC_PID3rs {
    type Ux = u32;
}
///`read()` method returns [`tzc_pid3::R`](R) reader structure
impl crate::Readable for TZC_PID3rs {}
///`reset()` method sets TZC_PID3 to value 0
impl crate::Resettable for TZC_PID3rs {
    const RESET_VALUE: u32 = 0;
}
