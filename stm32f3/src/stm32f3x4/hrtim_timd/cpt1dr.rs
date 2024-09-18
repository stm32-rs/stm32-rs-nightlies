///Register `CPT1DR` reader
pub type R = crate::R<CPT1DRrs>;
///Field `CPT1x` reader - Timerx Capture 1 value
pub type CPT1X_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Timerx Capture 1 value
    #[inline(always)]
    pub fn cpt1x(&self) -> CPT1X_R {
        CPT1X_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPT1DR")
            .field("cpt1x", &self.cpt1x())
            .finish()
    }
}
/**Timerx Capture 1 Register

You can [`read`](crate::Reg::read) this register and get [`cpt1dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:CPT1DR)*/
pub struct CPT1DRrs;
impl crate::RegisterSpec for CPT1DRrs {
    type Ux = u32;
}
///`read()` method returns [`cpt1dr::R`](R) reader structure
impl crate::Readable for CPT1DRrs {}
///`reset()` method sets CPT1DR to value 0
impl crate::Resettable for CPT1DRrs {
    const RESET_VALUE: u32 = 0;
}
