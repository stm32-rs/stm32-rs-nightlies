///Register `MACATSNR` reader
pub type R = crate::R<MACATSNRrs>;
///Field `AUXTSLO` reader - AUXTSLO
pub type AUXTSLO_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:30 - AUXTSLO
    #[inline(always)]
    pub fn auxtslo(&self) -> AUXTSLO_R {
        AUXTSLO_R::new(self.bits & 0x7fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACATSNR")
            .field("auxtslo", &self.auxtslo())
            .finish()
    }
}
/**The Auxiliary Timestamp Nanoseconds register, along with ETH_MACATSSR, gives the 64-bit timestamp stored as auxiliary snapshot. These two registers form the read port of a 64-bit wide FIFO with a depth of 4 words. You can store multiple snapshots in this FIFO. Bits\[29:25\] in ETH_MACTSSR indicate the fill-level of the FIFO. The top of the FIFO is removed only when the last byte of MAC Register 91 (Auxiliary Timestamp - Seconds Register) is read. In the little-endian mode, this means when Bits\[31:24\] are read and in big-endian mode, Bits\[7:0\] are read.

You can [`read`](crate::Reg::read) this register and get [`macatsnr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:MACATSNR)*/
pub struct MACATSNRrs;
impl crate::RegisterSpec for MACATSNRrs {
    type Ux = u32;
}
///`read()` method returns [`macatsnr::R`](R) reader structure
impl crate::Readable for MACATSNRrs {}
///`reset()` method sets MACATSNR to value 0
impl crate::Resettable for MACATSNRrs {}
