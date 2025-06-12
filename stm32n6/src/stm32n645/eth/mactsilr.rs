///Register `MACTSILR` reader
pub type R = crate::R<MACTSILRrs>;
///Field `ITLSNS` reader - Ingress Timestamp Latency, in subnanoseconds
pub type ITLSNS_R = crate::FieldReader;
///Field `ITLNS` reader - Ingress Timestamp Latency, in nanoseconds
pub type ITLNS_R = crate::FieldReader<u16>;
impl R {
    ///Bits 8:15 - Ingress Timestamp Latency, in subnanoseconds
    #[inline(always)]
    pub fn itlsns(&self) -> ITLSNS_R {
        ITLSNS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:27 - Ingress Timestamp Latency, in nanoseconds
    #[inline(always)]
    pub fn itlns(&self) -> ITLNS_R {
        ITLNS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACTSILR")
            .field("itlsns", &self.itlsns())
            .field("itlns", &self.itlns())
            .finish()
    }
}
/**Timestamp Ingress Latency register

You can [`read`](crate::Reg::read) this register and get [`mactsilr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MACTSILR)*/
pub struct MACTSILRrs;
impl crate::RegisterSpec for MACTSILRrs {
    type Ux = u32;
}
///`read()` method returns [`mactsilr::R`](R) reader structure
impl crate::Readable for MACTSILRrs {}
///`reset()` method sets MACTSILR to value 0
impl crate::Resettable for MACTSILRrs {}
