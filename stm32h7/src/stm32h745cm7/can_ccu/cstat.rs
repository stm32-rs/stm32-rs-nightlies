///Register `CSTAT` reader
pub type R = crate::R<CSTATrs>;
///Register `CSTAT` writer
pub type W = crate::W<CSTATrs>;
///Field `OCPC` reader - Oscillator Clock Period Counter
pub type OCPC_R = crate::FieldReader<u32>;
///Field `OCPC` writer - Oscillator Clock Period Counter
pub type OCPC_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
///Field `TQC` reader - Time Quanta Counter
pub type TQC_R = crate::FieldReader<u16>;
///Field `TQC` writer - Time Quanta Counter
pub type TQC_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `CALS` reader - Calibration State
pub type CALS_R = crate::FieldReader;
///Field `CALS` writer - Calibration State
pub type CALS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:17 - Oscillator Clock Period Counter
    #[inline(always)]
    pub fn ocpc(&self) -> OCPC_R {
        OCPC_R::new(self.bits & 0x0003_ffff)
    }
    ///Bits 18:28 - Time Quanta Counter
    #[inline(always)]
    pub fn tqc(&self) -> TQC_R {
        TQC_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    ///Bits 30:31 - Calibration State
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
impl W {
    ///Bits 0:17 - Oscillator Clock Period Counter
    #[inline(always)]
    pub fn ocpc(&mut self) -> OCPC_W<'_, CSTATrs> {
        OCPC_W::new(self, 0)
    }
    ///Bits 18:28 - Time Quanta Counter
    #[inline(always)]
    pub fn tqc(&mut self) -> TQC_W<'_, CSTATrs> {
        TQC_W::new(self, 18)
    }
    ///Bits 30:31 - Calibration State
    #[inline(always)]
    pub fn cals(&mut self) -> CALS_W<'_, CSTATrs> {
        CALS_W::new(self, 30)
    }
}
/**Calibration Status Register

You can [`read`](crate::Reg::read) this register and get [`cstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM7.html#CAN_CCU:CSTAT)*/
pub struct CSTATrs;
impl crate::RegisterSpec for CSTATrs {
    type Ux = u32;
}
///`read()` method returns [`cstat::R`](R) reader structure
impl crate::Readable for CSTATrs {}
///`write(|w| ..)` method takes [`cstat::W`](W) writer structure
impl crate::Writable for CSTATrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSTAT to value 0
impl crate::Resettable for CSTATrs {}
