///Register `EDATA1R_CUR` reader
pub type R = crate::R<EDATA1R_CURrs>;
///Field `EDATA1_STRT` reader - EDATA1_STRT contains the start sectors of the flash high-cycle data area in Bank1 There is no hardware effect to those bits.
pub type EDATA1_STRT_R = crate::FieldReader;
///Field `EDATA1_EN` reader - Bank1 flash high-cycle data enable
pub type EDATA1_EN_R = crate::BitReader;
impl R {
    ///Bits 0:2 - EDATA1_STRT contains the start sectors of the flash high-cycle data area in Bank1 There is no hardware effect to those bits.
    #[inline(always)]
    pub fn edata1_strt(&self) -> EDATA1_STRT_R {
        EDATA1_STRT_R::new((self.bits & 7) as u8)
    }
    ///Bit 15 - Bank1 flash high-cycle data enable
    #[inline(always)]
    pub fn edata1_en(&self) -> EDATA1_EN_R {
        EDATA1_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EDATA1R_CUR")
            .field("edata1_strt", &self.edata1_strt())
            .field("edata1_en", &self.edata1_en())
            .finish()
    }
}
/**FLASH data sector configuration Bank1

You can [`read`](crate::Reg::read) this register and get [`edata1r_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:EDATA1R_CUR)*/
pub struct EDATA1R_CURrs;
impl crate::RegisterSpec for EDATA1R_CURrs {
    type Ux = u32;
}
///`read()` method returns [`edata1r_cur::R`](R) reader structure
impl crate::Readable for EDATA1R_CURrs {}
///`reset()` method sets EDATA1R_CUR to value 0
impl crate::Resettable for EDATA1R_CURrs {}
