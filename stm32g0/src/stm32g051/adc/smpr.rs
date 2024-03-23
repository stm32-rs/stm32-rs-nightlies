#[doc = "Register `SMPR` reader"]
pub type R = crate::R<SMPRrs>;
#[doc = "Register `SMPR` writer"]
pub type W = crate::W<SMPRrs>;
#[doc = "Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP1 {
    #[doc = "0: 1.5 ADC clock cycles"]
    Cycles1_5 = 0,
    #[doc = "1: 3.5 ADC clock cycles"]
    Cycles3_5 = 1,
    #[doc = "2: 7.5 ADC clock cycles"]
    Cycles7_5 = 2,
    #[doc = "3: 12.5 ADC clock cycles"]
    Cycles12_5 = 3,
    #[doc = "4: 19.5 ADC clock cycles"]
    Cycles19_5 = 4,
    #[doc = "5: 39.5 ADC clock cycles"]
    Cycles39_5 = 5,
    #[doc = "6: 79.5 ADC clock cycles"]
    Cycles79_5 = 6,
    #[doc = "7: 160.5 ADC clock cycles"]
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
#[doc = "Field `SMP1` reader - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub type SMP1_R = crate::FieldReader<SMP1>;
impl SMP1_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SMP1::Cycles1_5
    }
    #[doc = "3.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles3_5(&self) -> bool {
        *self == SMP1::Cycles3_5
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SMP1::Cycles7_5
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles12_5(&self) -> bool {
        *self == SMP1::Cycles12_5
    }
    #[doc = "19.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles19_5(&self) -> bool {
        *self == SMP1::Cycles19_5
    }
    #[doc = "39.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles39_5(&self) -> bool {
        *self == SMP1::Cycles39_5
    }
    #[doc = "79.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles79_5(&self) -> bool {
        *self == SMP1::Cycles79_5
    }
    #[doc = "160.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles160_5(&self) -> bool {
        *self == SMP1::Cycles160_5
    }
}
#[doc = "Field `SMP1` writer - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub type SMP1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SMP1>;
impl<'a, REG> SMP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles1_5)
    }
    #[doc = "3.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles3_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles3_5)
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles7_5)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles12_5)
    }
    #[doc = "19.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles19_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles19_5)
    }
    #[doc = "39.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles39_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles39_5)
    }
    #[doc = "79.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles79_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles79_5)
    }
    #[doc = "160.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles160_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles160_5)
    }
}
#[doc = "Field `SMP2` reader - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMP1_R as SMP2_R;
#[doc = "Field `SMP2` writer - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMP1_W as SMP2_W;
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL0 {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1 register"]
    Smp1 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2 register"]
    Smp2 = 1,
}
impl From<SMPSEL0> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSEL0` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL0_R = crate::BitReader<SMPSEL0>;
impl SMPSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL0 {
        match self.bits {
            false => SMPSEL0::Smp1,
            true => SMPSEL0::Smp2,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1 register"]
    #[inline(always)]
    pub fn is_smp1(&self) -> bool {
        *self == SMPSEL0::Smp1
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2 register"]
    #[inline(always)]
    pub fn is_smp2(&self) -> bool {
        *self == SMPSEL0::Smp2
    }
}
#[doc = "Field `SMPSEL0` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL0_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL0>;
impl<'a, REG> SMPSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1 register"]
    #[inline(always)]
    pub fn smp1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL0::Smp1)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2 register"]
    #[inline(always)]
    pub fn smp2(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL0::Smp2)
    }
}
#[doc = "Field `SMPSEL1` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_R as SMPSEL1_R;
#[doc = "Field `SMPSEL2` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_R as SMPSEL2_R;
#[doc = "Field `SMPSEL3` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_R as SMPSEL3_R;
#[doc = "Field `SMPSEL4` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_R as SMPSEL4_R;
#[doc = "Field `SMPSEL5` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_R as SMPSEL5_R;
#[doc = "Field `SMPSEL6` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_R as SMPSEL6_R;
#[doc = "Field `SMPSEL7` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_R as SMPSEL7_R;
#[doc = "Field `SMPSEL8` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_R as SMPSEL8_R;
#[doc = "Field `SMPSEL9` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_R as SMPSEL9_R;
#[doc = "Field `SMPSEL10` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_R as SMPSEL10_R;
#[doc = "Field `SMPSEL11` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_R as SMPSEL11_R;
#[doc = "Field `SMPSEL12` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_R as SMPSEL12_R;
#[doc = "Field `SMPSEL13` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_R as SMPSEL13_R;
#[doc = "Field `SMPSEL14` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_R as SMPSEL14_R;
#[doc = "Field `SMPSEL15` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_R as SMPSEL15_R;
#[doc = "Field `SMPSEL16` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_R as SMPSEL16_R;
#[doc = "Field `SMPSEL17` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_R as SMPSEL17_R;
#[doc = "Field `SMPSEL18` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_R as SMPSEL18_R;
#[doc = "Field `SMPSEL1` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_W as SMPSEL1_W;
#[doc = "Field `SMPSEL2` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_W as SMPSEL2_W;
#[doc = "Field `SMPSEL3` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_W as SMPSEL3_W;
#[doc = "Field `SMPSEL4` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_W as SMPSEL4_W;
#[doc = "Field `SMPSEL5` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_W as SMPSEL5_W;
#[doc = "Field `SMPSEL6` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_W as SMPSEL6_W;
#[doc = "Field `SMPSEL7` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_W as SMPSEL7_W;
#[doc = "Field `SMPSEL8` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_W as SMPSEL8_W;
#[doc = "Field `SMPSEL9` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_W as SMPSEL9_W;
#[doc = "Field `SMPSEL10` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_W as SMPSEL10_W;
#[doc = "Field `SMPSEL11` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_W as SMPSEL11_W;
#[doc = "Field `SMPSEL12` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_W as SMPSEL12_W;
#[doc = "Field `SMPSEL13` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_W as SMPSEL13_W;
#[doc = "Field `SMPSEL14` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_W as SMPSEL14_W;
#[doc = "Field `SMPSEL15` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_W as SMPSEL15_W;
#[doc = "Field `SMPSEL16` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_W as SMPSEL16_W;
#[doc = "Field `SMPSEL17` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_W as SMPSEL17_W;
#[doc = "Field `SMPSEL18` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub use SMPSEL0_W as SMPSEL18_W;
impl R {
    #[doc = "Bits 0:2 - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel0(&self) -> SMPSEL0_R {
        SMPSEL0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel1(&self) -> SMPSEL1_R {
        SMPSEL1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel2(&self) -> SMPSEL2_R {
        SMPSEL2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel3(&self) -> SMPSEL3_R {
        SMPSEL3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel4(&self) -> SMPSEL4_R {
        SMPSEL4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel5(&self) -> SMPSEL5_R {
        SMPSEL5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel6(&self) -> SMPSEL6_R {
        SMPSEL6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel7(&self) -> SMPSEL7_R {
        SMPSEL7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel8(&self) -> SMPSEL8_R {
        SMPSEL8_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel9(&self) -> SMPSEL9_R {
        SMPSEL9_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel10(&self) -> SMPSEL10_R {
        SMPSEL10_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel11(&self) -> SMPSEL11_R {
        SMPSEL11_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel12(&self) -> SMPSEL12_R {
        SMPSEL12_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel13(&self) -> SMPSEL13_R {
        SMPSEL13_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel14(&self) -> SMPSEL14_R {
        SMPSEL14_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel15(&self) -> SMPSEL15_R {
        SMPSEL15_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel16(&self) -> SMPSEL16_R {
        SMPSEL16_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel17(&self) -> SMPSEL17_R {
        SMPSEL17_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel18(&self) -> SMPSEL18_R {
        SMPSEL18_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smp1(&mut self) -> SMP1_W<SMPRrs> {
        SMP1_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smp2(&mut self) -> SMP2_W<SMPRrs> {
        SMP2_W::new(self, 4)
    }
    #[doc = "Bit 8 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel0(&mut self) -> SMPSEL0_W<SMPRrs> {
        SMPSEL0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel1(&mut self) -> SMPSEL1_W<SMPRrs> {
        SMPSEL1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel2(&mut self) -> SMPSEL2_W<SMPRrs> {
        SMPSEL2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel3(&mut self) -> SMPSEL3_W<SMPRrs> {
        SMPSEL3_W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel4(&mut self) -> SMPSEL4_W<SMPRrs> {
        SMPSEL4_W::new(self, 12)
    }
    #[doc = "Bit 13 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel5(&mut self) -> SMPSEL5_W<SMPRrs> {
        SMPSEL5_W::new(self, 13)
    }
    #[doc = "Bit 14 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel6(&mut self) -> SMPSEL6_W<SMPRrs> {
        SMPSEL6_W::new(self, 14)
    }
    #[doc = "Bit 15 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel7(&mut self) -> SMPSEL7_W<SMPRrs> {
        SMPSEL7_W::new(self, 15)
    }
    #[doc = "Bit 16 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel8(&mut self) -> SMPSEL8_W<SMPRrs> {
        SMPSEL8_W::new(self, 16)
    }
    #[doc = "Bit 17 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel9(&mut self) -> SMPSEL9_W<SMPRrs> {
        SMPSEL9_W::new(self, 17)
    }
    #[doc = "Bit 18 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel10(&mut self) -> SMPSEL10_W<SMPRrs> {
        SMPSEL10_W::new(self, 18)
    }
    #[doc = "Bit 19 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel11(&mut self) -> SMPSEL11_W<SMPRrs> {
        SMPSEL11_W::new(self, 19)
    }
    #[doc = "Bit 20 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel12(&mut self) -> SMPSEL12_W<SMPRrs> {
        SMPSEL12_W::new(self, 20)
    }
    #[doc = "Bit 21 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel13(&mut self) -> SMPSEL13_W<SMPRrs> {
        SMPSEL13_W::new(self, 21)
    }
    #[doc = "Bit 22 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel14(&mut self) -> SMPSEL14_W<SMPRrs> {
        SMPSEL14_W::new(self, 22)
    }
    #[doc = "Bit 23 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel15(&mut self) -> SMPSEL15_W<SMPRrs> {
        SMPSEL15_W::new(self, 23)
    }
    #[doc = "Bit 24 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel16(&mut self) -> SMPSEL16_W<SMPRrs> {
        SMPSEL16_W::new(self, 24)
    }
    #[doc = "Bit 25 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel17(&mut self) -> SMPSEL17_W<SMPRrs> {
        SMPSEL17_W::new(self, 25)
    }
    #[doc = "Bit 26 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel18(&mut self) -> SMPSEL18_W<SMPRrs> {
        SMPSEL18_W::new(self, 26)
    }
}
#[doc = "ADC sampling time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMPRrs;
impl crate::RegisterSpec for SMPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpr::R`](R) reader structure"]
impl crate::Readable for SMPRrs {}
#[doc = "`write(|w| ..)` method takes [`smpr::W`](W) writer structure"]
impl crate::Writable for SMPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMPR to value 0"]
impl crate::Resettable for SMPRrs {
    const RESET_VALUE: u32 = 0;
}
