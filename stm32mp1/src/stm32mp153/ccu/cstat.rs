///Register `CSTAT` reader
pub type R = crate::R<CSTATrs>;
///Field `OCPC` reader - OCPC
pub type OCPC_R = crate::FieldReader<u32>;
///Field `TQC` reader - TQC
pub type TQC_R = crate::FieldReader<u16>;
///Field `CALS` reader - CALS
pub type CALS_R = crate::FieldReader;
impl R {
    ///Bits 0:17 - OCPC
    #[inline(always)]
    pub fn ocpc(&self) -> OCPC_R {
        OCPC_R::new(self.bits & 0x0003_ffff)
    }
    ///Bits 18:28 - TQC
    #[inline(always)]
    pub fn tqc(&self) -> TQC_R {
        TQC_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    ///Bits 30:31 - CALS
    #[inline(always)]
    pub fn cals(&self) -> CALS_R {
        CALS_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSTAT")
            .field("ocpc", &self.ocpc())
            .field("tqc", &self.tqc())
            .field("cals", &self.cals())
            .finish()
    }
}
/**Calibration status register

You can [`read`](crate::Reg::read) this register and get [`cstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#CCU:CSTAT)*/
pub struct CSTATrs;
impl crate::RegisterSpec for CSTATrs {
    type Ux = u32;
}
///`read()` method returns [`cstat::R`](R) reader structure
impl crate::Readable for CSTATrs {}
///`reset()` method sets CSTAT to value 0x0203_ffff
impl crate::Resettable for CSTATrs {
    const RESET_VALUE: u32 = 0x0203_ffff;
}
