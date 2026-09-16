///Register `PCellID1` reader
pub type R = crate::R<PCELL_ID1rs>;
///Field `RNGPCellID1` reader - These bits are read back as 0xF0
pub type RNGPCELL_ID1_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - These bits are read back as 0xF0
    #[inline(always)]
    pub fn rngpcell_id1(&self) -> RNGPCELL_ID1_R {
        RNGPCELL_ID1_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCellID1")
            .field("rngpcell_id1", &self.rngpcell_id1())
            .finish()
    }
}
/**RNGPCellID1 register

You can [`read`](crate::Reg::read) this register and get [`pcell_id1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:PCellID1)*/
pub struct PCELL_ID1rs;
impl crate::RegisterSpec for PCELL_ID1rs {
    type Ux = u32;
}
///`read()` method returns [`pcell_id1::R`](R) reader structure
impl crate::Readable for PCELL_ID1rs {}
///`reset()` method sets PCellID1 to value 0xf0
impl crate::Resettable for PCELL_ID1rs {
    const RESET_VALUE: u32 = 0xf0;
}
