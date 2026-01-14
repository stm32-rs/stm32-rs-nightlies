///Register `FLT4JDATAR` reader
pub type R = crate::R<FLT4JDATARrs>;
///Field `JDATACH` reader - JDATACH
pub type JDATACH_R = crate::FieldReader;
///Field `JDATA` reader - JDATA
pub type JDATA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:2 - JDATACH
    #[inline(always)]
    pub fn jdatach(&self) -> JDATACH_R {
        JDATACH_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:31 - JDATA
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLT4JDATAR")
            .field("jdatach", &self.jdatach())
            .field("jdata", &self.jdata())
            .finish()
    }
}
/**DFSDM filter 4 data register for injected group

You can [`read`](crate::Reg::read) this register and get [`flt4jdatar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:FLT4JDATAR)*/
pub struct FLT4JDATARrs;
impl crate::RegisterSpec for FLT4JDATARrs {
    type Ux = u32;
}
///`read()` method returns [`flt4jdatar::R`](R) reader structure
impl crate::Readable for FLT4JDATARrs {}
///`reset()` method sets FLT4JDATAR to value 0
impl crate::Resettable for FLT4JDATARrs {}
