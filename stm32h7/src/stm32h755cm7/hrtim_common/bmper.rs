///Register `BMPER` reader
pub type R = crate::R<BMPERrs>;
///Register `BMPER` writer
pub type W = crate::W<BMPERrs>;
///Field `BMPER` reader - Burst mode Period
pub type BMPER_R = crate::FieldReader<u16>;
///Field `BMPER` writer - Burst mode Period
pub type BMPER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Burst mode Period
    #[inline(always)]
    pub fn bmper(&self) -> BMPER_R {
        BMPER_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BMPER")
            .field("bmper", &self.bmper())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Burst mode Period
    #[inline(always)]
    pub fn bmper(&mut self) -> BMPER_W<'_, BMPERrs> {
        BMPER_W::new(self, 0)
    }
}
/**Burst Mode Period Register

You can [`read`](crate::Reg::read) this register and get [`bmper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#HRTIM_Common:BMPER)*/
pub struct BMPERrs;
impl crate::RegisterSpec for BMPERrs {
    type Ux = u32;
}
///`read()` method returns [`bmper::R`](R) reader structure
impl crate::Readable for BMPERrs {}
///`write(|w| ..)` method takes [`bmper::W`](W) writer structure
impl crate::Writable for BMPERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BMPER to value 0
impl crate::Resettable for BMPERrs {}
