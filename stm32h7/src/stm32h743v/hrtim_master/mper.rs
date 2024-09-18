///Register `MPER` reader
pub type R = crate::R<MPERrs>;
///Register `MPER` writer
pub type W = crate::W<MPERrs>;
///Field `MPER` reader - Master Timer Period value
pub type MPER_R = crate::FieldReader<u16>;
///Field `MPER` writer - Master Timer Period value
pub type MPER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Master Timer Period value
    #[inline(always)]
    pub fn mper(&self) -> MPER_R {
        MPER_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPER").field("mper", &self.mper()).finish()
    }
}
impl W {
    ///Bits 0:15 - Master Timer Period value
    #[inline(always)]
    #[must_use]
    pub fn mper(&mut self) -> MPER_W<MPERrs> {
        MPER_W::new(self, 0)
    }
}
/**Master Timer Period Register

You can [`read`](crate::Reg::read) this register and get [`mper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_Master:MPER)*/
pub struct MPERrs;
impl crate::RegisterSpec for MPERrs {
    type Ux = u32;
}
///`read()` method returns [`mper::R`](R) reader structure
impl crate::Readable for MPERrs {}
///`write(|w| ..)` method takes [`mper::W`](W) writer structure
impl crate::Writable for MPERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MPER to value 0xffff
impl crate::Resettable for MPERrs {
    const RESET_VALUE: u32 = 0xffff;
}
