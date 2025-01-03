///Register `SWTRGR` writer
pub type W = crate::W<SWTRGRrs>;
/**DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWTRIG1 {
    ///0: No trigger
    NoTrigger = 0,
    ///1: Trigger
    Trigger = 1,
}
impl From<SWTRIG1> for bool {
    #[inline(always)]
    fn from(variant: SWTRIG1) -> Self {
        variant as u8 != 0
    }
}
///Field `SWTRIG1` writer - DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register.
pub type SWTRIG1_W<'a, REG> = crate::BitWriter<'a, REG, SWTRIG1>;
impl<'a, REG> SWTRIG1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No trigger
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(SWTRIG1::NoTrigger)
    }
    ///Trigger
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(SWTRIG1::Trigger)
    }
}
///Field `SWTRIG2` writer - DAC channel2 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR2 register value has been loaded into the DAC_DOR2 register.
pub use SWTRIG1_W as SWTRIG2_W;
///Field `SWTRIGB1` writer - DAC channel1 software trigger B
pub type SWTRIGB1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWTRIGB2` writer - DAC channel2 software trigger B
pub type SWTRIGB2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SWTRGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register.
    #[inline(always)]
    pub fn swtrig1(&mut self) -> SWTRIG1_W<SWTRGRrs> {
        SWTRIG1_W::new(self, 0)
    }
    ///Bit 1 - DAC channel2 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR2 register value has been loaded into the DAC_DOR2 register.
    #[inline(always)]
    pub fn swtrig2(&mut self) -> SWTRIG2_W<SWTRGRrs> {
        SWTRIG2_W::new(self, 1)
    }
    ///Bit 16 - DAC channel1 software trigger B
    #[inline(always)]
    pub fn swtrigb1(&mut self) -> SWTRIGB1_W<SWTRGRrs> {
        SWTRIGB1_W::new(self, 16)
    }
    ///Bit 17 - DAC channel2 software trigger B
    #[inline(always)]
    pub fn swtrigb2(&mut self) -> SWTRIGB2_W<SWTRGRrs> {
        SWTRIGB2_W::new(self, 17)
    }
}
/**DAC software trigger register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431.html#DAC1:SWTRGR)*/
pub struct SWTRGRrs;
impl crate::RegisterSpec for SWTRGRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`swtrgr::W`](W) writer structure
impl crate::Writable for SWTRGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SWTRGR to value 0
impl crate::Resettable for SWTRGRrs {
    const RESET_VALUE: u32 = 0;
}
