///Register `HDP1R_PRG` reader
pub type R = crate::R<HDP1R_PRGrs>;
///Field `HDP1_STRT` reader - Bank 1 HDPL barrier start set in number of 8 Kbytes sectors
pub type HDP1_STRT_R = crate::FieldReader;
///Field `HDP1_END` reader - Bank 1 HDPL barrier end set in number of 8 Kbytes sectors
pub type HDP1_END_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - Bank 1 HDPL barrier start set in number of 8 Kbytes sectors
    #[inline(always)]
    pub fn hdp1_strt(&self) -> HDP1_STRT_R {
        HDP1_STRT_R::new((self.bits & 7) as u8)
    }
    ///Bits 16:18 - Bank 1 HDPL barrier end set in number of 8 Kbytes sectors
    #[inline(always)]
    pub fn hdp1_end(&self) -> HDP1_END_R {
        HDP1_END_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HDP1R_PRG")
            .field("hdp1_strt", &self.hdp1_strt())
            .field("hdp1_end", &self.hdp1_end())
            .finish()
    }
}
/**FLASH HDP Bank1 register

You can [`read`](crate::Reg::read) this register and get [`hdp1r_prg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:HDP1R_PRG)*/
pub struct HDP1R_PRGrs;
impl crate::RegisterSpec for HDP1R_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`hdp1r_prg::R`](R) reader structure
impl crate::Readable for HDP1R_PRGrs {}
///`reset()` method sets HDP1R_PRG to value 0
impl crate::Resettable for HDP1R_PRGrs {}
