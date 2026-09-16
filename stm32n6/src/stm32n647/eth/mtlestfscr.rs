///Register `MTLESTFSCR` reader
pub type R = crate::R<MTLESTFSCRrs>;
///Field `HBFS` reader - Frame Size of HLBF
pub type HBFS_R = crate::FieldReader<u16>;
///Field `HBFQ` reader - Queue Number of HLBF
pub type HBFQ_R = crate::BitReader;
impl R {
    ///Bits 0:14 - Frame Size of HLBF
    #[inline(always)]
    pub fn hbfs(&self) -> HBFS_R {
        HBFS_R::new((self.bits & 0x7fff) as u16)
    }
    ///Bit 16 - Queue Number of HLBF
    #[inline(always)]
    pub fn hbfq(&self) -> HBFQ_R {
        HBFQ_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLESTFSCR")
            .field("hbfs", &self.hbfs())
            .field("hbfq", &self.hbfq())
            .finish()
    }
}
/**EST Frame size Capture Register

You can [`read`](crate::Reg::read) this register and get [`mtlestfscr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ETH:MTLESTFSCR)*/
pub struct MTLESTFSCRrs;
impl crate::RegisterSpec for MTLESTFSCRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlestfscr::R`](R) reader structure
impl crate::Readable for MTLESTFSCRrs {}
///`reset()` method sets MTLESTFSCR to value 0
impl crate::Resettable for MTLESTFSCRrs {}
