///Register `CALFACT` reader
pub type R = crate::R<CALFACTrs>;
///Register `CALFACT` writer
pub type W = crate::W<CALFACTrs>;
///Field `CALFACT_S` reader - Calibration factors In Single-ended mode
pub type CALFACT_S_R = crate::FieldReader<u16>;
///Field `CALFACT_S` writer - Calibration factors In Single-ended mode
pub type CALFACT_S_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `CALFACT_D` reader - Calibration factors in Differential mode
pub type CALFACT_D_R = crate::FieldReader<u16>;
///Field `CALFACT_D` writer - Calibration factors in Differential mode
pub type CALFACT_D_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `CALADDOS` reader - Calibration additional offset
pub type CALADDOS_R = crate::BitReader;
///Field `CALADDOS` writer - Calibration additional offset
pub type CALADDOS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:8 - Calibration factors In Single-ended mode
    #[inline(always)]
    pub fn calfact_s(&self) -> CALFACT_S_R {
        CALFACT_S_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Calibration factors in Differential mode
    #[inline(always)]
    pub fn calfact_d(&self) -> CALFACT_D_R {
        CALFACT_D_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    ///Bit 31 - Calibration additional offset
    #[inline(always)]
    pub fn caladdos(&self) -> CALADDOS_R {
        CALADDOS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALFACT")
            .field("calfact_s", &self.calfact_s())
            .field("calfact_d", &self.calfact_d())
            .field("caladdos", &self.caladdos())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Calibration factors In Single-ended mode
    #[inline(always)]
    pub fn calfact_s(&mut self) -> CALFACT_S_W<CALFACTrs> {
        CALFACT_S_W::new(self, 0)
    }
    ///Bits 16:24 - Calibration factors in Differential mode
    #[inline(always)]
    pub fn calfact_d(&mut self) -> CALFACT_D_W<CALFACTrs> {
        CALFACT_D_W::new(self, 16)
    }
    ///Bit 31 - Calibration additional offset
    #[inline(always)]
    pub fn caladdos(&mut self) -> CALADDOS_W<CALFACTrs> {
        CALADDOS_W::new(self, 31)
    }
}
/**ADC calibration factors

You can [`read`](crate::Reg::read) this register and get [`calfact::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfact::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:CALFACT)*/
pub struct CALFACTrs;
impl crate::RegisterSpec for CALFACTrs {
    type Ux = u32;
}
///`read()` method returns [`calfact::R`](R) reader structure
impl crate::Readable for CALFACTrs {}
///`write(|w| ..)` method takes [`calfact::W`](W) writer structure
impl crate::Writable for CALFACTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CALFACT to value 0
impl crate::Resettable for CALFACTrs {}
