///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `ISOST` reader - FMC isolation state with respect to the AXI interface
pub type ISOST_R = crate::FieldReader;
///Field `PEF` reader - Pipe Empty Flag
pub type PEF_R = crate::BitReader;
///Field `NWRF` reader - NAND write request flag
pub type NWRF_R = crate::BitReader;
impl R {
    ///Bits 0:1 - FMC isolation state with respect to the AXI interface
    #[inline(always)]
    pub fn isost(&self) -> ISOST_R {
        ISOST_R::new((self.bits & 3) as u8)
    }
    ///Bit 4 - Pipe Empty Flag
    #[inline(always)]
    pub fn pef(&self) -> PEF_R {
        PEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - NAND write request flag
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
/**FMC status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#FMC1:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0x53
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0x53;
}
