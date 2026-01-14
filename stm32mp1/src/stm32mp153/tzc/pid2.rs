///Register `PID2` reader
pub type R = crate::R<PID2rs>;
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
        f.debug_struct("PID2")
            .field("per_id_2", &self.per_id_2())
            .finish()
    }
}
/**Peripheral ID 2.

You can [`read`](crate::Reg::read) this register and get [`pid2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TZC:PID2)*/
pub struct PID2rs;
impl crate::RegisterSpec for PID2rs {
    type Ux = u32;
}
///`read()` method returns [`pid2::R`](R) reader structure
impl crate::Readable for PID2rs {}
///`reset()` method sets PID2 to value 0x2b
impl crate::Resettable for PID2rs {
    const RESET_VALUE: u32 = 0x2b;
}
