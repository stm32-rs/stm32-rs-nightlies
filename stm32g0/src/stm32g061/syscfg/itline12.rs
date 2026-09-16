///Register `ITLINE12` reader
pub type R = crate::R<ITLINE12rs>;
/**ADC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<ADC> for bool {
    #[inline(always)]
    fn from(variant: ADC) -> Self {
        variant as u8 != 0
    }
}
///Field `ADC` reader - ADC
pub type ADC_R = crate::BitReader<ADC>;
impl ADC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADC {
        match self.bits {
            false => ADC::NotInterrupted,
            true => ADC::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == ADC::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == ADC::Interrupted
    }
}
///Field `COMP1` reader - COMP1
pub use ADC_R as COMP1_R;
///Field `COMP2` reader - COMP2
pub use ADC_R as COMP2_R;
///Field `COMP3` reader - COMP3
pub use ADC_R as COMP3_R;
impl R {
    ///Bit 0 - ADC
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - COMP1
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - COMP2
    #[inline(always)]
    pub fn comp2(&self) -> COMP2_R {
        COMP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - COMP3
    #[inline(always)]
    pub fn comp3(&self) -> COMP3_R {
        COMP3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE12")
            .field("adc", &self.adc())
            .field("comp1", &self.comp1())
            .field("comp2", &self.comp2())
            .field("comp3", &self.comp3())
            .finish()
    }
}
/**interrupt line 12 status register

You can [`read`](crate::Reg::read) this register and get [`itline12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#SYSCFG:ITLINE12)*/
pub struct ITLINE12rs;
impl crate::RegisterSpec for ITLINE12rs {
    type Ux = u32;
}
///`read()` method returns [`itline12::R`](R) reader structure
impl crate::Readable for ITLINE12rs {}
///`reset()` method sets ITLINE12 to value 0
impl crate::Resettable for ITLINE12rs {}
