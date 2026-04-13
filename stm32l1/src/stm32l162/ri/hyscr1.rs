///Register `HYSCR1` reader
pub type R = crate::R<HYSCR1rs>;
///Register `HYSCR1` writer
pub type W = crate::W<HYSCR1rs>;
///Field `PA` reader - Port A hysteresis control on/off
pub type PA_R = crate::FieldReader<u16>;
///Field `PA` writer - Port A hysteresis control on/off
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `PB` reader - Port B hysteresis control on/off
pub type PB_R = crate::FieldReader<u16>;
///Field `PB` writer - Port B hysteresis control on/off
pub type PB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Port A hysteresis control on/off
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Port B hysteresis control on/off
    #[inline(always)]
    pub fn pb(&self) -> PB_R {
        PB_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HYSCR1")
            .field("pb", &self.pb())
            .field("pa", &self.pa())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Port A hysteresis control on/off
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W<'_, HYSCR1rs> {
        PA_W::new(self, 0)
    }
    ///Bits 16:31 - Port B hysteresis control on/off
    #[inline(always)]
    pub fn pb(&mut self) -> PB_W<'_, HYSCR1rs> {
        PB_W::new(self, 16)
    }
}
/**RI hysteresis control register 1

You can [`read`](crate::Reg::read) this register and get [`hyscr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hyscr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#RI:HYSCR1)*/
pub struct HYSCR1rs;
impl crate::RegisterSpec for HYSCR1rs {
    type Ux = u32;
}
///`read()` method returns [`hyscr1::R`](R) reader structure
impl crate::Readable for HYSCR1rs {}
///`write(|w| ..)` method takes [`hyscr1::W`](W) writer structure
impl crate::Writable for HYSCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HYSCR1 to value 0
impl crate::Resettable for HYSCR1rs {}
