///Register `HYSCR4` reader
pub type R = crate::R<HYSCR4rs>;
///Register `HYSCR4` writer
pub type W = crate::W<HYSCR4rs>;
///Field `PG` reader - Port G hysteresis control on/off
pub type PG_R = crate::FieldReader<u16>;
///Field `PG` writer - Port G hysteresis control on/off
pub type PG_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Port G hysteresis control on/off
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HYSCR4").field("pg", &self.pg()).finish()
    }
}
impl W {
    ///Bits 0:15 - Port G hysteresis control on/off
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W<'_, HYSCR4rs> {
        PG_W::new(self, 0)
    }
}
/**Hysteresis control register

You can [`read`](crate::Reg::read) this register and get [`hyscr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hyscr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:HYSCR4)*/
pub struct HYSCR4rs;
impl crate::RegisterSpec for HYSCR4rs {
    type Ux = u32;
}
///`read()` method returns [`hyscr4::R`](R) reader structure
impl crate::Readable for HYSCR4rs {}
///`write(|w| ..)` method takes [`hyscr4::W`](W) writer structure
impl crate::Writable for HYSCR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HYSCR4 to value 0
impl crate::Resettable for HYSCR4rs {}
