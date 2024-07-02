///Register `TZC_PID2` reader
pub type R = crate::R<TZC_PID2rs>;
///Field `PER_ID_2` reader - PER_ID_2
pub type PER_ID_2_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - PER_ID_2
    #[inline(always)]
    pub fn per_id_2(&self) -> PER_ID_2_R {
        PER_ID_2_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZC_PID2")
            .field("per_id_2", &self.per_id_2())
            .finish()
    }
}
/**Peripheral ID 2.

You can [`read`](crate::Reg::read) this register and get [`tzc_pid2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:TZC_PID2)*/
pub struct TZC_PID2rs;
impl crate::RegisterSpec for TZC_PID2rs {
    type Ux = u32;
}
///`read()` method returns [`tzc_pid2::R`](R) reader structure
impl crate::Readable for TZC_PID2rs {}
///`reset()` method sets TZC_PID2 to value 0x2b
impl crate::Resettable for TZC_PID2rs {
    const RESET_VALUE: u32 = 0x2b;
}
