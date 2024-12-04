///Register `SMPR` reader
pub type R = crate::R<SMPRrs>;
///Register `SMPR` writer
pub type W = crate::W<SMPRrs>;
/**Sampling time selection

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
///Field `SMP1` reader - Sampling time selection
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
///Field `SMP1` writer - Sampling time selection
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
///Field `SMP2` reader - Sampling time selection
pub use SMP1_R as SMP2_R;
///Field `SMP2` writer - Sampling time selection
pub use SMP1_W as SMP2_W;
/**Channel sampling time selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SMPSEL {
    ///0: Sampling time of CHANNELx use the setting of SMP1 register
    Smp1 = 0,
    ///1: Sampling time of CHANNELx use the setting of SMP2 register
    Smp2 = 1,
}
impl From<SMPSEL> for u32 {
    #[inline(always)]
    fn from(variant: SMPSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMPSEL {
    type Ux = u32;
}
impl crate::IsEnum for SMPSEL {}
///Field `SMPSEL` reader - Channel sampling time selection
pub type SMPSEL_R = crate::FieldReader<SMPSEL>;
impl SMPSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SMPSEL> {
        match self.bits {
            0 => Some(SMPSEL::Smp1),
            1 => Some(SMPSEL::Smp2),
            _ => None,
        }
    }
    ///Sampling time of CHANNELx use the setting of SMP1 register
    #[inline(always)]
    pub fn is_smp1(&self) -> bool {
        *self == SMPSEL::Smp1
    }
    ///Sampling time of CHANNELx use the setting of SMP2 register
    #[inline(always)]
    pub fn is_smp2(&self) -> bool {
        *self == SMPSEL::Smp2
    }
}
///Field `SMPSEL` writer - Channel sampling time selection
pub type SMPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 19, SMPSEL>;
impl<'a, REG> SMPSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    ///Sampling time of CHANNELx use the setting of SMP1 register
    #[inline(always)]
    pub fn smp1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL::Smp1)
    }
    ///Sampling time of CHANNELx use the setting of SMP2 register
    #[inline(always)]
    pub fn smp2(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL::Smp2)
    }
}
impl R {
    ///Bits 0:2 - Sampling time selection
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Sampling time selection
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:26 - Channel sampling time selection
    #[inline(always)]
    pub fn smpsel(&self) -> SMPSEL_R {
        SMPSEL_R::new((self.bits >> 8) & 0x0007_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR")
            .field("smp1", &self.smp1())
            .field("smp2", &self.smp2())
            .field("smpsel", &self.smpsel())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Sampling time selection
    #[inline(always)]
    pub fn smp1(&mut self) -> SMP1_W<SMPRrs> {
        SMP1_W::new(self, 0)
    }
    ///Bits 4:6 - Sampling time selection
    #[inline(always)]
    pub fn smp2(&mut self) -> SMP2_W<SMPRrs> {
        SMP2_W::new(self, 4)
    }
    ///Bits 8:26 - Channel sampling time selection
    #[inline(always)]
    pub fn smpsel(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 8)
    }
}
/**ADC sampling time register

You can [`read`](crate::Reg::read) this register and get [`smpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#ADC:SMPR)*/
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
