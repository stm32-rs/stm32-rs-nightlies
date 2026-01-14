///Register `SCM_COUNTER_VAL` reader
pub type R = crate::R<SCM_COUNTER_VALrs>;
///Field `SCM_COUNTER_CURRVAL` reader - Slow Clock Measurement: number of 16 MHz clock cycles contained in 32 slow clock periods.
pub type SCM_COUNTER_CURRVAL_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:14 - Slow Clock Measurement: number of 16 MHz clock cycles contained in 32 slow clock periods.
    #[inline(always)]
    pub fn scm_counter_currval(&self) -> SCM_COUNTER_CURRVAL_R {
        SCM_COUNTER_CURRVAL_R::new((self.bits & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCM_COUNTER_VAL")
            .field("scm_counter_currval", &self.scm_counter_currval())
            .finish()
    }
}
/**SCM_COUNTER_VAL register

You can [`read`](crate::Reg::read) this register and get [`scm_counter_val::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MISC:SCM_COUNTER_VAL)*/
pub struct SCM_COUNTER_VALrs;
impl crate::RegisterSpec for SCM_COUNTER_VALrs {
    type Ux = u32;
}
///`read()` method returns [`scm_counter_val::R`](R) reader structure
impl crate::Readable for SCM_COUNTER_VALrs {}
///`reset()` method sets SCM_COUNTER_VAL to value 0
impl crate::Resettable for SCM_COUNTER_VALrs {}
