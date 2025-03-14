///Register `FLT1RDATAR` reader
pub type R = crate::R<FLT1RDATARrs>;
///Field `RDATACH` reader - Regular channel most recently converted
pub type RDATACH_R = crate::FieldReader;
///Field `RPEND` reader - Regular channel pending data
pub type RPEND_R = crate::BitReader;
///Field `RDATA` reader - Regular channel conversion data
pub type RDATA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:2 - Regular channel most recently converted
    #[inline(always)]
    pub fn rdatach(&self) -> RDATACH_R {
        RDATACH_R::new((self.bits & 7) as u8)
    }
    ///Bit 4 - Regular channel pending data
    #[inline(always)]
    pub fn rpend(&self) -> RPEND_R {
        RPEND_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:31 - Regular channel conversion data
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLT1RDATAR")
            .field("rdata", &self.rdata())
            .field("rpend", &self.rpend())
            .field("rdatach", &self.rdatach())
            .finish()
    }
}
/**data register for the regular channel

You can [`read`](crate::Reg::read) this register and get [`flt1rdatar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#DFSDM1:FLT1RDATAR)*/
pub struct FLT1RDATARrs;
impl crate::RegisterSpec for FLT1RDATARrs {
    type Ux = u32;
}
///`read()` method returns [`flt1rdatar::R`](R) reader structure
impl crate::Readable for FLT1RDATARrs {}
///`reset()` method sets FLT1RDATAR to value 0
impl crate::Resettable for FLT1RDATARrs {}
