///Register `RFC2R` reader
pub type R = crate::R<RFC2Rrs>;
///Field `FRAME` reader - frame number Current value of the relative frame counter 2.
pub type FRAME_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:11 - frame number Current value of the relative frame counter 2.
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFC2R")
            .field("frame", &self.frame())
            .finish()
    }
}
/**GFXTIM relative frame counter 2 register

You can [`read`](crate::Reg::read) this register and get [`rfc2r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#GFXTIM:RFC2R)*/
pub struct RFC2Rrs;
impl crate::RegisterSpec for RFC2Rrs {
    type Ux = u32;
}
///`read()` method returns [`rfc2r::R`](R) reader structure
impl crate::Readable for RFC2Rrs {}
///`reset()` method sets RFC2R to value 0
impl crate::Resettable for RFC2Rrs {}
