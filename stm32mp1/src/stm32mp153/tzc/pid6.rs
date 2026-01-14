///Register `PID6` reader
pub type R = crate::R<PID6rs>;
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
        f.debug_struct("PID6")
            .field("per_id_6", &self.per_id_6())
            .finish()
    }
}
/**Peripheral ID 6.

You can [`read`](crate::Reg::read) this register and get [`pid6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TZC:PID6)*/
pub struct PID6rs;
impl crate::RegisterSpec for PID6rs {
    type Ux = u32;
}
///`read()` method returns [`pid6::R`](R) reader structure
impl crate::Readable for PID6rs {}
///`reset()` method sets PID6 to value 0
impl crate::Resettable for PID6rs {}
