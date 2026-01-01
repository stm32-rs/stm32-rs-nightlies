///Register `P1BPRCR` reader
pub type R = crate::R<P1BPRCRrs>;
///Register `P1BPRCR` writer
pub type W = crate::W<P1BPRCRrs>;
///Field `ENABLE` reader - Bad pixel detection must be enabled only for raw Bayer flows, as it corrupts RGB flows.
pub type ENABLE_R = crate::BitReader;
///Field `ENABLE` writer - Bad pixel detection must be enabled only for raw Bayer flows, as it corrupts RGB flows.
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STRENGTH` reader - Strength (aggressiveness) of the bad pixel detection
pub type STRENGTH_R = crate::FieldReader;
///Field `STRENGTH` writer - Strength (aggressiveness) of the bad pixel detection
pub type STRENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - Bad pixel detection must be enabled only for raw Bayer flows, as it corrupts RGB flows.
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - Strength (aggressiveness) of the bad pixel detection
    #[inline(always)]
    pub fn strength(&self) -> STRENGTH_R {
        STRENGTH_R::new(((self.bits >> 1) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1BPRCR")
            .field("enable", &self.enable())
            .field("strength", &self.strength())
            .finish()
    }
}
impl W {
    ///Bit 0 - Bad pixel detection must be enabled only for raw Bayer flows, as it corrupts RGB flows.
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<'_, P1BPRCRrs> {
        ENABLE_W::new(self, 0)
    }
    ///Bits 1:3 - Strength (aggressiveness) of the bad pixel detection
    #[inline(always)]
    pub fn strength(&mut self) -> STRENGTH_W<'_, P1BPRCRrs> {
        STRENGTH_W::new(self, 1)
    }
}
/**DCMIPP Pipe1 bad pixel removal control register

You can [`read`](crate::Reg::read) this register and get [`p1bprcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1bprcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P1BPRCR)*/
pub struct P1BPRCRrs;
impl crate::RegisterSpec for P1BPRCRrs {
    type Ux = u32;
}
///`read()` method returns [`p1bprcr::R`](R) reader structure
impl crate::Readable for P1BPRCRrs {}
///`write(|w| ..)` method takes [`p1bprcr::W`](W) writer structure
impl crate::Writable for P1BPRCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1BPRCR to value 0
impl crate::Resettable for P1BPRCRrs {}
