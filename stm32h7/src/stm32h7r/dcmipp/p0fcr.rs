///Register `P0FCR` writer
pub type W = crate::W<P0FCRrs>;
///Field `CLINEF` writer - Multi-line capture complete interrupt status clear Writing a 1 into this bit clears LINEF in the DCMIPP_P0SR register.
pub type CLINEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CFRAMEF` writer - Frame capture complete interrupt status clear Writing a 1 into this bit clears the FRAMEF bit in the DCMIPP_P0SR register.
pub type CFRAMEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CVSYNCF` writer - Vertical synchronization interrupt status clear Writing a 1 into this bit clears the VSYNCF bit in the DCMIPP_P0SR register.
pub type CVSYNCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLIMITF` writer - limit interrupt status clear Writing a 1 into this bit clears LIMITF in the DCMIPP_P0SR register.
pub type CLIMITF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COVRF` writer - Overrun interrupt status clear Writing a 1 into this bit clears the OVRF bit in the DCMIPP_P0SR register.
pub type COVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<P0FCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Multi-line capture complete interrupt status clear Writing a 1 into this bit clears LINEF in the DCMIPP_P0SR register.
    #[inline(always)]
    pub fn clinef(&mut self) -> CLINEF_W<'_, P0FCRrs> {
        CLINEF_W::new(self, 0)
    }
    ///Bit 1 - Frame capture complete interrupt status clear Writing a 1 into this bit clears the FRAMEF bit in the DCMIPP_P0SR register.
    #[inline(always)]
    pub fn cframef(&mut self) -> CFRAMEF_W<'_, P0FCRrs> {
        CFRAMEF_W::new(self, 1)
    }
    ///Bit 2 - Vertical synchronization interrupt status clear Writing a 1 into this bit clears the VSYNCF bit in the DCMIPP_P0SR register.
    #[inline(always)]
    pub fn cvsyncf(&mut self) -> CVSYNCF_W<'_, P0FCRrs> {
        CVSYNCF_W::new(self, 2)
    }
    ///Bit 6 - limit interrupt status clear Writing a 1 into this bit clears LIMITF in the DCMIPP_P0SR register.
    #[inline(always)]
    pub fn climitf(&mut self) -> CLIMITF_W<'_, P0FCRrs> {
        CLIMITF_W::new(self, 6)
    }
    ///Bit 7 - Overrun interrupt status clear Writing a 1 into this bit clears the OVRF bit in the DCMIPP_P0SR register.
    #[inline(always)]
    pub fn covrf(&mut self) -> COVRF_W<'_, P0FCRrs> {
        COVRF_W::new(self, 7)
    }
}
/**DCMIPP Pipe0 interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:P0FCR)*/
pub struct P0FCRrs;
impl crate::RegisterSpec for P0FCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`p0fcr::W`](W) writer structure
impl crate::Writable for P0FCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P0FCR to value 0
impl crate::Resettable for P0FCRrs {}
