///Register `PID7` reader
pub type R = crate::R<PID7rs>;
///Field `PER_ID_7` reader - PER_ID_7
pub type PER_ID_7_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - PER_ID_7
    #[inline(always)]
    pub fn per_id_7(&self) -> PER_ID_7_R {
        PER_ID_7_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PID7")
            .field("per_id_7", &self.per_id_7())
            .finish()
    }
}
/**Peripheral ID 7.

You can [`read`](crate::Reg::read) this register and get [`pid7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TZC:PID7)*/
pub struct PID7rs;
impl crate::RegisterSpec for PID7rs {
    type Ux = u32;
}
///`read()` method returns [`pid7::R`](R) reader structure
impl crate::Readable for PID7rs {}
///`reset()` method sets PID7 to value 0
impl crate::Resettable for PID7rs {}
