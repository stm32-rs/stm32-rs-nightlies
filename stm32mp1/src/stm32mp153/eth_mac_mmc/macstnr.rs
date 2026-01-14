///Register `MACSTNR` reader
pub type R = crate::R<MACSTNRrs>;
///Field `TSSS` reader - TSSS
pub type TSSS_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:30 - TSSS
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
/**The System Time Nanoseconds register, along with System Time Seconds register, indicates the current value of the system time maintained by the MAC. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.

You can [`read`](crate::Reg::read) this register and get [`macstnr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACSTNR)*/
pub struct MACSTNRrs;
impl crate::RegisterSpec for MACSTNRrs {
    type Ux = u32;
}
///`read()` method returns [`macstnr::R`](R) reader structure
impl crate::Readable for MACSTNRrs {}
///`reset()` method sets MACSTNR to value 0
impl crate::Resettable for MACSTNRrs {}
