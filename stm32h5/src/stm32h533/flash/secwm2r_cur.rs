///Register `SECWM2R_CUR` reader
pub type R = crate::R<SECWM2R_CURrs>;
///Field `SECWM2_STRT` reader - Bank2 security WM area start sector
pub type SECWM2_STRT_R = crate::FieldReader;
///Field `SECWM2_END` reader - Bank2 security WM end sector
pub type SECWM2_END_R = crate::FieldReader;
impl R {
    ///Bits 0:6 - Bank2 security WM area start sector
    #[inline(always)]
    pub fn secwm2_strt(&self) -> SECWM2_STRT_R {
        SECWM2_STRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - Bank2 security WM end sector
    #[inline(always)]
    pub fn secwm2_end(&self) -> SECWM2_END_R {
        SECWM2_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECWM2R_CUR")
            .field("secwm2_strt", &self.secwm2_strt())
            .field("secwm2_end", &self.secwm2_end())
            .finish()
    }
}
/**FLASH security watermark for Bank2

You can [`read`](crate::Reg::read) this register and get [`secwm2r_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#FLASH:SECWM2R_CUR)*/
pub struct SECWM2R_CURrs;
impl crate::RegisterSpec for SECWM2R_CURrs {
    type Ux = u32;
}
///`read()` method returns [`secwm2r_cur::R`](R) reader structure
impl crate::Readable for SECWM2R_CURrs {}
///`reset()` method sets SECWM2R_CUR to value 0
impl crate::Resettable for SECWM2R_CURrs {}
