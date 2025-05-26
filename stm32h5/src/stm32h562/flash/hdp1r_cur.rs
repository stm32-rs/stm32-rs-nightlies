///Register `HDP1R_CUR` reader
pub type R = crate::R<HDP1R_CURrs>;
///Field `HDP1_STRT` reader - HDPL barrier start set in number of 8-Kbyte sectors
pub type HDP1_STRT_R = crate::FieldReader;
///Field `HDP1_END` reader - HDPL barrier end set in number of 8-Kbyte sectors
pub type HDP1_END_R = crate::FieldReader;
impl R {
    ///Bits 0:6 - HDPL barrier start set in number of 8-Kbyte sectors
    #[inline(always)]
    pub fn hdp1_strt(&self) -> HDP1_STRT_R {
        HDP1_STRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - HDPL barrier end set in number of 8-Kbyte sectors
    #[inline(always)]
    pub fn hdp1_end(&self) -> HDP1_END_R {
        HDP1_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HDP1R_CUR")
            .field("hdp1_strt", &self.hdp1_strt())
            .field("hdp1_end", &self.hdp1_end())
            .finish()
    }
}
/**FLASH HDP Bank 1 configuration

You can [`read`](crate::Reg::read) this register and get [`hdp1r_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FLASH:HDP1R_CUR)*/
pub struct HDP1R_CURrs;
impl crate::RegisterSpec for HDP1R_CURrs {
    type Ux = u32;
}
///`read()` method returns [`hdp1r_cur::R`](R) reader structure
impl crate::Readable for HDP1R_CURrs {}
///`reset()` method sets HDP1R_CUR to value 0
impl crate::Resettable for HDP1R_CURrs {}
