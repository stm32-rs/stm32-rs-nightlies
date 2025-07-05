///Register `CDR` reader
pub type R = crate::R<CDRrs>;
///Field `RDATA_MST` reader - Regular data of the master ADC
pub type RDATA_MST_R = crate::FieldReader<u16>;
///Field `RDATA_SLV` reader - Regular data of the slave ADC
pub type RDATA_SLV_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Regular data of the master ADC
    #[inline(always)]
    pub fn rdata_mst(&self) -> RDATA_MST_R {
        RDATA_MST_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Regular data of the slave ADC
    #[inline(always)]
    pub fn rdata_slv(&self) -> RDATA_SLV_R {
        RDATA_SLV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CDR")
            .field("rdata_slv", &self.rdata_slv())
            .field("rdata_mst", &self.rdata_mst())
            .finish()
    }
}
/**ADC common regular data register for dual and triple modes

You can [`read`](crate::Reg::read) this register and get [`cdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#ADC12_Common:CDR)*/
pub struct CDRrs;
impl crate::RegisterSpec for CDRrs {
    type Ux = u32;
}
///`read()` method returns [`cdr::R`](R) reader structure
impl crate::Readable for CDRrs {}
///`reset()` method sets CDR to value 0
impl crate::Resettable for CDRrs {}
