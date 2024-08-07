///Register `SMPR` reader
pub type R = crate::R<SMPRrs>;
///Register `SMPR` writer
pub type W = crate::W<SMPRrs>;
/**SMP1

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
    ///7: 814.5 ADC clock cycles
    Cycles814_5 = 7,
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
///Field `SMP1` reader - SMP1
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
            7 => SMP1::Cycles814_5,
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
    ///814.5 ADC clock cycles
    #[inline(always)]
    pub fn is_cycles814_5(&self) -> bool {
        *self == SMP1::Cycles814_5
    }
}
///Field `SMP1` writer - SMP1
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
    ///814.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles814_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1::Cycles814_5)
    }
}
///Field `SMP2` reader - SMP2
pub use SMP1_R as SMP2_R;
///Field `SMP2` writer - SMP2
pub use SMP1_W as SMP2_W;
/**SMPSEL0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL0 {
    ///0: Sampling time of channel x uses the setting of SMP1 register.
    Smp1 = 0,
    ///1: Sampling time of channel x uses the setting of SMP2 register.
    Smp2 = 1,
}
impl From<SMPSEL0> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL0) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSEL0` reader - SMPSEL0
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
    ///Sampling time of channel x uses the setting of SMP1 register.
    #[inline(always)]
    pub fn is_smp1(&self) -> bool {
        *self == SMPSEL0::Smp1
    }
    ///Sampling time of channel x uses the setting of SMP2 register.
    #[inline(always)]
    pub fn is_smp2(&self) -> bool {
        *self == SMPSEL0::Smp2
    }
}
///Field `SMPSEL0` writer - SMPSEL0
pub type SMPSEL0_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL0>;
impl<'a, REG> SMPSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sampling time of channel x uses the setting of SMP1 register.
    #[inline(always)]
    pub fn smp1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL0::Smp1)
    }
    ///Sampling time of channel x uses the setting of SMP2 register.
    #[inline(always)]
    pub fn smp2(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL0::Smp2)
    }
}
///Field `SMPSEL1` reader - SMPSEL1
pub use SMPSEL0_R as SMPSEL1_R;
///Field `SMPSEL2` reader - SMPSEL2
pub use SMPSEL0_R as SMPSEL2_R;
///Field `SMPSEL3` reader - SMPSEL3
pub use SMPSEL0_R as SMPSEL3_R;
///Field `SMPSEL4` reader - SMPSEL4
pub use SMPSEL0_R as SMPSEL4_R;
///Field `SMPSEL5` reader - SMPSEL5
pub use SMPSEL0_R as SMPSEL5_R;
///Field `SMPSEL6` reader - SMPSEL6
pub use SMPSEL0_R as SMPSEL6_R;
///Field `SMPSEL7` reader - SMPSEL7
pub use SMPSEL0_R as SMPSEL7_R;
///Field `SMPSEL8` reader - SMPSEL8
pub use SMPSEL0_R as SMPSEL8_R;
///Field `SMPSEL9` reader - SMPSEL9
pub use SMPSEL0_R as SMPSEL9_R;
///Field `SMPSEL10` reader - SMPSEL10
pub use SMPSEL0_R as SMPSEL10_R;
///Field `SMPSEL11` reader - SMPSEL11
pub use SMPSEL0_R as SMPSEL11_R;
///Field `SMPSEL12` reader - SMPSEL12
pub use SMPSEL0_R as SMPSEL12_R;
///Field `SMPSEL13` reader - SMPSEL13
pub use SMPSEL0_R as SMPSEL13_R;
///Field `SMPSEL14` reader - SMPSEL14
pub use SMPSEL0_R as SMPSEL14_R;
///Field `SMPSEL15` reader - SMPSEL15
pub use SMPSEL0_R as SMPSEL15_R;
///Field `SMPSEL16` reader - SMPSEL16
pub use SMPSEL0_R as SMPSEL16_R;
///Field `SMPSEL17` reader - SMPSEL17
pub use SMPSEL0_R as SMPSEL17_R;
///Field `SMPSEL18` reader - SMPSEL18
pub use SMPSEL0_R as SMPSEL18_R;
///Field `SMPSEL19` reader - SMPSEL19
pub use SMPSEL0_R as SMPSEL19_R;
///Field `SMPSEL20` reader - SMPSEL20
pub use SMPSEL0_R as SMPSEL20_R;
///Field `SMPSEL21` reader - SMPSEL21
pub use SMPSEL0_R as SMPSEL21_R;
///Field `SMPSEL22` reader - SMPSEL22
pub use SMPSEL0_R as SMPSEL22_R;
///Field `SMPSEL23` reader - SMPSEL23
pub use SMPSEL0_R as SMPSEL23_R;
///Field `SMPSEL1` writer - SMPSEL1
pub use SMPSEL0_W as SMPSEL1_W;
///Field `SMPSEL2` writer - SMPSEL2
pub use SMPSEL0_W as SMPSEL2_W;
///Field `SMPSEL3` writer - SMPSEL3
pub use SMPSEL0_W as SMPSEL3_W;
///Field `SMPSEL4` writer - SMPSEL4
pub use SMPSEL0_W as SMPSEL4_W;
///Field `SMPSEL5` writer - SMPSEL5
pub use SMPSEL0_W as SMPSEL5_W;
///Field `SMPSEL6` writer - SMPSEL6
pub use SMPSEL0_W as SMPSEL6_W;
///Field `SMPSEL7` writer - SMPSEL7
pub use SMPSEL0_W as SMPSEL7_W;
///Field `SMPSEL8` writer - SMPSEL8
pub use SMPSEL0_W as SMPSEL8_W;
///Field `SMPSEL9` writer - SMPSEL9
pub use SMPSEL0_W as SMPSEL9_W;
///Field `SMPSEL10` writer - SMPSEL10
pub use SMPSEL0_W as SMPSEL10_W;
///Field `SMPSEL11` writer - SMPSEL11
pub use SMPSEL0_W as SMPSEL11_W;
///Field `SMPSEL12` writer - SMPSEL12
pub use SMPSEL0_W as SMPSEL12_W;
///Field `SMPSEL13` writer - SMPSEL13
pub use SMPSEL0_W as SMPSEL13_W;
///Field `SMPSEL14` writer - SMPSEL14
pub use SMPSEL0_W as SMPSEL14_W;
///Field `SMPSEL15` writer - SMPSEL15
pub use SMPSEL0_W as SMPSEL15_W;
///Field `SMPSEL16` writer - SMPSEL16
pub use SMPSEL0_W as SMPSEL16_W;
///Field `SMPSEL17` writer - SMPSEL17
pub use SMPSEL0_W as SMPSEL17_W;
///Field `SMPSEL18` writer - SMPSEL18
pub use SMPSEL0_W as SMPSEL18_W;
///Field `SMPSEL19` writer - SMPSEL19
pub use SMPSEL0_W as SMPSEL19_W;
///Field `SMPSEL20` writer - SMPSEL20
pub use SMPSEL0_W as SMPSEL20_W;
///Field `SMPSEL21` writer - SMPSEL21
pub use SMPSEL0_W as SMPSEL21_W;
///Field `SMPSEL22` writer - SMPSEL22
pub use SMPSEL0_W as SMPSEL22_W;
///Field `SMPSEL23` writer - SMPSEL23
pub use SMPSEL0_W as SMPSEL23_W;
impl R {
    ///Bits 0:2 - SMP1
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - SMP2
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 8 - SMPSEL0
    #[inline(always)]
    pub fn smpsel0(&self) -> SMPSEL0_R {
        SMPSEL0_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SMPSEL1
    #[inline(always)]
    pub fn smpsel1(&self) -> SMPSEL1_R {
        SMPSEL1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SMPSEL2
    #[inline(always)]
    pub fn smpsel2(&self) -> SMPSEL2_R {
        SMPSEL2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SMPSEL3
    #[inline(always)]
    pub fn smpsel3(&self) -> SMPSEL3_R {
        SMPSEL3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SMPSEL4
    #[inline(always)]
    pub fn smpsel4(&self) -> SMPSEL4_R {
        SMPSEL4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SMPSEL5
    #[inline(always)]
    pub fn smpsel5(&self) -> SMPSEL5_R {
        SMPSEL5_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SMPSEL6
    #[inline(always)]
    pub fn smpsel6(&self) -> SMPSEL6_R {
        SMPSEL6_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SMPSEL7
    #[inline(always)]
    pub fn smpsel7(&self) -> SMPSEL7_R {
        SMPSEL7_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SMPSEL8
    #[inline(always)]
    pub fn smpsel8(&self) -> SMPSEL8_R {
        SMPSEL8_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SMPSEL9
    #[inline(always)]
    pub fn smpsel9(&self) -> SMPSEL9_R {
        SMPSEL9_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - SMPSEL10
    #[inline(always)]
    pub fn smpsel10(&self) -> SMPSEL10_R {
        SMPSEL10_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - SMPSEL11
    #[inline(always)]
    pub fn smpsel11(&self) -> SMPSEL11_R {
        SMPSEL11_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SMPSEL12
    #[inline(always)]
    pub fn smpsel12(&self) -> SMPSEL12_R {
        SMPSEL12_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SMPSEL13
    #[inline(always)]
    pub fn smpsel13(&self) -> SMPSEL13_R {
        SMPSEL13_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SMPSEL14
    #[inline(always)]
    pub fn smpsel14(&self) -> SMPSEL14_R {
        SMPSEL14_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SMPSEL15
    #[inline(always)]
    pub fn smpsel15(&self) -> SMPSEL15_R {
        SMPSEL15_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - SMPSEL16
    #[inline(always)]
    pub fn smpsel16(&self) -> SMPSEL16_R {
        SMPSEL16_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SMPSEL17
    #[inline(always)]
    pub fn smpsel17(&self) -> SMPSEL17_R {
        SMPSEL17_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SMPSEL18
    #[inline(always)]
    pub fn smpsel18(&self) -> SMPSEL18_R {
        SMPSEL18_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - SMPSEL19
    #[inline(always)]
    pub fn smpsel19(&self) -> SMPSEL19_R {
        SMPSEL19_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SMPSEL20
    #[inline(always)]
    pub fn smpsel20(&self) -> SMPSEL20_R {
        SMPSEL20_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SMPSEL21
    #[inline(always)]
    pub fn smpsel21(&self) -> SMPSEL21_R {
        SMPSEL21_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - SMPSEL22
    #[inline(always)]
    pub fn smpsel22(&self) -> SMPSEL22_R {
        SMPSEL22_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SMPSEL23
    #[inline(always)]
    pub fn smpsel23(&self) -> SMPSEL23_R {
        SMPSEL23_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR")
            .field("smpsel0", &self.smpsel0())
            .field("smpsel23", &self.smpsel23())
            .field("smpsel22", &self.smpsel22())
            .field("smpsel21", &self.smpsel21())
            .field("smpsel20", &self.smpsel20())
            .field("smpsel19", &self.smpsel19())
            .field("smpsel18", &self.smpsel18())
            .field("smpsel17", &self.smpsel17())
            .field("smpsel16", &self.smpsel16())
            .field("smpsel15", &self.smpsel15())
            .field("smpsel14", &self.smpsel14())
            .field("smpsel13", &self.smpsel13())
            .field("smpsel12", &self.smpsel12())
            .field("smpsel11", &self.smpsel11())
            .field("smpsel10", &self.smpsel10())
            .field("smpsel9", &self.smpsel9())
            .field("smpsel8", &self.smpsel8())
            .field("smpsel7", &self.smpsel7())
            .field("smpsel6", &self.smpsel6())
            .field("smpsel5", &self.smpsel5())
            .field("smpsel4", &self.smpsel4())
            .field("smpsel3", &self.smpsel3())
            .field("smpsel2", &self.smpsel2())
            .field("smpsel1", &self.smpsel1())
            .field("smp1", &self.smp1())
            .field("smp2", &self.smp2())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - SMP1
    #[inline(always)]
    #[must_use]
    pub fn smp1(&mut self) -> SMP1_W<SMPRrs> {
        SMP1_W::new(self, 0)
    }
    ///Bits 4:6 - SMP2
    #[inline(always)]
    #[must_use]
    pub fn smp2(&mut self) -> SMP2_W<SMPRrs> {
        SMP2_W::new(self, 4)
    }
    ///Bit 8 - SMPSEL0
    #[inline(always)]
    #[must_use]
    pub fn smpsel0(&mut self) -> SMPSEL0_W<SMPRrs> {
        SMPSEL0_W::new(self, 8)
    }
    ///Bit 9 - SMPSEL1
    #[inline(always)]
    #[must_use]
    pub fn smpsel1(&mut self) -> SMPSEL1_W<SMPRrs> {
        SMPSEL1_W::new(self, 9)
    }
    ///Bit 10 - SMPSEL2
    #[inline(always)]
    #[must_use]
    pub fn smpsel2(&mut self) -> SMPSEL2_W<SMPRrs> {
        SMPSEL2_W::new(self, 10)
    }
    ///Bit 11 - SMPSEL3
    #[inline(always)]
    #[must_use]
    pub fn smpsel3(&mut self) -> SMPSEL3_W<SMPRrs> {
        SMPSEL3_W::new(self, 11)
    }
    ///Bit 12 - SMPSEL4
    #[inline(always)]
    #[must_use]
    pub fn smpsel4(&mut self) -> SMPSEL4_W<SMPRrs> {
        SMPSEL4_W::new(self, 12)
    }
    ///Bit 13 - SMPSEL5
    #[inline(always)]
    #[must_use]
    pub fn smpsel5(&mut self) -> SMPSEL5_W<SMPRrs> {
        SMPSEL5_W::new(self, 13)
    }
    ///Bit 14 - SMPSEL6
    #[inline(always)]
    #[must_use]
    pub fn smpsel6(&mut self) -> SMPSEL6_W<SMPRrs> {
        SMPSEL6_W::new(self, 14)
    }
    ///Bit 15 - SMPSEL7
    #[inline(always)]
    #[must_use]
    pub fn smpsel7(&mut self) -> SMPSEL7_W<SMPRrs> {
        SMPSEL7_W::new(self, 15)
    }
    ///Bit 16 - SMPSEL8
    #[inline(always)]
    #[must_use]
    pub fn smpsel8(&mut self) -> SMPSEL8_W<SMPRrs> {
        SMPSEL8_W::new(self, 16)
    }
    ///Bit 17 - SMPSEL9
    #[inline(always)]
    #[must_use]
    pub fn smpsel9(&mut self) -> SMPSEL9_W<SMPRrs> {
        SMPSEL9_W::new(self, 17)
    }
    ///Bit 18 - SMPSEL10
    #[inline(always)]
    #[must_use]
    pub fn smpsel10(&mut self) -> SMPSEL10_W<SMPRrs> {
        SMPSEL10_W::new(self, 18)
    }
    ///Bit 19 - SMPSEL11
    #[inline(always)]
    #[must_use]
    pub fn smpsel11(&mut self) -> SMPSEL11_W<SMPRrs> {
        SMPSEL11_W::new(self, 19)
    }
    ///Bit 20 - SMPSEL12
    #[inline(always)]
    #[must_use]
    pub fn smpsel12(&mut self) -> SMPSEL12_W<SMPRrs> {
        SMPSEL12_W::new(self, 20)
    }
    ///Bit 21 - SMPSEL13
    #[inline(always)]
    #[must_use]
    pub fn smpsel13(&mut self) -> SMPSEL13_W<SMPRrs> {
        SMPSEL13_W::new(self, 21)
    }
    ///Bit 22 - SMPSEL14
    #[inline(always)]
    #[must_use]
    pub fn smpsel14(&mut self) -> SMPSEL14_W<SMPRrs> {
        SMPSEL14_W::new(self, 22)
    }
    ///Bit 23 - SMPSEL15
    #[inline(always)]
    #[must_use]
    pub fn smpsel15(&mut self) -> SMPSEL15_W<SMPRrs> {
        SMPSEL15_W::new(self, 23)
    }
    ///Bit 24 - SMPSEL16
    #[inline(always)]
    #[must_use]
    pub fn smpsel16(&mut self) -> SMPSEL16_W<SMPRrs> {
        SMPSEL16_W::new(self, 24)
    }
    ///Bit 25 - SMPSEL17
    #[inline(always)]
    #[must_use]
    pub fn smpsel17(&mut self) -> SMPSEL17_W<SMPRrs> {
        SMPSEL17_W::new(self, 25)
    }
    ///Bit 26 - SMPSEL18
    #[inline(always)]
    #[must_use]
    pub fn smpsel18(&mut self) -> SMPSEL18_W<SMPRrs> {
        SMPSEL18_W::new(self, 26)
    }
    ///Bit 27 - SMPSEL19
    #[inline(always)]
    #[must_use]
    pub fn smpsel19(&mut self) -> SMPSEL19_W<SMPRrs> {
        SMPSEL19_W::new(self, 27)
    }
    ///Bit 28 - SMPSEL20
    #[inline(always)]
    #[must_use]
    pub fn smpsel20(&mut self) -> SMPSEL20_W<SMPRrs> {
        SMPSEL20_W::new(self, 28)
    }
    ///Bit 29 - SMPSEL21
    #[inline(always)]
    #[must_use]
    pub fn smpsel21(&mut self) -> SMPSEL21_W<SMPRrs> {
        SMPSEL21_W::new(self, 29)
    }
    ///Bit 30 - SMPSEL22
    #[inline(always)]
    #[must_use]
    pub fn smpsel22(&mut self) -> SMPSEL22_W<SMPRrs> {
        SMPSEL22_W::new(self, 30)
    }
    ///Bit 31 - SMPSEL23
    #[inline(always)]
    #[must_use]
    pub fn smpsel23(&mut self) -> SMPSEL23_W<SMPRrs> {
        SMPSEL23_W::new(self, 31)
    }
}
/**ADC sample time register

You can [`read`](crate::Reg::read) this register and get [`smpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#ADC4:SMPR)*/
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
