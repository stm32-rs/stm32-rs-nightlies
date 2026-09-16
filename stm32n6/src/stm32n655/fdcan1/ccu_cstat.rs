///Register `CCU_CSTAT` reader
pub type R = crate::R<CCU_CSTATrs>;
///Field `OCPC` reader - Oscillator clock period counter
pub type OCPC_R = crate::FieldReader<u32>;
///Field `TQC` reader - Time quanta counter
pub type TQC_R = crate::FieldReader<u16>;
///Field `CALS` reader - Calibration state
pub type CALS_R = crate::FieldReader;
impl R {
    ///Bits 0:17 - Oscillator clock period counter
    #[inline(always)]
    pub fn ocpc(&self) -> OCPC_R {
        OCPC_R::new(self.bits & 0x0003_ffff)
    }
    ///Bits 18:28 - Time quanta counter
    #[inline(always)]
    pub fn tqc(&self) -> TQC_R {
        TQC_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    ///Bits 30:31 - Calibration state
    #[inline(always)]
    pub fn cals(&self) -> CALS_R {
        CALS_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCU_CSTAT")
            .field("ocpc", &self.ocpc())
            .field("tqc", &self.tqc())
            .field("cals", &self.cals())
            .finish()
    }
}
/**Calibration status register

You can [`read`](crate::Reg::read) this register and get [`ccu_cstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#FDCAN1:CCU_CSTAT)*/
pub struct CCU_CSTATrs;
impl crate::RegisterSpec for CCU_CSTATrs {
    type Ux = u32;
}
///`read()` method returns [`ccu_cstat::R`](R) reader structure
impl crate::Readable for CCU_CSTATrs {}
///`reset()` method sets CCU_CSTAT to value 0x0203_ffff
impl crate::Resettable for CCU_CSTATrs {
    const RESET_VALUE: u32 = 0x0203_ffff;
}
