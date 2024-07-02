///Register `CPT2CR` reader
pub type R = crate::R<CPT2CRrs>;
///Field `CPT2x` reader - Timerx Capture 2 value
pub type CPT2X_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Timerx Capture 2 value
    #[inline(always)]
    pub fn cpt2x(&self) -> CPT2X_R {
        CPT2X_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPT2CR")
            .field("cpt2x", &self.cpt2x())
            .finish()
    }
}
/**Timerx Capture 2 Register

You can [`read`](crate::Reg::read) this register and get [`cpt2cr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:CPT2CR)*/
pub struct CPT2CRrs;
impl crate::RegisterSpec for CPT2CRrs {
    type Ux = u32;
}
///`read()` method returns [`cpt2cr::R`](R) reader structure
impl crate::Readable for CPT2CRrs {}
///`reset()` method sets CPT2CR to value 0
impl crate::Resettable for CPT2CRrs {
    const RESET_VALUE: u32 = 0;
}
