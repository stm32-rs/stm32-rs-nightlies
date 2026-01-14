///Register `MACPPSW1R` reader
pub type R = crate::R<MACPPSW1Rrs>;
///Register `MACPPSW1R` writer
pub type W = crate::W<MACPPSW1Rrs>;
///Field `PPSWIDTH0` reader - PPS Output Signal Width
pub type PPSWIDTH0_R = crate::FieldReader<u32>;
///Field `PPSWIDTH0` writer - PPS Output Signal Width
pub type PPSWIDTH0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - PPS Output Signal Width
    #[inline(always)]
    pub fn ppswidth0(&self) -> PPSWIDTH0_R {
        PPSWIDTH0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACPPSW1R")
            .field("ppswidth0", &self.ppswidth0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - PPS Output Signal Width
    #[inline(always)]
    pub fn ppswidth0(&mut self) -> PPSWIDTH0_W<'_, MACPPSW1Rrs> {
        PPSWIDTH0_W::new(self, 0)
    }
}
/**PPS 1 width register

You can [`read`](crate::Reg::read) this register and get [`macppsw1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsw1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MACPPSW1R)*/
pub struct MACPPSW1Rrs;
impl crate::RegisterSpec for MACPPSW1Rrs {
    type Ux = u32;
}
///`read()` method returns [`macppsw1r::R`](R) reader structure
impl crate::Readable for MACPPSW1Rrs {}
///`write(|w| ..)` method takes [`macppsw1r::W`](W) writer structure
impl crate::Writable for MACPPSW1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACPPSW1R to value 0
impl crate::Resettable for MACPPSW1Rrs {}
