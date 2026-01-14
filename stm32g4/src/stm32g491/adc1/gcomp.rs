///Register `GCOMP` reader
pub type R = crate::R<GCOMPrs>;
///Register `GCOMP` writer
pub type W = crate::W<GCOMPrs>;
///Field `GCOMPCOEFF` reader - Gain compensation coefficient
pub type GCOMPCOEFF_R = crate::FieldReader<u16>;
///Field `GCOMPCOEFF` writer - Gain compensation coefficient
pub type GCOMPCOEFF_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bits 0:13 - Gain compensation coefficient
    #[inline(always)]
    pub fn gcompcoeff(&self) -> GCOMPCOEFF_R {
        GCOMPCOEFF_R::new((self.bits & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCOMP")
            .field("gcompcoeff", &self.gcompcoeff())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - Gain compensation coefficient
    #[inline(always)]
    pub fn gcompcoeff(&mut self) -> GCOMPCOEFF_W<'_, GCOMPrs> {
        GCOMPCOEFF_W::new(self, 0)
    }
}
/**Gain compensation Register

You can [`read`](crate::Reg::read) this register and get [`gcomp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcomp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G491.html#ADC1:GCOMP)*/
pub struct GCOMPrs;
impl crate::RegisterSpec for GCOMPrs {
    type Ux = u32;
}
///`read()` method returns [`gcomp::R`](R) reader structure
impl crate::Readable for GCOMPrs {}
///`write(|w| ..)` method takes [`gcomp::W`](W) writer structure
impl crate::Writable for GCOMPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GCOMP to value 0
impl crate::Resettable for GCOMPrs {}
