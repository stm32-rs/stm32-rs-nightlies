///Register `PERR` reader
pub type R = crate::R<PERRrs>;
///Register `PERR` writer
pub type W = crate::W<PERRrs>;
///Field `PER` reader - Master Timer Period value
pub type PER_R = crate::FieldReader<u16>;
///Field `PER` writer - Master Timer Period value
pub type PER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Master Timer Period value
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERR").field("per", &self.per()).finish()
    }
}
impl W {
    ///Bits 0:15 - Master Timer Period value
    #[inline(always)]
    pub fn per(&mut self) -> PER_W<'_, PERRrs> {
        PER_W::new(self, 0)
    }
}
/**Master Timer Period Register

You can [`read`](crate::Reg::read) this register and get [`perr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#HRTIM_Master:PERR)*/
pub struct PERRrs;
impl crate::RegisterSpec for PERRrs {
    type Ux = u32;
}
///`read()` method returns [`perr::R`](R) reader structure
impl crate::Readable for PERRrs {}
///`write(|w| ..)` method takes [`perr::W`](W) writer structure
impl crate::Writable for PERRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PERR to value 0xffff
impl crate::Resettable for PERRrs {
    const RESET_VALUE: u32 = 0xffff;
}
