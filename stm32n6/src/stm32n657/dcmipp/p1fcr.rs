///Register `P1FCR` writer
pub type W = crate::W<P1FCRrs>;
///Field `CLINEF` writer - Multi-line capture complete interrupt status clear
pub type CLINEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFRAMEF` writer - Frame capture complete interrupt status clear
pub type CFRAMEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CVSYNCF` writer - Vertical synchronization interrupt status clear
pub type CVSYNCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COVRF` writer - Overrun interrupt status clear
pub type COVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<P1FCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Multi-line capture complete interrupt status clear
    #[inline(always)]
    pub fn clinef(&mut self) -> CLINEF_W<'_, P1FCRrs> {
        CLINEF_W::new(self, 0)
    }
    ///Bit 1 - Frame capture complete interrupt status clear
    #[inline(always)]
    pub fn cframef(&mut self) -> CFRAMEF_W<'_, P1FCRrs> {
        CFRAMEF_W::new(self, 1)
    }
    ///Bit 2 - Vertical synchronization interrupt status clear
    #[inline(always)]
    pub fn cvsyncf(&mut self) -> CVSYNCF_W<'_, P1FCRrs> {
        CVSYNCF_W::new(self, 2)
    }
    ///Bit 7 - Overrun interrupt status clear
    #[inline(always)]
    pub fn covrf(&mut self) -> COVRF_W<'_, P1FCRrs> {
        COVRF_W::new(self, 7)
    }
}
/**DCMIPP Pipe1 interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1FCR)*/
pub struct P1FCRrs;
impl crate::RegisterSpec for P1FCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`p1fcr::W`](W) writer structure
impl crate::Writable for P1FCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1FCR to value 0
impl crate::Resettable for P1FCRrs {}
