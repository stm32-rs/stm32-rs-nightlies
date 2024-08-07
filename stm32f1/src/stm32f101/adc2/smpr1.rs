///Register `SMPR1` reader
pub type R = crate::R<SMPR1rs>;
///Register `SMPR1` writer
pub type W = crate::W<SMPR1rs>;
/**Sample time bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SMPX_X {
    ///0: 1.5 ADC clock cycles
    Cycles1_5 = 0,
    ///1: 7.5 ADC clock cycles
    Cycles7_5 = 1,
    ///2: 13.5 ADC clock cycles
    Cycles13_5 = 2,
    ///3: 28.5 ADC clock cycles
    Cycles28_5 = 3,
    ///4: 41.5 ADC clock cycles
    Cycles41_5 = 4,
    ///5: 55.5 ADC clock cycles
    Cycles55_5 = 5,
    ///6: 71.5 ADC clock cycles
    Cycles71_5 = 6,
    ///7: 239.5 ADC clock cycles
    Cycles239_5 = 7,
}
impl From<SMPX_X> for u32 {
    #[inline(always)]
    fn from(variant: SMPX_X) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMPX_X {
    type Ux = u32;
}
impl crate::IsEnum for SMPX_X {}
///Field `SMPx_x` reader - Sample time bits
pub type SMPX_X_R = crate::FieldReader<SMPX_X>;
impl SMPX_X_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SMPX_X> {
        match self.bits {
            0 => Some(SMPX_X::Cycles1_5),
            1 => Some(SMPX_X::Cycles7_5),
            2 => Some(SMPX_X::Cycles13_5),
            3 => Some(SMPX_X::Cycles28_5),
            4 => Some(SMPX_X::Cycles41_5),
            5 => Some(SMPX_X::Cycles55_5),
            6 => Some(SMPX_X::Cycles71_5),
            7 => Some(SMPX_X::Cycles239_5),
            _ => None,
        }
    }
    ///1.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SMPX_X::Cycles1_5
    }
    ///7.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SMPX_X::Cycles7_5
    }
    ///13.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles13_5(&self) -> bool {
        *self == SMPX_X::Cycles13_5
    }
    ///28.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles28_5(&self) -> bool {
        *self == SMPX_X::Cycles28_5
    }
    ///41.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles41_5(&self) -> bool {
        *self == SMPX_X::Cycles41_5
    }
    ///55.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles55_5(&self) -> bool {
        *self == SMPX_X::Cycles55_5
    }
    ///71.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles71_5(&self) -> bool {
        *self == SMPX_X::Cycles71_5
    }
    ///239.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles239_5(&self) -> bool {
        *self == SMPX_X::Cycles239_5
    }
}
///Field `SMPx_x` writer - Sample time bits
pub type SMPX_X_W<'a, REG> = crate::FieldWriter<'a, REG, 32, SMPX_X>;
impl<'a, REG> SMPX_X_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    ///1.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles1_5)
    }
    ///7.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles7_5)
    }
    ///13.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles13_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles13_5)
    }
    ///28.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles28_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles28_5)
    }
    ///41.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles41_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles41_5)
    }
    ///55.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles55_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles55_5)
    }
    ///71.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles71_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles71_5)
    }
    ///239.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles239_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles239_5)
    }
}
impl R {
    ///Bits 0:31 - Sample time bits
    #[inline(always)]
    pub fn smpx_x(&self) -> SMPX_X_R {
        SMPX_X_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR1")
            .field("smpx_x", &self.smpx_x())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Sample time bits
    #[inline(always)]
    #[must_use]
    pub fn smpx_x(&mut self) -> SMPX_X_W<SMPR1rs> {
        SMPX_X_W::new(self, 0)
    }
}
/**sample time register 1

You can [`read`](crate::Reg::read) this register and get [`smpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#ADC2:SMPR1)*/
pub struct SMPR1rs;
impl crate::RegisterSpec for SMPR1rs {
    type Ux = u32;
}
///`read()` method returns [`smpr1::R`](R) reader structure
impl crate::Readable for SMPR1rs {}
///`write(|w| ..)` method takes [`smpr1::W`](W) writer structure
impl crate::Writable for SMPR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SMPR1 to value 0
impl crate::Resettable for SMPR1rs {
    const RESET_VALUE: u32 = 0;
}
