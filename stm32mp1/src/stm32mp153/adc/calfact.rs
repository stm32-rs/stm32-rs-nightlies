///Register `CALFACT` reader
pub type R = crate::R<CALFACTrs>;
///Register `CALFACT` writer
pub type W = crate::W<CALFACTrs>;
///Field `CALFACT_S` reader - CALFACT_S
pub type CALFACT_S_R = crate::FieldReader<u16>;
///Field `CALFACT_S` writer - CALFACT_S
pub type CALFACT_S_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `CALFACT_D` reader - CALFACT_D
pub type CALFACT_D_R = crate::FieldReader<u16>;
///Field `CALFACT_D` writer - CALFACT_D
pub type CALFACT_D_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - CALFACT_S
    #[inline(always)]
    pub fn calfact_s(&self) -> CALFACT_S_R {
        CALFACT_S_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - CALFACT_D
    #[inline(always)]
    pub fn calfact_d(&self) -> CALFACT_D_R {
        CALFACT_D_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALFACT")
            .field("calfact_s", &self.calfact_s())
            .field("calfact_d", &self.calfact_d())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - CALFACT_S
    #[inline(always)]
    pub fn calfact_s(&mut self) -> CALFACT_S_W<'_, CALFACTrs> {
        CALFACT_S_W::new(self, 0)
    }
    ///Bits 16:26 - CALFACT_D
    #[inline(always)]
    pub fn calfact_d(&mut self) -> CALFACT_D_W<'_, CALFACTrs> {
        CALFACT_D_W::new(self, 16)
    }
}
/**ADC calibration factors register

You can [`read`](crate::Reg::read) this register and get [`calfact::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfact::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ADC:CALFACT)*/
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
