///Register `P1BPRSR` reader
pub type R = crate::R<P1BPRSRrs>;
///Field `BADCNT` reader - Amount of detected bad pixels
pub type BADCNT_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:11 - Amount of detected bad pixels
    #[inline(always)]
    pub fn badcnt(&self) -> BADCNT_R {
        BADCNT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1BPRSR")
            .field("badcnt", &self.badcnt())
            .finish()
    }
}
/**DCMIPP Pipe1 bad pixel removal status register

You can [`read`](crate::Reg::read) this register and get [`p1bprsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P1BPRSR)*/
pub struct P1BPRSRrs;
impl crate::RegisterSpec for P1BPRSRrs {
    type Ux = u32;
}
///`read()` method returns [`p1bprsr::R`](R) reader structure
impl crate::Readable for P1BPRSRrs {}
///`reset()` method sets P1BPRSR to value 0
impl crate::Resettable for P1BPRSRrs {}
