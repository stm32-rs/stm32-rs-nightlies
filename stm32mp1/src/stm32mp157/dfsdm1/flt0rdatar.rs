///Register `FLT0RDATAR` reader
pub type R = crate::R<FLT0RDATARrs>;
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
        f.debug_struct("FLT0RDATAR")
            .field("rdatach", &self.rdatach())
            .field("rpend", &self.rpend())
            .field("rdata", &self.rdata())
            .finish()
    }
}
/**DFSDM filter 0 data register for the regular channel

You can [`read`](crate::Reg::read) this register and get [`flt0rdatar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:FLT0RDATAR)*/
pub struct FLT0RDATARrs;
impl crate::RegisterSpec for FLT0RDATARrs {
    type Ux = u32;
}
///`read()` method returns [`flt0rdatar::R`](R) reader structure
impl crate::Readable for FLT0RDATARrs {}
///`reset()` method sets FLT0RDATAR to value 0
impl crate::Resettable for FLT0RDATARrs {
    const RESET_VALUE: u32 = 0;
}