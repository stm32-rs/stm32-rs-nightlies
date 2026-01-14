///Register `RDATAR` reader
pub type R = crate::R<RDATARrs>;
///Field `RDATA` reader - Regular channel conversion data
pub type RDATA_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Regular channel conversion data
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDATAR")
            .field("rdata", &self.rdata())
            .finish()
    }
}
/**data register for the regular channel

You can [`read`](crate::Reg::read) this register and get [`rdatar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:RDATAR)*/
pub struct RDATARrs;
impl crate::RegisterSpec for RDATARrs {
    type Ux = u32;
}
///`read()` method returns [`rdatar::R`](R) reader structure
impl crate::Readable for RDATARrs {}
///`reset()` method sets RDATAR to value 0
impl crate::Resettable for RDATARrs {}
