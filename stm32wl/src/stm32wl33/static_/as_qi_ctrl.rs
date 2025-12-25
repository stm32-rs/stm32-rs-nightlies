///Register `AS_QI_CTRL` reader
pub type R = crate::R<AS_QI_CTRLrs>;
///Register `AS_QI_CTRL` writer
pub type W = crate::W<AS_QI_CTRLrs>;
///Field `RSSI_THR` reader - Signal detect threshold in 1 dB resolution.
pub type RSSI_THR_R = crate::FieldReader<u16>;
///Field `RSSI_THR` writer - Signal detect threshold in 1 dB resolution.
pub type RSSI_THR_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `PQI_THR` reader - PQI threshold (if 0 then ).
pub type PQI_THR_R = crate::FieldReader;
///Field `PQI_THR` writer - PQI threshold (if 0 then ).
pub type PQI_THR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CS_MODE` reader - Carrier Sense mode selection
pub type CS_MODE_R = crate::FieldReader;
///Field `CS_MODE` writer - Carrier Sense mode selection
pub type CS_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SQI_EN` reader - SQI enable
pub type SQI_EN_R = crate::BitReader;
///Field `SQI_EN` writer - SQI enable
pub type SQI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SQI_THR` reader - SQI threshold defining the precision requested to detect the SYNC word.
pub type SQI_THR_R = crate::FieldReader;
///Field `SQI_THR` writer - SQI threshold defining the precision requested to detect the SYNC word.
pub type SQI_THR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `AS_EQU_CTRL` reader - ISI cancellation equalizer
pub type AS_EQU_CTRL_R = crate::FieldReader;
///Field `AS_EQU_CTRL` writer - ISI cancellation equalizer
pub type AS_EQU_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `AS_MEAS_TIME` reader - Select the RSSI measurement duration during Antenna switching procedure
pub type AS_MEAS_TIME_R = crate::FieldReader;
///Field `AS_MEAS_TIME` writer - Select the RSSI measurement duration during Antenna switching procedure
pub type AS_MEAS_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `AS_CS_BLANKING` reader - Blank received data if signal is below the CS threshold
pub type AS_CS_BLANKING_R = crate::BitReader;
///Field `AS_CS_BLANKING` writer - Blank received data if signal is below the CS threshold
pub type AS_CS_BLANKING_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:8 - Signal detect threshold in 1 dB resolution.
    #[inline(always)]
    pub fn rssi_thr(&self) -> RSSI_THR_R {
        RSSI_THR_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:12 - PQI threshold (if 0 then ).
    #[inline(always)]
    pub fn pqi_thr(&self) -> PQI_THR_R {
        PQI_THR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    ///Bits 13:14 - Carrier Sense mode selection
    #[inline(always)]
    pub fn cs_mode(&self) -> CS_MODE_R {
        CS_MODE_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - SQI enable
    #[inline(always)]
    pub fn sqi_en(&self) -> SQI_EN_R {
        SQI_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:18 - SQI threshold defining the precision requested to detect the SYNC word.
    #[inline(always)]
    pub fn sqi_thr(&self) -> SQI_THR_R {
        SQI_THR_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 26:27 - ISI cancellation equalizer
    #[inline(always)]
    pub fn as_equ_ctrl(&self) -> AS_EQU_CTRL_R {
        AS_EQU_CTRL_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:30 - Select the RSSI measurement duration during Antenna switching procedure
    #[inline(always)]
    pub fn as_meas_time(&self) -> AS_MEAS_TIME_R {
        AS_MEAS_TIME_R::new(((self.bits >> 28) & 7) as u8)
    }
    ///Bit 31 - Blank received data if signal is below the CS threshold
    #[inline(always)]
    pub fn as_cs_blanking(&self) -> AS_CS_BLANKING_R {
        AS_CS_BLANKING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AS_QI_CTRL")
            .field("rssi_thr", &self.rssi_thr())
            .field("pqi_thr", &self.pqi_thr())
            .field("cs_mode", &self.cs_mode())
            .field("sqi_en", &self.sqi_en())
            .field("sqi_thr", &self.sqi_thr())
            .field("as_equ_ctrl", &self.as_equ_ctrl())
            .field("as_meas_time", &self.as_meas_time())
            .field("as_cs_blanking", &self.as_cs_blanking())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Signal detect threshold in 1 dB resolution.
    #[inline(always)]
    pub fn rssi_thr(&mut self) -> RSSI_THR_W<'_, AS_QI_CTRLrs> {
        RSSI_THR_W::new(self, 0)
    }
    ///Bits 9:12 - PQI threshold (if 0 then ).
    #[inline(always)]
    pub fn pqi_thr(&mut self) -> PQI_THR_W<'_, AS_QI_CTRLrs> {
        PQI_THR_W::new(self, 9)
    }
    ///Bits 13:14 - Carrier Sense mode selection
    #[inline(always)]
    pub fn cs_mode(&mut self) -> CS_MODE_W<'_, AS_QI_CTRLrs> {
        CS_MODE_W::new(self, 13)
    }
    ///Bit 15 - SQI enable
    #[inline(always)]
    pub fn sqi_en(&mut self) -> SQI_EN_W<'_, AS_QI_CTRLrs> {
        SQI_EN_W::new(self, 15)
    }
    ///Bits 16:18 - SQI threshold defining the precision requested to detect the SYNC word.
    #[inline(always)]
    pub fn sqi_thr(&mut self) -> SQI_THR_W<'_, AS_QI_CTRLrs> {
        SQI_THR_W::new(self, 16)
    }
    ///Bits 26:27 - ISI cancellation equalizer
    #[inline(always)]
    pub fn as_equ_ctrl(&mut self) -> AS_EQU_CTRL_W<'_, AS_QI_CTRLrs> {
        AS_EQU_CTRL_W::new(self, 26)
    }
    ///Bits 28:30 - Select the RSSI measurement duration during Antenna switching procedure
    #[inline(always)]
    pub fn as_meas_time(&mut self) -> AS_MEAS_TIME_W<'_, AS_QI_CTRLrs> {
        AS_MEAS_TIME_W::new(self, 28)
    }
    ///Bit 31 - Blank received data if signal is below the CS threshold
    #[inline(always)]
    pub fn as_cs_blanking(&mut self) -> AS_CS_BLANKING_W<'_, AS_QI_CTRLrs> {
        AS_CS_BLANKING_W::new(self, 31)
    }
}
/**AS_QI_CTRL register

You can [`read`](crate::Reg::read) this register and get [`as_qi_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`as_qi_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:AS_QI_CTRL)*/
pub struct AS_QI_CTRLrs;
impl crate::RegisterSpec for AS_QI_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`as_qi_ctrl::R`](R) reader structure
impl crate::Readable for AS_QI_CTRLrs {}
///`write(|w| ..)` method takes [`as_qi_ctrl::W`](W) writer structure
impl crate::Writable for AS_QI_CTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AS_QI_CTRL to value 0x5800_8028
impl crate::Resettable for AS_QI_CTRLrs {
    const RESET_VALUE: u32 = 0x5800_8028;
}
