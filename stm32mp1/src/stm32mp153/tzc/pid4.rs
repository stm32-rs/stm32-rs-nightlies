///Register `PID4` reader
pub type R = crate::R<PID4rs>;
///Field `PER_ID_4` reader - PER_ID_4
pub type PER_ID_4_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - PER_ID_4
    #[inline(always)]
    pub fn per_id_4(&self) -> PER_ID_4_R {
        PER_ID_4_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PID4")
            .field("per_id_4", &self.per_id_4())
            .finish()
    }
}
/**Peripheral ID 4.

You can [`read`](crate::Reg::read) this register and get [`pid4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TZC:PID4)*/
pub struct PID4rs;
impl crate::RegisterSpec for PID4rs {
    type Ux = u32;
}
///`read()` method returns [`pid4::R`](R) reader structure
impl crate::Readable for PID4rs {}
///`reset()` method sets PID4 to value 0x04
impl crate::Resettable for PID4rs {
    const RESET_VALUE: u32 = 0x04;
}
