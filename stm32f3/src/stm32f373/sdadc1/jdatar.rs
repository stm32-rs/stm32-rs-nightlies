///Register `JDATAR` reader
pub type R = crate::R<JDATARrs>;
///Field `JDATA` reader - Injected group conversion data
pub type JDATA_R = crate::FieldReader<u16>;
///Field `JDATACH` reader - Injected channel most recently converted
pub type JDATACH_R = crate::FieldReader;
impl R {
    ///Bits 0:15 - Injected group conversion data
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 25:28 - Injected channel most recently converted
    #[inline(always)]
    pub fn jdatach(&self) -> JDATACH_R {
        JDATACH_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDATAR")
            .field("jdatach", &self.jdatach())
            .field("jdata", &self.jdata())
            .finish()
    }
}
/**data register for injected group

You can [`read`](crate::Reg::read) this register and get [`jdatar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:JDATAR)*/
pub struct JDATARrs;
impl crate::RegisterSpec for JDATARrs {
    type Ux = u32;
}
///`read()` method returns [`jdatar::R`](R) reader structure
impl crate::Readable for JDATARrs {}
///`reset()` method sets JDATAR to value 0
impl crate::Resettable for JDATARrs {}
