///Register `PCellID3` reader
pub type R = crate::R<PCELL_ID3rs>;
///Field `RNGPCellID3` reader - These bits are read back as 0xB1
pub type RNGPCELL_ID3_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - These bits are read back as 0xB1
    #[inline(always)]
    pub fn rngpcell_id3(&self) -> RNGPCELL_ID3_R {
        RNGPCELL_ID3_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCellID3")
            .field("rngpcell_id3", &self.rngpcell_id3())
            .finish()
    }
}
/**RNGPCellID3 register

You can [`read`](crate::Reg::read) this register and get [`pcell_id3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:PCellID3)*/
pub struct PCELL_ID3rs;
impl crate::RegisterSpec for PCELL_ID3rs {
    type Ux = u32;
}
///`read()` method returns [`pcell_id3::R`](R) reader structure
impl crate::Readable for PCELL_ID3rs {}
///`reset()` method sets PCellID3 to value 0xb1
impl crate::Resettable for PCELL_ID3rs {
    const RESET_VALUE: u32 = 0xb1;
}
