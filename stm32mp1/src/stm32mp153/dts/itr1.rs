///Register `ITR1` reader
pub type R = crate::R<ITR1rs>;
///Register `ITR1` writer
pub type W = crate::W<ITR1rs>;
///Field `TS1_LITTHD` reader - TS1_LITTHD
pub type TS1_LITTHD_R = crate::FieldReader<u16>;
///Field `TS1_LITTHD` writer - TS1_LITTHD
pub type TS1_LITTHD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `TS1_HITTHD` reader - TS1_HITTHD
pub type TS1_HITTHD_R = crate::FieldReader<u16>;
///Field `TS1_HITTHD` writer - TS1_HITTHD
pub type TS1_HITTHD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - TS1_LITTHD
    #[inline(always)]
    pub fn ts1_litthd(&self) -> TS1_LITTHD_R {
        TS1_LITTHD_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - TS1_HITTHD
    #[inline(always)]
    pub fn ts1_hitthd(&self) -> TS1_HITTHD_R {
        TS1_HITTHD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITR1")
            .field("ts1_litthd", &self.ts1_litthd())
            .field("ts1_hitthd", &self.ts1_hitthd())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - TS1_LITTHD
    #[inline(always)]
    pub fn ts1_litthd(&mut self) -> TS1_LITTHD_W<ITR1rs> {
        TS1_LITTHD_W::new(self, 0)
    }
    ///Bits 16:31 - TS1_HITTHD
    #[inline(always)]
    pub fn ts1_hitthd(&mut self) -> TS1_HITTHD_W<ITR1rs> {
        TS1_HITTHD_W::new(self, 16)
    }
}
/**DTS_ITR1 contains the threshold values for sensor 1.

You can [`read`](crate::Reg::read) this register and get [`itr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DTS:ITR1)*/
pub struct ITR1rs;
impl crate::RegisterSpec for ITR1rs {
    type Ux = u32;
}
///`read()` method returns [`itr1::R`](R) reader structure
impl crate::Readable for ITR1rs {}
///`write(|w| ..)` method takes [`itr1::W`](W) writer structure
impl crate::Writable for ITR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ITR1 to value 0
impl crate::Resettable for ITR1rs {}
