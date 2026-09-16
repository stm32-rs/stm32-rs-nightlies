///Register `MACSTNR` reader
pub type R = crate::R<MACSTNRrs>;
///Field `TSSS` reader - Timestamp Sub-seconds
pub type TSSS_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:30 - Timestamp Sub-seconds
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(self.bits & 0x7fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACSTNR")
            .field("tsss", &self.tsss())
            .finish()
    }
}
/**System time nanoseconds register

You can [`read`](crate::Reg::read) this register and get [`macstnr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#Ethernet_MAC:MACSTNR)*/
pub struct MACSTNRrs;
impl crate::RegisterSpec for MACSTNRrs {
    type Ux = u32;
}
///`read()` method returns [`macstnr::R`](R) reader structure
impl crate::Readable for MACSTNRrs {}
///`reset()` method sets MACSTNR to value 0
impl crate::Resettable for MACSTNRrs {}
