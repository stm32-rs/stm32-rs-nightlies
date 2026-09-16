///Register `RFC1R` reader
pub type R = crate::R<RFC1Rrs>;
///Field `FRAME` reader - frame number
pub type FRAME_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:11 - frame number
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFC1R")
            .field("frame", &self.frame())
            .finish()
    }
}
/**GFXTIM relative frame counter 1 register

You can [`read`](crate::Reg::read) this register and get [`rfc1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#GFXTIM:RFC1R)*/
pub struct RFC1Rrs;
impl crate::RegisterSpec for RFC1Rrs {
    type Ux = u32;
}
///`read()` method returns [`rfc1r::R`](R) reader structure
impl crate::Readable for RFC1Rrs {}
///`reset()` method sets RFC1R to value 0
impl crate::Resettable for RFC1Rrs {}
