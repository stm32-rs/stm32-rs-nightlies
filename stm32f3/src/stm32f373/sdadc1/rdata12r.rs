///Register `RDATA12R` reader
pub type R = crate::R<RDATA12Rrs>;
///Field `RDATA1` reader - Regular conversion data for SDADC1
pub type RDATA1_R = crate::FieldReader<u16>;
///Field `RDATA2` reader - Regular conversion data for SDADC2
pub type RDATA2_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Regular conversion data for SDADC1
    #[inline(always)]
    pub fn rdata1(&self) -> RDATA1_R {
        RDATA1_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Regular conversion data for SDADC2
    #[inline(always)]
    pub fn rdata2(&self) -> RDATA2_R {
        RDATA2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDATA12R")
            .field("rdata2", &self.rdata2())
            .field("rdata1", &self.rdata1())
            .finish()
    }
}
/**SDADC1 and SDADC2 regular data register

You can [`read`](crate::Reg::read) this register and get [`rdata12r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:RDATA12R)*/
pub struct RDATA12Rrs;
impl crate::RegisterSpec for RDATA12Rrs {
    type Ux = u32;
}
///`read()` method returns [`rdata12r::R`](R) reader structure
impl crate::Readable for RDATA12Rrs {}
///`reset()` method sets RDATA12R to value 0
impl crate::Resettable for RDATA12Rrs {}
