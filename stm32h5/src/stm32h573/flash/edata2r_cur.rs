///Register `EDATA2R_CUR` reader
pub type R = crate::R<EDATA2R_CURrs>;
///Field `EDATA2_STRT` reader - EDATA2_STRT contains the start sectors of the flash high-cycle data area in Bank 2 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 2 are reserved for flash high-cycle data
pub type EDATA2_STRT_R = crate::FieldReader;
///Field `EDATA2_EN` reader - Bank2 flash high-cycle data enable
pub type EDATA2_EN_R = crate::BitReader;
impl R {
    ///Bits 0:2 - EDATA2_STRT contains the start sectors of the flash high-cycle data area in Bank 2 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 2 are reserved for flash high-cycle data
    #[inline(always)]
    pub fn edata2_strt(&self) -> EDATA2_STRT_R {
        EDATA2_STRT_R::new((self.bits & 7) as u8)
    }
    ///Bit 15 - Bank2 flash high-cycle data enable
    #[inline(always)]
    pub fn edata2_en(&self) -> EDATA2_EN_R {
        EDATA2_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EDATA2R_CUR")
            .field("edata2_strt", &self.edata2_strt())
            .field("edata2_en", &self.edata2_en())
            .finish()
    }
}
/**FLASH data sectors configuration Bank 2

You can [`read`](crate::Reg::read) this register and get [`edata2r_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#FLASH:EDATA2R_CUR)*/
pub struct EDATA2R_CURrs;
impl crate::RegisterSpec for EDATA2R_CURrs {
    type Ux = u32;
}
///`read()` method returns [`edata2r_cur::R`](R) reader structure
impl crate::Readable for EDATA2R_CURrs {}
///`reset()` method sets EDATA2R_CUR to value 0
impl crate::Resettable for EDATA2R_CURrs {}
