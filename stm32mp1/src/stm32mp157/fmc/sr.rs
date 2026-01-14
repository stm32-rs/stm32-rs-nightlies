///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `ISOST` reader - ISOST
pub type ISOST_R = crate::FieldReader;
///Field `PEF` reader - PEF
pub type PEF_R = crate::BitReader;
///Field `NWRF` reader - NWRF
pub type NWRF_R = crate::BitReader;
impl R {
    ///Bits 0:1 - ISOST
    #[inline(always)]
    pub fn isost(&self) -> ISOST_R {
        ISOST_R::new((self.bits & 3) as u8)
    }
    ///Bit 4 - PEF
    #[inline(always)]
    pub fn pef(&self) -> PEF_R {
        PEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - NWRF
    #[inline(always)]
    pub fn nwrf(&self) -> NWRF_R {
        NWRF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("isost", &self.isost())
            .field("pef", &self.pef())
            .field("nwrf", &self.nwrf())
            .finish()
    }
}
/**This register contains information about the AXI interface isolation status and the NAND write requests status. The FMC has to be disabled before modifying some registers. As requests might be pending, it is necessary to wait till the AXI interface is stable and the core of the block is totally isolated from its AXI interface before reconfiguring the registers. The PEF and PNWEF bits indicate the status of the pipe. If Hamming algorithm is used, the ECC is calculated while data are written to the memory. To read the correct ECC, the software must consequently wait untill no write request to the NAND controller are pending, by polling PEF and NWRF bits.

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FMC:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0x40
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x40;
}
