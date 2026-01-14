///Register `PCellID2` reader
pub type R = crate::R<PCELL_ID2rs>;
///Field `RNGPCellID2` reader - These bits are read back as 0x05
pub type RNGPCELL_ID2_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - These bits are read back as 0x05
    #[inline(always)]
    pub fn rngpcell_id2(&self) -> RNGPCELL_ID2_R {
        RNGPCELL_ID2_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCellID2")
            .field("rngpcell_id2", &self.rngpcell_id2())
            .finish()
    }
}
/**RNGPCellID2 register

You can [`read`](crate::Reg::read) this register and get [`pcell_id2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:PCellID2)*/
pub struct PCELL_ID2rs;
impl crate::RegisterSpec for PCELL_ID2rs {
    type Ux = u32;
}
///`read()` method returns [`pcell_id2::R`](R) reader structure
impl crate::Readable for PCELL_ID2rs {}
///`reset()` method sets PCellID2 to value 0x05
impl crate::Resettable for PCELL_ID2rs {
    const RESET_VALUE: u32 = 0x05;
}
