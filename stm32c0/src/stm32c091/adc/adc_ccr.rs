///Register `ADC_CCR` reader
pub type R = crate::R<ADC_CCRrs>;
///Register `ADC_CCR` writer
pub type W = crate::W<ADC_CCRrs>;
/**ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC {
    ///0: input ADC clock not divided
    B0x0 = 0,
    ///1: input ADC clock divided by 2
    B0x1 = 1,
    ///2: input ADC clock divided by 4
    B0x2 = 2,
    ///3: input ADC clock divided by 6
    B0x3 = 3,
    ///4: input ADC clock divided by 8
    B0x4 = 4,
    ///5: input ADC clock divided by 10
    B0x5 = 5,
    ///6: input ADC clock divided by 12
    B0x6 = 6,
    ///7: input ADC clock divided by 16
    B0x7 = 7,
    ///8: input ADC clock divided by 32
    B0x8 = 8,
    ///9: input ADC clock divided by 64
    B0x9 = 9,
    ///10: input ADC clock divided by 128
    B0xA = 10,
    ///11: input ADC clock divided by 256
    B0xB = 11,
}
impl From<PRESC> for u8 {
    #[inline(always)]
    fn from(variant: PRESC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESC {
    type Ux = u8;
}
impl crate::IsEnum for PRESC {}
///Field `PRESC` reader - ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type PRESC_R = crate::FieldReader<PRESC>;
impl PRESC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRESC> {
        match self.bits {
            0 => Some(PRESC::B0x0),
            1 => Some(PRESC::B0x1),
            2 => Some(PRESC::B0x2),
            3 => Some(PRESC::B0x3),
            4 => Some(PRESC::B0x4),
            5 => Some(PRESC::B0x5),
            6 => Some(PRESC::B0x6),
            7 => Some(PRESC::B0x7),
            8 => Some(PRESC::B0x8),
            9 => Some(PRESC::B0x9),
            10 => Some(PRESC::B0xA),
            11 => Some(PRESC::B0xB),
            _ => None,
        }
    }
    ///input ADC clock not divided
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PRESC::B0x0
    }
    ///input ADC clock divided by 2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PRESC::B0x1
    }
    ///input ADC clock divided by 4
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PRESC::B0x2
    }
    ///input ADC clock divided by 6
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PRESC::B0x3
    }
    ///input ADC clock divided by 8
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == PRESC::B0x4
    }
    ///input ADC clock divided by 10
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == PRESC::B0x5
    }
    ///input ADC clock divided by 12
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == PRESC::B0x6
    }
    ///input ADC clock divided by 16
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == PRESC::B0x7
    }
    ///input ADC clock divided by 32
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == PRESC::B0x8
    }
    ///input ADC clock divided by 64
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == PRESC::B0x9
    }
    ///input ADC clock divided by 128
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == PRESC::B0xA
    }
    ///input ADC clock divided by 256
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == PRESC::B0xB
    }
}
///Field `PRESC` writer - ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PRESC>;
impl<'a, REG> PRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///input ADC clock not divided
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::B0x0)
    }
    ///input ADC clock divided by 2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::B0x1)
    }
    ///input ADC clock divided by 4
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::B0x2)
    }
    ///input ADC clock divided by 6
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::B0x3)
    }
    ///input ADC clock divided by 8
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::B0x4)
    }
    ///input ADC clock divided by 10
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::B0x5)
    }
    ///input ADC clock divided by 12
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::B0x6)
    }
    ///input ADC clock divided by 16
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::B0x7)
    }
    ///input ADC clock divided by 32
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::B0x8)
    }
    ///input ADC clock divided by 64
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::B0x9)
    }
    ///input ADC clock divided by 128
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::B0xA)
    }
    ///input ADC clock divided by 256
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::B0xB)
    }
}
/**V<sub>REFINT</sub> enable This bit is set and cleared by software to enable/disable the V<sub>REFINT</sub>. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFEN {
    ///0: V<sub>REFINT</sub> disabled
    B0x0 = 0,
    ///1: V<sub>REFINT</sub> enabled
    B0x1 = 1,
}
impl From<VREFEN> for bool {
    #[inline(always)]
    fn from(variant: VREFEN) -> Self {
        variant as u8 != 0
    }
}
///Field `VREFEN` reader - V<sub>REFINT</sub> enable This bit is set and cleared by software to enable/disable the V<sub>REFINT</sub>. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type VREFEN_R = crate::BitReader<VREFEN>;
impl VREFEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VREFEN {
        match self.bits {
            false => VREFEN::B0x0,
            true => VREFEN::B0x1,
        }
    }
    ///V<sub>REFINT</sub> disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == VREFEN::B0x0
    }
    ///V<sub>REFINT</sub> enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == VREFEN::B0x1
    }
}
///Field `VREFEN` writer - V<sub>REFINT</sub> enable This bit is set and cleared by software to enable/disable the V<sub>REFINT</sub>. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG, VREFEN>;
impl<'a, REG> VREFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///V<sub>REFINT</sub> disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN::B0x0)
    }
    ///V<sub>REFINT</sub> enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN::B0x1)
    }
}
/**Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSEN {
    ///0: Temperature sensor disabled
    B0x0 = 0,
    ///1: Temperature sensor enabled
    B0x1 = 1,
}
impl From<TSEN> for bool {
    #[inline(always)]
    fn from(variant: TSEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TSEN` reader - Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type TSEN_R = crate::BitReader<TSEN>;
impl TSEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSEN {
        match self.bits {
            false => TSEN::B0x0,
            true => TSEN::B0x1,
        }
    }
    ///Temperature sensor disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TSEN::B0x0
    }
    ///Temperature sensor enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TSEN::B0x1
    }
}
///Field `TSEN` writer - Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type TSEN_W<'a, REG> = crate::BitWriter<'a, REG, TSEN>;
impl<'a, REG> TSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Temperature sensor disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TSEN::B0x0)
    }
    ///Temperature sensor enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TSEN::B0x1)
    }
}
impl R {
    ///Bits 18:21 - ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bit 22 - V<sub>REFINT</sub> enable This bit is set and cleared by software to enable/disable the V<sub>REFINT</sub>. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CCR")
            .field("presc", &self.presc())
            .field("vrefen", &self.vrefen())
            .field("tsen", &self.tsen())
            .finish()
    }
}
impl W {
    ///Bits 18:21 - ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W<'_, ADC_CCRrs> {
        PRESC_W::new(self, 18)
    }
    ///Bit 22 - V<sub>REFINT</sub> enable This bit is set and cleared by software to enable/disable the V<sub>REFINT</sub>. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W<'_, ADC_CCRrs> {
        VREFEN_W::new(self, 22)
    }
    ///Bit 23 - Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W<'_, ADC_CCRrs> {
        TSEN_W::new(self, 23)
    }
}
/**ADC common configuration register

You can [`read`](crate::Reg::read) this register and get [`adc_ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#ADC:ADC_CCR)*/
pub struct ADC_CCRrs;
impl crate::RegisterSpec for ADC_CCRrs {
    type Ux = u32;
}
///`read()` method returns [`adc_ccr::R`](R) reader structure
impl crate::Readable for ADC_CCRrs {}
///`write(|w| ..)` method takes [`adc_ccr::W`](W) writer structure
impl crate::Writable for ADC_CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADC_CCR to value 0
impl crate::Resettable for ADC_CCRrs {}
