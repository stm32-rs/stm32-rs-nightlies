///Register `MACSTSR` reader
pub type R = crate::R<MACSTSRrs>;
///Field `TSS` reader - Timestamp Second
pub type TSS_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Timestamp Second
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACSTSR").field("tss", &self.tss()).finish()
    }
}
/**System time seconds register

You can [`read`](crate::Reg::read) this register and get [`macstsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#Ethernet_MAC:MACSTSR)*/
pub struct MACSTSRrs;
impl crate::RegisterSpec for MACSTSRrs {
    type Ux = u32;
}
///`read()` method returns [`macstsr::R`](R) reader structure
impl crate::Readable for MACSTSRrs {}
///`reset()` method sets MACSTSR to value 0
impl crate::Resettable for MACSTSRrs {}
