///Register `DFSDM3_JDATAR` reader
pub type R = crate::R<DFSDM3_JDATARrs>;
///Field `JDATACH` reader - Injected channel most recently converted
pub type JDATACH_R = crate::FieldReader;
///Field `JDATA` reader - Injected group conversion data
pub type JDATA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:2 - Injected channel most recently converted
    #[inline(always)]
    pub fn jdatach(&self) -> JDATACH_R {
        JDATACH_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:31 - Injected group conversion data
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFSDM3_JDATAR")
            .field("jdatach", &self.jdatach())
            .field("jdata", &self.jdata())
            .finish()
    }
}
/**DFSDM data register for injected group

You can [`read`](crate::Reg::read) this register and get [`dfsdm3_jdatar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F765.html#DFSDM1:DFSDM3_JDATAR)*/
pub struct DFSDM3_JDATARrs;
impl crate::RegisterSpec for DFSDM3_JDATARrs {
    type Ux = u32;
}
///`read()` method returns [`dfsdm3_jdatar::R`](R) reader structure
impl crate::Readable for DFSDM3_JDATARrs {}
///`reset()` method sets DFSDM3_JDATAR to value 0
impl crate::Resettable for DFSDM3_JDATARrs {
    const RESET_VALUE: u32 = 0;
}
