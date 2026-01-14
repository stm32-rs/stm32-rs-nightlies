///Register `RDATA13R` reader
pub type R = crate::R<RDATA13Rrs>;
///Field `RDATA1` reader - Regular conversion data for SDADC1
pub type RDATA1_R = crate::FieldReader<u16>;
///Field `RDATA3` reader - Regular conversion data for SDADC3
pub type RDATA3_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Regular conversion data for SDADC1
    #[inline(always)]
    pub fn rdata1(&self) -> RDATA1_R {
        RDATA1_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Regular conversion data for SDADC3
    #[inline(always)]
    pub fn rdata3(&self) -> RDATA3_R {
        RDATA3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDATA13R")
            .field("rdata3", &self.rdata3())
            .field("rdata1", &self.rdata1())
            .finish()
    }
}
/**SDADC1 and SDADC3 regular data register

You can [`read`](crate::Reg::read) this register and get [`rdata13r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:RDATA13R)*/
pub struct RDATA13Rrs;
impl crate::RegisterSpec for RDATA13Rrs {
    type Ux = u32;
}
///`read()` method returns [`rdata13r::R`](R) reader structure
impl crate::Readable for RDATA13Rrs {}
///`reset()` method sets RDATA13R to value 0
impl crate::Resettable for RDATA13Rrs {}
