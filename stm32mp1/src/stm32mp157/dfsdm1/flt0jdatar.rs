///Register `FLT0JDATAR` reader
pub type R = crate::R<FLT0JDATARrs>;
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
        f.debug_struct("FLT0JDATAR")
            .field("jdatach", &self.jdatach())
            .field("jdata", &self.jdata())
            .finish()
    }
}
/**DFSDM filter 0 data register for injected group

You can [`read`](crate::Reg::read) this register and get [`flt0jdatar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT0JDATAR)*/
pub struct FLT0JDATARrs;
impl crate::RegisterSpec for FLT0JDATARrs {
    type Ux = u32;
}
///`read()` method returns [`flt0jdatar::R`](R) reader structure
impl crate::Readable for FLT0JDATARrs {}
///`reset()` method sets FLT0JDATAR to value 0
impl crate::Resettable for FLT0JDATARrs {}
