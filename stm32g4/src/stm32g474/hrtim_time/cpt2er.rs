///Register `CPT2ER` reader
pub type R = crate::R<CPT2ERrs>;
///Field `CPT2x` reader - Timerx Capture 2 value
pub type CPT2X_R = crate::FieldReader<u16>;
///Field `DIR` reader - Timerx Capture 1 Direction status
pub type DIR_R = crate::BitReader;
impl R {
    ///Bits 0:15 - Timerx Capture 2 value
    #[inline(always)]
    pub fn cpt2x(&self) -> CPT2X_R {
        CPT2X_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - Timerx Capture 1 Direction status
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPT2ER")
            .field("cpt2x", &self.cpt2x())
            .field("dir", &self.dir())
            .finish()
    }
}
/**Timerx Capture 2 Register

You can [`read`](crate::Reg::read) this register and get [`cpt2er::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIME:CPT2ER)*/
pub struct CPT2ERrs;
impl crate::RegisterSpec for CPT2ERrs {
    type Ux = u32;
}
///`read()` method returns [`cpt2er::R`](R) reader structure
impl crate::Readable for CPT2ERrs {}
///`reset()` method sets CPT2ER to value 0
impl crate::Resettable for CPT2ERrs {
    const RESET_VALUE: u32 = 0;
}
