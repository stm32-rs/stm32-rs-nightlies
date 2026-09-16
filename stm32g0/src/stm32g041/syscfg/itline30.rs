///Register `ITLINE30` reader
pub type R = crate::R<ITLINE30rs>;
/**CEC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEC {
    ///0: Interrupt not triggered
    NotInterrupted = 0,
    ///1: Interrup triggered
    Interrupted = 1,
}
impl From<CEC> for bool {
    #[inline(always)]
    fn from(variant: CEC) -> Self {
        variant as u8 != 0
    }
}
///Field `CEC` reader - CEC
pub type CEC_R = crate::BitReader<CEC>;
impl CEC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CEC {
        match self.bits {
            false => CEC::NotInterrupted,
            true => CEC::Interrupted,
        }
    }
    ///Interrupt not triggered
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == CEC::NotInterrupted
    }
    ///Interrup triggered
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == CEC::Interrupted
    }
}
impl R {
    ///Bit 0 - CEC
    #[inline(always)]
    pub fn cec(&self) -> CEC_R {
        CEC_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITLINE30")
            .field("cec", &self.cec())
            .finish()
    }
}
/**interrupt line 25 status register

You can [`read`](crate::Reg::read) this register and get [`itline30::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G041.html#SYSCFG:ITLINE30)*/
pub struct ITLINE30rs;
impl crate::RegisterSpec for ITLINE30rs {
    type Ux = u32;
}
///`read()` method returns [`itline30::R`](R) reader structure
impl crate::Readable for ITLINE30rs {}
///`reset()` method sets ITLINE30 to value 0
impl crate::Resettable for ITLINE30rs {}
