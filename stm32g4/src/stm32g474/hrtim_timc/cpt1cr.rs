///Register `CPT1CR` reader
pub type R = crate::R<CPT1CRrs>;
///Field `CPT1x` reader - Timerx Capture 1 value
pub type CPT1X_R = crate::FieldReader<u16>;
///Field `DIR` reader - Timerx Capture 1 Direction status
pub type DIR_R = crate::BitReader;
impl R {
    ///Bits 0:15 - Timerx Capture 1 value
    #[inline(always)]
    pub fn cpt1x(&self) -> CPT1X_R {
        CPT1X_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - Timerx Capture 1 Direction status
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPT1CR")
            .field("cpt1x", &self.cpt1x())
            .field("dir", &self.dir())
            .finish()
    }
}
/**Timerx Capture 1 Register

You can [`read`](crate::Reg::read) this register and get [`cpt1cr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIMC:CPT1CR)*/
pub struct CPT1CRrs;
impl crate::RegisterSpec for CPT1CRrs {
    type Ux = u32;
}
///`read()` method returns [`cpt1cr::R`](R) reader structure
impl crate::Readable for CPT1CRrs {}
///`reset()` method sets CPT1CR to value 0
impl crate::Resettable for CPT1CRrs {
    const RESET_VALUE: u32 = 0;
}
