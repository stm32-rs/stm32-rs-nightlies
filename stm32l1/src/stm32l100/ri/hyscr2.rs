///Register `HYSCR2` reader
pub type R = crate::R<HYSCR2rs>;
///Register `HYSCR2` writer
pub type W = crate::W<HYSCR2rs>;
///Field `PC` reader - Port C hysteresis control on/off
pub type PC_R = crate::FieldReader<u16>;
///Field `PC` writer - Port C hysteresis control on/off
pub type PC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `PD` reader - Port D hysteresis control on/off
pub type PD_R = crate::FieldReader<u16>;
///Field `PD` writer - Port D hysteresis control on/off
pub type PD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Port C hysteresis control on/off
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Port D hysteresis control on/off
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HYSCR2")
            .field("pd", &self.pd())
            .field("pc", &self.pc())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Port C hysteresis control on/off
    #[inline(always)]
    pub fn pc(&mut self) -> PC_W<'_, HYSCR2rs> {
        PC_W::new(self, 0)
    }
    ///Bits 16:31 - Port D hysteresis control on/off
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W<'_, HYSCR2rs> {
        PD_W::new(self, 16)
    }
}
/**RI hysteresis control register 2

You can [`read`](crate::Reg::read) this register and get [`hyscr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hyscr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:HYSCR2)*/
pub struct HYSCR2rs;
impl crate::RegisterSpec for HYSCR2rs {
    type Ux = u32;
}
///`read()` method returns [`hyscr2::R`](R) reader structure
impl crate::Readable for HYSCR2rs {}
///`write(|w| ..)` method takes [`hyscr2::W`](W) writer structure
impl crate::Writable for HYSCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HYSCR2 to value 0
impl crate::Resettable for HYSCR2rs {}
