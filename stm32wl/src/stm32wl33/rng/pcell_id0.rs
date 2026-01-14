///Register `PCellID0` reader
pub type R = crate::R<PCELL_ID0rs>;
///Field `RNGPCellID0` reader - These bits are read back as 0x0D
pub type RNGPCELL_ID0_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - These bits are read back as 0x0D
    #[inline(always)]
    pub fn rngpcell_id0(&self) -> RNGPCELL_ID0_R {
        RNGPCELL_ID0_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCellID0")
            .field("rngpcell_id0", &self.rngpcell_id0())
            .finish()
    }
}
/**RNGPCellID0 register

You can [`read`](crate::Reg::read) this register and get [`pcell_id0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:PCellID0)*/
pub struct PCELL_ID0rs;
impl crate::RegisterSpec for PCELL_ID0rs {
    type Ux = u32;
}
///`read()` method returns [`pcell_id0::R`](R) reader structure
impl crate::Readable for PCELL_ID0rs {}
///`reset()` method sets PCellID0 to value 0x0d
impl crate::Resettable for PCELL_ID0rs {
    const RESET_VALUE: u32 = 0x0d;
}
