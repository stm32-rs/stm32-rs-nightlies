///Register `PTPTSLR` reader
pub type R = crate::R<PTPTSLRrs>;
///Field `STSS` reader - STSS
pub type STSS_R = crate::FieldReader<u32>;
///Field `STPNS` reader - STPNS
pub type STPNS_R = crate::BitReader;
impl R {
    ///Bits 0:30 - STSS
    #[inline(always)]
    pub fn stss(&self) -> STSS_R {
        STSS_R::new(self.bits & 0x7fff_ffff)
    }
    ///Bit 31 - STPNS
    #[inline(always)]
    pub fn stpns(&self) -> STPNS_R {
        STPNS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTSLR")
            .field("stss", &self.stss())
            .field("stpns", &self.stpns())
            .finish()
    }
}
/**Ethernet PTP time stamp low register

You can [`read`](crate::Reg::read) this register and get [`ptptslr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#Ethernet_PTP:PTPTSLR)*/
pub struct PTPTSLRrs;
impl crate::RegisterSpec for PTPTSLRrs {
    type Ux = u32;
}
///`read()` method returns [`ptptslr::R`](R) reader structure
impl crate::Readable for PTPTSLRrs {}
///`reset()` method sets PTPTSLR to value 0
impl crate::Resettable for PTPTSLRrs {}
