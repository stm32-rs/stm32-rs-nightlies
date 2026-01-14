///Register `MACPPSI0R` reader
pub type R = crate::R<MACPPSI0Rrs>;
///Register `MACPPSI0R` writer
pub type W = crate::W<MACPPSI0Rrs>;
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
        f.debug_struct("MACPPSI0R")
            .field("ppsint0", &self.ppsint0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - PPS Output Signal Interval
    #[inline(always)]
    pub fn ppsint0(&mut self) -> PPSINT0_W<'_, MACPPSI0Rrs> {
        PPSINT0_W::new(self, 0)
    }
}
/**PPS 0 interval register

You can [`read`](crate::Reg::read) this register and get [`macppsi0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsi0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACPPSI0R)*/
pub struct MACPPSI0Rrs;
impl crate::RegisterSpec for MACPPSI0Rrs {
    type Ux = u32;
}
///`read()` method returns [`macppsi0r::R`](R) reader structure
impl crate::Readable for MACPPSI0Rrs {}
///`write(|w| ..)` method takes [`macppsi0r::W`](W) writer structure
impl crate::Writable for MACPPSI0Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACPPSI0R to value 0
impl crate::Resettable for MACPPSI0Rrs {}
