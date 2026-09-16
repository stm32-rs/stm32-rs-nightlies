///Register `DR` reader
pub type R = crate::R<DRrs>;
///Register `DR` writer
pub type W = crate::W<DRrs>;
///Field `TS1_MFREQ` reader - Value of the counter output value for temperature sensor 1
pub type TS1_MFREQ_R = crate::FieldReader<u16>;
///Field `TS1_MFREQ` writer - Value of the counter output value for temperature sensor 1
pub type TS1_MFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Value of the counter output value for temperature sensor 1
    #[inline(always)]
    pub fn ts1_mfreq(&self) -> TS1_MFREQ_R {
        TS1_MFREQ_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR")
            .field("ts1_mfreq", &self.ts1_mfreq())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Value of the counter output value for temperature sensor 1
    #[inline(always)]
    pub fn ts1_mfreq(&mut self) -> TS1_MFREQ_W<'_, DRrs> {
        TS1_MFREQ_W::new(self, 0)
    }
}
/**Temperature sensor data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#DTS:DR)*/
pub struct DRrs;
impl crate::RegisterSpec for DRrs {
    type Ux = u32;
}
///`read()` method returns [`dr::R`](R) reader structure
impl crate::Readable for DRrs {}
///`write(|w| ..)` method takes [`dr::W`](W) writer structure
impl crate::Writable for DRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DR to value 0
impl crate::Resettable for DRrs {}
