///Register `HDP2R_CUR` reader
pub type R = crate::R<HDP2R_CURrs>;
///Field `HDP2_STRT` reader - HDPL barrier start set in number of 8-Kbyte sectors
pub type HDP2_STRT_R = crate::FieldReader;
///Field `HDP2_END` reader - HDPL barrier end set in number of 8-Kbyte sectors
pub type HDP2_END_R = crate::FieldReader;
impl R {
    ///Bits 0:6 - HDPL barrier start set in number of 8-Kbyte sectors
    #[inline(always)]
    pub fn hdp2_strt(&self) -> HDP2_STRT_R {
        HDP2_STRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - HDPL barrier end set in number of 8-Kbyte sectors
    #[inline(always)]
    pub fn hdp2_end(&self) -> HDP2_END_R {
        HDP2_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HDP2R_CUR")
            .field("hdp2_strt", &self.hdp2_strt())
            .field("hdp2_end", &self.hdp2_end())
            .finish()
    }
}
/**FLASH HDP Bank 2 configuration

You can [`read`](crate::Reg::read) this register and get [`hdp2r_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#FLASH:HDP2R_CUR)*/
pub struct HDP2R_CURrs;
impl crate::RegisterSpec for HDP2R_CURrs {
    type Ux = u32;
}
///`read()` method returns [`hdp2r_cur::R`](R) reader structure
impl crate::Readable for HDP2R_CURrs {}
///`reset()` method sets HDP2R_CUR to value 0
impl crate::Resettable for HDP2R_CURrs {}
