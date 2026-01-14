///Register `ATR` reader
pub type R = crate::R<ATRrs>;
///Field `LINE` reader - line number
pub type LINE_R = crate::FieldReader<u16>;
///Field `FRAME` reader - fame number
pub type FRAME_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:11 - line number
    #[inline(always)]
    pub fn line(&self) -> LINE_R {
        LINE_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 12:31 - fame number
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ATR")
            .field("line", &self.line())
            .field("frame", &self.frame())
            .finish()
    }
}
/**GFXTIM absolute time register

You can [`read`](crate::Reg::read) this register and get [`atr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:ATR)*/
pub struct ATRrs;
impl crate::RegisterSpec for ATRrs {
    type Ux = u32;
}
///`read()` method returns [`atr::R`](R) reader structure
impl crate::Readable for ATRrs {}
///`reset()` method sets ATR to value 0
impl crate::Resettable for ATRrs {}
