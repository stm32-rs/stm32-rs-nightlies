///Register `CPT1R` reader
pub type R = crate::R<CPT1Rrs>;
///Field `CPT` reader - Timerx Capture 1 value
pub type CPT_R = crate::FieldReader<u16>;
///Field `DIR` reader - Timerx Capture 1 Direction status
pub type DIR_R = crate::BitReader;
impl R {
    ///Bits 0:15 - Timerx Capture 1 value
    #[inline(always)]
    pub fn cpt(&self) -> CPT_R {
        CPT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - Timerx Capture 1 Direction status
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPT1R")
            .field("cpt", &self.cpt())
            .field("dir", &self.dir())
            .finish()
    }
}
/**Timerx Capture 1 Register

You can [`read`](crate::Reg::read) this register and get [`cpt1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#HRTIM_TIMA:CPT1R)*/
pub struct CPT1Rrs;
impl crate::RegisterSpec for CPT1Rrs {
    type Ux = u32;
}
///`read()` method returns [`cpt1r::R`](R) reader structure
impl crate::Readable for CPT1Rrs {}
///`reset()` method sets CPT1R to value 0
impl crate::Resettable for CPT1Rrs {}
