///Register `MACPPSTTS1R` reader
pub type R = crate::R<MACPPSTTS1Rrs>;
///Register `MACPPSTTS1R` writer
pub type W = crate::W<MACPPSTTS1Rrs>;
///Field `TSTRH0` reader - PPS Target Time Seconds Register
pub type TSTRH0_R = crate::FieldReader<u32>;
///Field `TSTRH0` writer - PPS Target Time Seconds Register
pub type TSTRH0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - PPS Target Time Seconds Register
    #[inline(always)]
    pub fn tstrh0(&self) -> TSTRH0_R {
        TSTRH0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACPPSTTS1R")
            .field("tstrh0", &self.tstrh0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - PPS Target Time Seconds Register
    #[inline(always)]
    pub fn tstrh0(&mut self) -> TSTRH0_W<'_, MACPPSTTS1Rrs> {
        TSTRH0_W::new(self, 0)
    }
}
/**PPS 1 target time seconds register

You can [`read`](crate::Reg::read) this register and get [`macppstts1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppstts1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MACPPSTTS1R)*/
pub struct MACPPSTTS1Rrs;
impl crate::RegisterSpec for MACPPSTTS1Rrs {
    type Ux = u32;
}
///`read()` method returns [`macppstts1r::R`](R) reader structure
impl crate::Readable for MACPPSTTS1Rrs {}
///`write(|w| ..)` method takes [`macppstts1r::W`](W) writer structure
impl crate::Writable for MACPPSTTS1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACPPSTTS1R to value 0
impl crate::Resettable for MACPPSTTS1Rrs {}
