///Register `PDCRH` reader
pub type R = crate::R<PDCRHrs>;
///Register `PDCRH` writer
pub type W = crate::W<PDCRHrs>;
///Field `PD0` reader - Port H pull-down bit y (y=0..1)
pub type PD0_R = crate::BitReader;
///Field `PD0` writer - Port H pull-down bit y (y=0..1)
pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD1` reader - Port H pull-down bit y (y=0..1)
pub type PD1_R = crate::BitReader;
///Field `PD1` writer - Port H pull-down bit y (y=0..1)
pub type PD1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Port H pull-down bit y (y=0..1)
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port H pull-down bit y (y=0..1)
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCRH")
            .field("pd1", &self.pd1())
            .field("pd0", &self.pd0())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port H pull-down bit y (y=0..1)
    #[inline(always)]
    pub fn pd0(&mut self) -> PD0_W<'_, PDCRHrs> {
        PD0_W::new(self, 0)
    }
    ///Bit 1 - Port H pull-down bit y (y=0..1)
    #[inline(always)]
    pub fn pd1(&mut self) -> PD1_W<'_, PDCRHrs> {
        PD1_W::new(self, 1)
    }
}
/**Power Port H pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x6.html#PWR:PDCRH)*/
pub struct PDCRHrs;
impl crate::RegisterSpec for PDCRHrs {
    type Ux = u32;
}
///`read()` method returns [`pdcrh::R`](R) reader structure
impl crate::Readable for PDCRHrs {}
///`write(|w| ..)` method takes [`pdcrh::W`](W) writer structure
impl crate::Writable for PDCRHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDCRH to value 0
impl crate::Resettable for PDCRHrs {}
