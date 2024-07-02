///Register `CPT1BR` reader
pub type R = crate::R<CPT1BRrs>;
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
        f.debug_struct("CPT1BR")
            .field("cpt1x", &self.cpt1x())
            .finish()
    }
}
/**Timerx Capture 1 Register

You can [`read`](crate::Reg::read) this register and get [`cpt1br::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#HRTIM_TIMB:CPT1BR)*/
pub struct CPT1BRrs;
impl crate::RegisterSpec for CPT1BRrs {
    type Ux = u32;
}
///`read()` method returns [`cpt1br::R`](R) reader structure
impl crate::Readable for CPT1BRrs {}
///`reset()` method sets CPT1BR to value 0
impl crate::Resettable for CPT1BRrs {
    const RESET_VALUE: u32 = 0;
}
