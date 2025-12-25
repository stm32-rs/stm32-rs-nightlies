///Register `MACPPSI1R` reader
pub type R = crate::R<MACPPSI1Rrs>;
///Register `MACPPSI1R` writer
pub type W = crate::W<MACPPSI1Rrs>;
///Field `PPSINT0` reader - PPS Output Signal Interval
pub type PPSINT0_R = crate::FieldReader<u32>;
///Field `PPSINT0` writer - PPS Output Signal Interval
pub type PPSINT0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - PPS Output Signal Interval
    #[inline(always)]
    pub fn ppsint0(&self) -> PPSINT0_R {
        PPSINT0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACPPSI1R")
            .field("ppsint0", &self.ppsint0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - PPS Output Signal Interval
    #[inline(always)]
    pub fn ppsint0(&mut self) -> PPSINT0_W<'_, MACPPSI1Rrs> {
        PPSINT0_W::new(self, 0)
    }
}
/**PPS 1 interval register

You can [`read`](crate::Reg::read) this register and get [`macppsi1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsi1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ETH:MACPPSI1R)*/
pub struct MACPPSI1Rrs;
impl crate::RegisterSpec for MACPPSI1Rrs {
    type Ux = u32;
}
///`read()` method returns [`macppsi1r::R`](R) reader structure
impl crate::Readable for MACPPSI1Rrs {}
///`write(|w| ..)` method takes [`macppsi1r::W`](W) writer structure
impl crate::Writable for MACPPSI1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACPPSI1R to value 0
impl crate::Resettable for MACPPSI1Rrs {}
