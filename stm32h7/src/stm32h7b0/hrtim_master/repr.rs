///Register `REPR` reader
pub type R = crate::R<REPRrs>;
///Register `REPR` writer
pub type W = crate::W<REPRrs>;
///Field `REP` reader - Master Timer Repetition counter value
pub type REP_R = crate::FieldReader;
///Field `REP` writer - Master Timer Repetition counter value
pub type REP_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:7 - Master Timer Repetition counter value
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REPR").field("rep", &self.rep()).finish()
    }
}
impl W {
    ///Bits 0:7 - Master Timer Repetition counter value
    #[inline(always)]
    pub fn rep(&mut self) -> REP_W<'_, REPRrs> {
        REP_W::new(self, 0)
    }
}
/**Master Timer Repetition Register

You can [`read`](crate::Reg::read) this register and get [`repr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`repr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#HRTIM_Master:REPR)*/
pub struct REPRrs;
impl crate::RegisterSpec for REPRrs {
    type Ux = u32;
}
///`read()` method returns [`repr::R`](R) reader structure
impl crate::Readable for REPRrs {}
///`write(|w| ..)` method takes [`repr::W`](W) writer structure
impl crate::Writable for REPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REPR to value 0
impl crate::Resettable for REPRrs {}
