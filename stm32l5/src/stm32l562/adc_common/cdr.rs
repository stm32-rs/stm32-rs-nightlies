///Register `CDR` reader
pub type R = crate::R<CDRrs>;
///Field `RDATA_MST` reader - RDATA_MST
pub type RDATA_MST_R = crate::FieldReader<u16>;
///Field `RDATA_SLV` reader - RDATA_SLV
pub type RDATA_SLV_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - RDATA_MST
    #[inline(always)]
    pub fn rdata_mst(&self) -> RDATA_MST_R {
        RDATA_MST_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - RDATA_SLV
    #[inline(always)]
    pub fn rdata_slv(&self) -> RDATA_SLV_R {
        RDATA_SLV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CDR")
            .field("rdata_mst", &self.rdata_mst())
            .field("rdata_slv", &self.rdata_slv())
            .finish()
    }
}
/**Common regular data register for dual mode

You can [`read`](crate::Reg::read) this register and get [`cdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#ADC_Common:CDR)*/
pub struct CDRrs;
impl crate::RegisterSpec for CDRrs {
    type Ux = u32;
}
///`read()` method returns [`cdr::R`](R) reader structure
impl crate::Readable for CDRrs {}
///`reset()` method sets CDR to value 0
impl crate::Resettable for CDRrs {}
