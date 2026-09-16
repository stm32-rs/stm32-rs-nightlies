///Register `FLT3RDATAR` reader
pub type R = crate::R<FLT3RDATARrs>;
///Field `RDATACH` reader - RDATACH
pub type RDATACH_R = crate::FieldReader;
///Field `RPEND` reader - RPEND
pub type RPEND_R = crate::BitReader;
///Field `RDATA` reader - RDATA
pub type RDATA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:2 - RDATACH
    #[inline(always)]
    pub fn rdatach(&self) -> RDATACH_R {
        RDATACH_R::new((self.bits & 7) as u8)
    }
    ///Bit 4 - RPEND
    #[inline(always)]
    pub fn rpend(&self) -> RPEND_R {
        RPEND_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:31 - RDATA
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLT3RDATAR")
            .field("rdatach", &self.rdatach())
            .field("rpend", &self.rpend())
            .field("rdata", &self.rdata())
            .finish()
    }
}
/**DFSDM filter 3 data register for the regular channel

You can [`read`](crate::Reg::read) this register and get [`flt3rdatar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:FLT3RDATAR)*/
pub struct FLT3RDATARrs;
impl crate::RegisterSpec for FLT3RDATARrs {
    type Ux = u32;
}
///`read()` method returns [`flt3rdatar::R`](R) reader structure
impl crate::Readable for FLT3RDATARrs {}
///`reset()` method sets FLT3RDATAR to value 0
impl crate::Resettable for FLT3RDATARrs {}
