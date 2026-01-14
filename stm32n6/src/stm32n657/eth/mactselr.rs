///Register `MACTSELR` reader
pub type R = crate::R<MACTSELRrs>;
///Field `ETLSNS` reader - Egress Timestamp Latency, in subnanoseconds
pub type ETLSNS_R = crate::FieldReader;
///Field `ETLNS` reader - Egress Timestamp Latency, in nanoseconds
pub type ETLNS_R = crate::FieldReader<u16>;
impl R {
    ///Bits 8:15 - Egress Timestamp Latency, in subnanoseconds
    #[inline(always)]
    pub fn etlsns(&self) -> ETLSNS_R {
        ETLSNS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:27 - Egress Timestamp Latency, in nanoseconds
    #[inline(always)]
    pub fn etlns(&self) -> ETLNS_R {
        ETLNS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACTSELR")
            .field("etlsns", &self.etlsns())
            .field("etlns", &self.etlns())
            .finish()
    }
}
/**Timestamp Egress Latency register

You can [`read`](crate::Reg::read) this register and get [`mactselr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ETH:MACTSELR)*/
pub struct MACTSELRrs;
impl crate::RegisterSpec for MACTSELRrs {
    type Ux = u32;
}
///`read()` method returns [`mactselr::R`](R) reader structure
impl crate::Readable for MACTSELRrs {}
///`reset()` method sets MACTSELR to value 0
impl crate::Resettable for MACTSELRrs {}
