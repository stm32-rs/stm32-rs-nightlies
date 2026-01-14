///Register `PID0` reader
pub type R = crate::R<PID0rs>;
///Field `PER_ID_0` reader - PER_ID_0
pub type PER_ID_0_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - PER_ID_0
    #[inline(always)]
    pub fn per_id_0(&self) -> PER_ID_0_R {
        PER_ID_0_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PID0")
            .field("per_id_0", &self.per_id_0())
            .finish()
    }
}
/**Peripheral ID 0.

You can [`read`](crate::Reg::read) this register and get [`pid0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TZC:PID0)*/
pub struct PID0rs;
impl crate::RegisterSpec for PID0rs {
    type Ux = u32;
}
///`read()` method returns [`pid0::R`](R) reader structure
impl crate::Readable for PID0rs {}
///`reset()` method sets PID0 to value 0x60
impl crate::Resettable for PID0rs {
    const RESET_VALUE: u32 = 0x60;
}
