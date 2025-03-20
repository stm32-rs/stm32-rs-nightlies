///Register `SWTRIGR` writer
pub type W = crate::W<SWTRIGRrs>;
/**DAC channel1 software trigger

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWTRIG1 {
    ///0: DAC channel X software trigger disabled
    Disabled = 0,
    ///1: DAC channel X software trigger enabled
    Enabled = 1,
}
impl From<SWTRIG1> for bool {
    #[inline(always)]
    fn from(variant: SWTRIG1) -> Self {
        variant as u8 != 0
    }
}
///Field `SWTRIG1` writer - DAC channel1 software trigger
pub type SWTRIG1_W<'a, REG> = crate::BitWriter<'a, REG, SWTRIG1>;
impl<'a, REG> SWTRIG1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DAC channel X software trigger disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SWTRIG1::Disabled)
    }
    ///DAC channel X software trigger enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SWTRIG1::Enabled)
    }
}
///Field `SWTRIG2` writer - DAC channel2 software trigger
pub use SWTRIG1_W as SWTRIG2_W;
impl core::fmt::Debug for crate::generic::Reg<SWTRIGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - DAC channel1 software trigger
    #[inline(always)]
    pub fn swtrig1(&mut self) -> SWTRIG1_W<SWTRIGRrs> {
        SWTRIG1_W::new(self, 0)
    }
    ///Bit 1 - DAC channel2 software trigger
    #[inline(always)]
    pub fn swtrig2(&mut self) -> SWTRIG2_W<SWTRIGRrs> {
        SWTRIG2_W::new(self, 1)
    }
}
/**software trigger register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrigr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F746.html#DAC:SWTRIGR)*/
pub struct SWTRIGRrs;
impl crate::RegisterSpec for SWTRIGRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`swtrigr::W`](W) writer structure
impl crate::Writable for SWTRIGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWTRIGR to value 0
impl crate::Resettable for SWTRIGRrs {}
