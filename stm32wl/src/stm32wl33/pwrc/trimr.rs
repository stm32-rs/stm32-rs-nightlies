///Register `TRIMR` reader
pub type R = crate::R<TRIMRrs>;
///Field `RFD_REG_TRIM` reader - RFD_REG_TRIM\[2:0\]: RF LDO Trimming By default, this value is taken from the engi bytes; and saved on V12o domain when OBL done. if associated ENGTRIM is enabled the RF LDO trimming can be controlled by the dedicated ENGTRIM register. Default= '100'.
pub type RFD_REG_TRIM_R = crate::FieldReader;
///Field `SPARE` reader -
pub type SPARE_R = crate::BitReader;
///Field `TRIM_MR` reader - TRIM_MR\[3:0\]: Main Regulator Voltage Trimming By default, this value is taken from the engi bytes; and saved on V12o domain when OBL done. if associated ENGTRIM.TRIMMREN is enabled the Main Regulator Voltage can be controlled by the dedicated ENGTRIM.TRIM_MR register. Default= '0000'.
pub type TRIM_MR_R = crate::FieldReader;
///Field `SMPS_TRIM` reader - SMPS_TRIM\[2:0\]: SMPS Output Voltage Trimming By default, this value is taken from the engi bytes; and saved on V12o domain when OBL done. if associated ENGTRIM is enabled the SMPS output voltage can be controlled by the dedicated ENGTRIM register. Default= '011'.
pub type SMPS_TRIM_R = crate::FieldReader;
///Field `BOF_TRIM` reader - BOF_TRIM\[2:0\]: Bypass On the Fly Output Voltage Trimming By default, this value is taken from the engi bytes; and saved on V12o domain when OBL done. if associated ENGTRIM is enabled the SMPS output voltage can be controlled by the dedicated ENGTRIM register. Default= '100'.
pub type BOF_TRIM_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - RFD_REG_TRIM\[2:0\]: RF LDO Trimming By default, this value is taken from the engi bytes; and saved on V12o domain when OBL done. if associated ENGTRIM is enabled the RF LDO trimming can be controlled by the dedicated ENGTRIM register. Default= '100'.
    #[inline(always)]
    pub fn rfd_reg_trim(&self) -> RFD_REG_TRIM_R {
        RFD_REG_TRIM_R::new((self.bits & 7) as u8)
    }
    ///Bit 3
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - TRIM_MR\[3:0\]: Main Regulator Voltage Trimming By default, this value is taken from the engi bytes; and saved on V12o domain when OBL done. if associated ENGTRIM.TRIMMREN is enabled the Main Regulator Voltage can be controlled by the dedicated ENGTRIM.TRIM_MR register. Default= '0000'.
    #[inline(always)]
    pub fn trim_mr(&self) -> TRIM_MR_R {
        TRIM_MR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:10 - SMPS_TRIM\[2:0\]: SMPS Output Voltage Trimming By default, this value is taken from the engi bytes; and saved on V12o domain when OBL done. if associated ENGTRIM is enabled the SMPS output voltage can be controlled by the dedicated ENGTRIM register. Default= '011'.
    #[inline(always)]
    pub fn smps_trim(&self) -> SMPS_TRIM_R {
        SMPS_TRIM_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:13 - BOF_TRIM\[2:0\]: Bypass On the Fly Output Voltage Trimming By default, this value is taken from the engi bytes; and saved on V12o domain when OBL done. if associated ENGTRIM is enabled the SMPS output voltage can be controlled by the dedicated ENGTRIM register. Default= '100'.
    #[inline(always)]
    pub fn bof_trim(&self) -> BOF_TRIM_R {
        BOF_TRIM_R::new(((self.bits >> 11) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRIMR")
            .field("rfd_reg_trim", &self.rfd_reg_trim())
            .field("spare", &self.spare())
            .field("trim_mr", &self.trim_mr())
            .field("smps_trim", &self.smps_trim())
            .field("bof_trim", &self.bof_trim())
            .finish()
    }
}
/**TRIMR register

You can [`read`](crate::Reg::read) this register and get [`trimr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:TRIMR)*/
pub struct TRIMRrs;
impl crate::RegisterSpec for TRIMRrs {
    type Ux = u32;
}
///`read()` method returns [`trimr::R`](R) reader structure
impl crate::Readable for TRIMRrs {}
///`reset()` method sets TRIMR to value 0x2304
impl crate::Resettable for TRIMRrs {
    const RESET_VALUE: u32 = 0x2304;
}
