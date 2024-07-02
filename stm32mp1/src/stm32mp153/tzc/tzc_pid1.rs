///Register `TZC_PID1` reader
pub type R = crate::R<TZC_PID1rs>;
///Field `PER_ID_1` reader - PER_ID_1
pub type PER_ID_1_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - PER_ID_1
    #[inline(always)]
    pub fn per_id_1(&self) -> PER_ID_1_R {
        PER_ID_1_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZC_PID1")
            .field("per_id_1", &self.per_id_1())
            .finish()
    }
}
/**Peripheral ID 1.

You can [`read`](crate::Reg::read) this register and get [`tzc_pid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TZC:TZC_PID1)*/
pub struct TZC_PID1rs;
impl crate::RegisterSpec for TZC_PID1rs {
    type Ux = u32;
}
///`read()` method returns [`tzc_pid1::R`](R) reader structure
impl crate::Readable for TZC_PID1rs {}
///`reset()` method sets TZC_PID1 to value 0xb4
impl crate::Resettable for TZC_PID1rs {
    const RESET_VALUE: u32 = 0xb4;
}
