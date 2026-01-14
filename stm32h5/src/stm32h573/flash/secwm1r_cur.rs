///Register `SECWM1R_CUR` reader
pub type R = crate::R<SECWM1R_CURrs>;
///Field `SECWM1_STRT` reader - Bank1 security WM area 1 start sector
pub type SECWM1_STRT_R = crate::FieldReader;
///Field `SECWM1_END` reader - Bank1 security WM area 1 end sector
pub type SECWM1_END_R = crate::FieldReader;
impl R {
    ///Bits 0:6 - Bank1 security WM area 1 start sector
    #[inline(always)]
    pub fn secwm1_strt(&self) -> SECWM1_STRT_R {
        SECWM1_STRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - Bank1 security WM area 1 end sector
    #[inline(always)]
    pub fn secwm1_end(&self) -> SECWM1_END_R {
        SECWM1_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECWM1R_CUR")
            .field("secwm1_strt", &self.secwm1_strt())
            .field("secwm1_end", &self.secwm1_end())
            .finish()
    }
}
/**FLASH security watermark for Bank 1

You can [`read`](crate::Reg::read) this register and get [`secwm1r_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#FLASH:SECWM1R_CUR)*/
pub struct SECWM1R_CURrs;
impl crate::RegisterSpec for SECWM1R_CURrs {
    type Ux = u32;
}
///`read()` method returns [`secwm1r_cur::R`](R) reader structure
impl crate::Readable for SECWM1R_CURrs {}
///`reset()` method sets SECWM1R_CUR to value 0
impl crate::Resettable for SECWM1R_CURrs {}
