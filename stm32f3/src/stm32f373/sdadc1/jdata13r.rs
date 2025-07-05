///Register `JDATA13R` reader
pub type R = crate::R<JDATA13Rrs>;
///Field `JDATA1` reader - Injected group conversion data for SDADC1
pub type JDATA1_R = crate::FieldReader<u16>;
///Field `JDATA3` reader - Injected group conversion data for SDADC3
pub type JDATA3_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Injected group conversion data for SDADC1
    #[inline(always)]
    pub fn jdata1(&self) -> JDATA1_R {
        JDATA1_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Injected group conversion data for SDADC3
    #[inline(always)]
    pub fn jdata3(&self) -> JDATA3_R {
        JDATA3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDATA13R")
            .field("jdata3", &self.jdata3())
            .field("jdata1", &self.jdata1())
            .finish()
    }
}
/**SDADC1 and SDADC3 injected data register

You can [`read`](crate::Reg::read) this register and get [`jdata13r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:JDATA13R)*/
pub struct JDATA13Rrs;
impl crate::RegisterSpec for JDATA13Rrs {
    type Ux = u32;
}
///`read()` method returns [`jdata13r::R`](R) reader structure
impl crate::Readable for JDATA13Rrs {}
///`reset()` method sets JDATA13R to value 0
impl crate::Resettable for JDATA13Rrs {}
