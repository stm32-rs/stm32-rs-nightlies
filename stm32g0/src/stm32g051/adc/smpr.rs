///Register `SMPR` reader
pub type R = crate::R<SMPRrs>;
///Register `SMPR` writer
pub type W = crate::W<SMPRrs>;
/**Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP1 {
    ///0: 1.5 ADC clock cycles
    Cycles1_5 = 0,
    ///1: 3.5 ADC clock cycles
    Cycles3_5 = 1,
    ///2: 7.5 ADC clock cycles
    Cycles7_5 = 2,
    ///3: 12.5 ADC clock cycles
    Cycles12_5 = 3,
    ///4: 19.5 ADC clock cycles
    Cycles19_5 = 4,
    ///5: 39.5 ADC clock cycles
    Cycles39_5 = 5,
    ///6: 79.5 ADC clock cycles
    Cycles79_5 = 6,
    ///7: 160.5 ADC clock cycles
    Cycles160_5 = 7,
}
impl From<SMP1> for u8 {
    #[inline(always)]
    fn from(variant: SMP1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP1 {
    type Ux = u8;
}
impl crate::IsEnum for SMP1 {}
///Field `SMP1` reader - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type SMP1_R = crate::FieldReader<SMP1>;
impl SMP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMP1 {
        match self.bits {
            0 => SMP1::Cycles1_5,
            1 => SMP1::Cycles3_5,
            2 => SMP1::Cycles7_5,
            3 => SMP1::Cycles12_5,
            4 => SMP1::Cycles19_5,
            5 => SMP1::Cycles39_5,
            6 => SMP1::Cycles79_5,
            7 => SMP1::Cycles160_5,
            _ => unreachable!(),
        }
    }
    ///1.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SMP1::Cycles1_5
    }
    ///3.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles3_5(&self) -> bool {
        *self == SMP1::Cycles3_5
    }
    ///7.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SMP1::Cycles7_5
    }
    ///12.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles12_5(&self) -> bool {
        *self == SMP1::Cycles12_5
    }
    ///19.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles19_5(&self) -> bool {
        *self == SMP1::Cycles19_5
    }
    ///39.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles39_5(&self) -> bool {
        *self == SMP1::Cycles39_5
    }
    ///79.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles79_5(&self) -> bool {
        *self == SMP1::Cycles79_5
    }
    ///160.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles160_5(&self) -> bool {
        *self == SMP1::Cycles160_5
    }
}
///Field `SMP1` writer - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type SMP1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP1, crate::Safe>;
impl<'a, REG> SMP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles1_5)
    }
    ///3.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles3_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles3_5)
    }
    ///7.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles7_5)
    }
    ///12.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles12_5)
    }
    ///19.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles19_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles19_5)
    }
    ///39.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles39_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles39_5)
    }
    ///79.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles79_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles79_5)
    }
    ///160.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles160_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles160_5)
    }
}
///Field `SMP2` reader - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMP1_R as SMP2_R;
///Field `SMP2` writer - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMP1_W as SMP2_W;
/**Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL0 {
    ///0: Sampling time of CHANNELx use the setting of SMP1 register
    Smp1 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2 register
    Smp2 = 1,
}
impl From<SMPSEL0> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL0) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL0` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type SMPSEL0_R = crate::BitReader<SMPSEL0>;
impl SMPSEL0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL0 {
        match self.bits {
            false => SMPSEL0::Smp1,
            true => SMPSEL0::Smp2,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1 register
    #[inline(always)]
    pub fn is_smp1(&self) -> bool {
        *self == SMPSEL0::Smp1
    }
    ///Sampling time of CHANNELx use the setting of SMP2 register
    #[inline(always)]
    pub fn is_smp2(&self) -> bool {
        *self == SMPSEL0::Smp2
    }
}
///Field `SMPSEL0` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub type SMPSEL0_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL0>;
impl<'a, REG> SMPSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of CHANNELx use the setting of SMP1 register
    #[inline(always)]
    pub fn smp1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL0::Smp1)
    }
    ///Sampling time of CHANNELx use the setting of SMP2 register
    #[inline(always)]
    pub fn smp2(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL0::Smp2)
    }
}
///Field `SMPSEL1` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_R as SMPSEL1_R;
///Field `SMPSEL2` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_R as SMPSEL2_R;
///Field `SMPSEL3` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_R as SMPSEL3_R;
///Field `SMPSEL4` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_R as SMPSEL4_R;
///Field `SMPSEL5` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_R as SMPSEL5_R;
///Field `SMPSEL6` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_R as SMPSEL6_R;
///Field `SMPSEL7` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_R as SMPSEL7_R;
///Field `SMPSEL8` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_R as SMPSEL8_R;
///Field `SMPSEL9` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_R as SMPSEL9_R;
///Field `SMPSEL10` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_R as SMPSEL10_R;
///Field `SMPSEL11` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_R as SMPSEL11_R;
///Field `SMPSEL12` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_R as SMPSEL12_R;
///Field `SMPSEL13` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_R as SMPSEL13_R;
///Field `SMPSEL14` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_R as SMPSEL14_R;
///Field `SMPSEL15` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_R as SMPSEL15_R;
///Field `SMPSEL16` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_R as SMPSEL16_R;
///Field `SMPSEL17` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_R as SMPSEL17_R;
///Field `SMPSEL18` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_R as SMPSEL18_R;
///Field `SMPSEL1` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_W as SMPSEL1_W;
///Field `SMPSEL2` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_W as SMPSEL2_W;
///Field `SMPSEL3` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_W as SMPSEL3_W;
///Field `SMPSEL4` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_W as SMPSEL4_W;
///Field `SMPSEL5` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_W as SMPSEL5_W;
///Field `SMPSEL6` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_W as SMPSEL6_W;
///Field `SMPSEL7` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_W as SMPSEL7_W;
///Field `SMPSEL8` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_W as SMPSEL8_W;
///Field `SMPSEL9` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_W as SMPSEL9_W;
///Field `SMPSEL10` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_W as SMPSEL10_W;
///Field `SMPSEL11` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_W as SMPSEL11_W;
///Field `SMPSEL12` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_W as SMPSEL12_W;
///Field `SMPSEL13` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_W as SMPSEL13_W;
///Field `SMPSEL14` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_W as SMPSEL14_W;
///Field `SMPSEL15` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_W as SMPSEL15_W;
///Field `SMPSEL16` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_W as SMPSEL16_W;
///Field `SMPSEL17` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_W as SMPSEL17_W;
///Field `SMPSEL18` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
pub use SMPSEL0_W as SMPSEL18_W;
impl R {
    ///Bits 0:2 - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 8 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel0(&self) -> SMPSEL0_R {
        SMPSEL0_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel1(&self) -> SMPSEL1_R {
        SMPSEL1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel2(&self) -> SMPSEL2_R {
        SMPSEL2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel3(&self) -> SMPSEL3_R {
        SMPSEL3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel4(&self) -> SMPSEL4_R {
        SMPSEL4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel5(&self) -> SMPSEL5_R {
        SMPSEL5_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel6(&self) -> SMPSEL6_R {
        SMPSEL6_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel7(&self) -> SMPSEL7_R {
        SMPSEL7_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel8(&self) -> SMPSEL8_R {
        SMPSEL8_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel9(&self) -> SMPSEL9_R {
        SMPSEL9_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel10(&self) -> SMPSEL10_R {
        SMPSEL10_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel11(&self) -> SMPSEL11_R {
        SMPSEL11_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel12(&self) -> SMPSEL12_R {
        SMPSEL12_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel13(&self) -> SMPSEL13_R {
        SMPSEL13_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel14(&self) -> SMPSEL14_R {
        SMPSEL14_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel15(&self) -> SMPSEL15_R {
        SMPSEL15_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel16(&self) -> SMPSEL16_R {
        SMPSEL16_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel17(&self) -> SMPSEL17_R {
        SMPSEL17_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel18(&self) -> SMPSEL18_R {
        SMPSEL18_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR")
            .field("smp1", &self.smp1())
            .field("smp2", &self.smp2())
            .field("smpsel0", &self.smpsel0())
            .field("smpsel1", &self.smpsel1())
            .field("smpsel2", &self.smpsel2())
            .field("smpsel3", &self.smpsel3())
            .field("smpsel4", &self.smpsel4())
            .field("smpsel5", &self.smpsel5())
            .field("smpsel6", &self.smpsel6())
            .field("smpsel7", &self.smpsel7())
            .field("smpsel8", &self.smpsel8())
            .field("smpsel9", &self.smpsel9())
            .field("smpsel10", &self.smpsel10())
            .field("smpsel11", &self.smpsel11())
            .field("smpsel12", &self.smpsel12())
            .field("smpsel13", &self.smpsel13())
            .field("smpsel14", &self.smpsel14())
            .field("smpsel15", &self.smpsel15())
            .field("smpsel16", &self.smpsel16())
            .field("smpsel17", &self.smpsel17())
            .field("smpsel18", &self.smpsel18())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smp1(&mut self) -> SMP1_W<SMPRrs> {
        SMP1_W::new(self, 0)
    }
    ///Bits 4:6 - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smp2(&mut self) -> SMP2_W<SMPRrs> {
        SMP2_W::new(self, 4)
    }
    ///Bit 8 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel0(&mut self) -> SMPSEL0_W<SMPRrs> {
        SMPSEL0_W::new(self, 8)
    }
    ///Bit 9 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel1(&mut self) -> SMPSEL1_W<SMPRrs> {
        SMPSEL1_W::new(self, 9)
    }
    ///Bit 10 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel2(&mut self) -> SMPSEL2_W<SMPRrs> {
        SMPSEL2_W::new(self, 10)
    }
    ///Bit 11 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel3(&mut self) -> SMPSEL3_W<SMPRrs> {
        SMPSEL3_W::new(self, 11)
    }
    ///Bit 12 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel4(&mut self) -> SMPSEL4_W<SMPRrs> {
        SMPSEL4_W::new(self, 12)
    }
    ///Bit 13 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel5(&mut self) -> SMPSEL5_W<SMPRrs> {
        SMPSEL5_W::new(self, 13)
    }
    ///Bit 14 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel6(&mut self) -> SMPSEL6_W<SMPRrs> {
        SMPSEL6_W::new(self, 14)
    }
    ///Bit 15 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel7(&mut self) -> SMPSEL7_W<SMPRrs> {
        SMPSEL7_W::new(self, 15)
    }
    ///Bit 16 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel8(&mut self) -> SMPSEL8_W<SMPRrs> {
        SMPSEL8_W::new(self, 16)
    }
    ///Bit 17 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel9(&mut self) -> SMPSEL9_W<SMPRrs> {
        SMPSEL9_W::new(self, 17)
    }
    ///Bit 18 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel10(&mut self) -> SMPSEL10_W<SMPRrs> {
        SMPSEL10_W::new(self, 18)
    }
    ///Bit 19 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel11(&mut self) -> SMPSEL11_W<SMPRrs> {
        SMPSEL11_W::new(self, 19)
    }
    ///Bit 20 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel12(&mut self) -> SMPSEL12_W<SMPRrs> {
        SMPSEL12_W::new(self, 20)
    }
    ///Bit 21 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel13(&mut self) -> SMPSEL13_W<SMPRrs> {
        SMPSEL13_W::new(self, 21)
    }
    ///Bit 22 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel14(&mut self) -> SMPSEL14_W<SMPRrs> {
        SMPSEL14_W::new(self, 22)
    }
    ///Bit 23 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel15(&mut self) -> SMPSEL15_W<SMPRrs> {
        SMPSEL15_W::new(self, 23)
    }
    ///Bit 24 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel16(&mut self) -> SMPSEL16_W<SMPRrs> {
        SMPSEL16_W::new(self, 24)
    }
    ///Bit 25 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel17(&mut self) -> SMPSEL17_W<SMPRrs> {
        SMPSEL17_W::new(self, 25)
    }
    ///Bit 26 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn smpsel18(&mut self) -> SMPSEL18_W<SMPRrs> {
        SMPSEL18_W::new(self, 26)
    }
}
/**ADC sampling time register

You can [`read`](crate::Reg::read) this register and get [`smpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G051.html#ADC:SMPR)*/
pub struct SMPRrs;
impl crate::RegisterSpec for SMPRrs {
    type Ux = u32;
}
///`read()` method returns [`smpr::R`](R) reader structure
impl crate::Readable for SMPRrs {}
///`write(|w| ..)` method takes [`smpr::W`](W) writer structure
impl crate::Writable for SMPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SMPR to value 0
impl crate::Resettable for SMPRrs {
    const RESET_VALUE: u32 = 0;
}
