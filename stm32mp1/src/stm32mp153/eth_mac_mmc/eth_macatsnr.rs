#[doc = "Register `ETH_MACATSNR` reader"]
pub type R = crate::R<ETH_MACATSNRrs>;
#[doc = "Field `AUXTSLO` reader - AUXTSLO"]
pub type AUXTSLO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30 - AUXTSLO"]
    #[inline(always)]
    pub fn auxtslo(&self) -> AUXTSLO_R {
        AUXTSLO_R::new(self.bits & 0x7fff_ffff)
    }
}
#[doc = "The Auxiliary Timestamp Nanoseconds register, along with ETH_MACATSSR, gives the 64-bit timestamp stored as auxiliary snapshot. These two registers form the read port of a 64-bit wide FIFO with a depth of 4 words. You can store multiple snapshots in this FIFO. Bits\\[29:25\\]
in ETH_MACTSSR indicate the fill-level of the FIFO. The top of the FIFO is removed only when the last byte of MAC Register 91 (Auxiliary Timestamp - Seconds Register) is read. In the little-endian mode, this means when Bits\\[31:24\\]
are read and in big-endian mode, Bits\\[7:0\\]
are read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macatsnr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACATSNRrs;
impl crate::RegisterSpec for ETH_MACATSNRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macatsnr::R`](R) reader structure"]
impl crate::Readable for ETH_MACATSNRrs {}
#[doc = "`reset()` method sets ETH_MACATSNR to value 0"]
impl crate::Resettable for ETH_MACATSNRrs {
    const RESET_VALUE: u32 = 0;
}
