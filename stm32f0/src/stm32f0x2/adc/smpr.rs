///Register `SMPR` reader
pub type R = crate::R<SMPRrs>;
///Register `SMPR` writer
pub type W = crate::W<SMPRrs>;
/**Sampling time selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP {
    ///0: 1.5 cycles
    Cycles1_5 = 0,
    ///1: 7.5 cycles
    Cycles7_5 = 1,
    ///2: 13.5 cycles
    Cycles13_5 = 2,
    ///3: 28.5 cycles
    Cycles28_5 = 3,
    ///4: 41.5 cycles
    Cycles41_5 = 4,
    ///5: 55.5 cycles
    Cycles55_5 = 5,
    ///6: 71.5 cycles
    Cycles71_5 = 6,
    ///7: 239.5 cycles
    Cycles239_5 = 7,
}
impl From<SMP> for u8 {
    #[inline(always)]
    fn from(variant: SMP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP {
    type Ux = u8;
}
impl crate::IsEnum for SMP {}
///Field `SMP` reader - Sampling time selection
pub type SMP_R = crate::FieldReader<SMP>;
impl SMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMP {
        match self.bits {
            0 => SMP::Cycles1_5,
            1 => SMP::Cycles7_5,
            2 => SMP::Cycles13_5,
            3 => SMP::Cycles28_5,
            4 => SMP::Cycles41_5,
            5 => SMP::Cycles55_5,
            6 => SMP::Cycles71_5,
            7 => SMP::Cycles239_5,
            _ => unreachable!(),
        }
    }
    ///1.5 cycles
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SMP::Cycles1_5
    }
    ///7.5 cycles
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SMP::Cycles7_5
    }
    ///13.5 cycles
    #[inline(always)]
    pub fn is_cycles13_5(&self) -> bool {
        *self == SMP::Cycles13_5
    }
    ///28.5 cycles
    #[inline(always)]
    pub fn is_cycles28_5(&self) -> bool {
        *self == SMP::Cycles28_5
    }
    ///41.5 cycles
    #[inline(always)]
    pub fn is_cycles41_5(&self) -> bool {
        *self == SMP::Cycles41_5
    }
    ///55.5 cycles
    #[inline(always)]
    pub fn is_cycles55_5(&self) -> bool {
        *self == SMP::Cycles55_5
    }
    ///71.5 cycles
    #[inline(always)]
    pub fn is_cycles71_5(&self) -> bool {
        *self == SMP::Cycles71_5
    }
    ///239.5 cycles
    #[inline(always)]
    pub fn is_cycles239_5(&self) -> bool {
        *self == SMP::Cycles239_5
    }
}
///Field `SMP` writer - Sampling time selection
pub type SMP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP, crate::Safe>;
impl<'a, REG> SMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1.5 cycles
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP::Cycles1_5)
    }
    ///7.5 cycles
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP::Cycles7_5)
    }
    ///13.5 cycles
    #[inline(always)]
    pub fn cycles13_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP::Cycles13_5)
    }
    ///28.5 cycles
    #[inline(always)]
    pub fn cycles28_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP::Cycles28_5)
    }
    ///41.5 cycles
    #[inline(always)]
    pub fn cycles41_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP::Cycles41_5)
    }
    ///55.5 cycles
    #[inline(always)]
    pub fn cycles55_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP::Cycles55_5)
    }
    ///71.5 cycles
    #[inline(always)]
    pub fn cycles71_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP::Cycles71_5)
    }
    ///239.5 cycles
    #[inline(always)]
    pub fn cycles239_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP::Cycles239_5)
    }
}
impl R {
    ///Bits 0:2 - Sampling time selection
    #[inline(always)]
    pub fn smp(&self) -> SMP_R {
        SMP_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPR").field("smp", &self.smp()).finish()
    }
}
impl W {
    ///Bits 0:2 - Sampling time selection
    #[inline(always)]
    pub fn smp(&mut self) -> SMP_W<'_, SMPRrs> {
        SMP_W::new(self, 0)
    }
}
/**sampling time register

You can [`read`](crate::Reg::read) this register and get [`smpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x2.html#ADC:SMPR)*/
pub struct SMPRrs;
impl crate::RegisterSpec for SMPRrs {
    type Ux = u32;
}
///`read()` method returns [`smpr::R`](R) reader structure
impl crate::Readable for SMPRrs {}
///`write(|w| ..)` method takes [`smpr::W`](W) writer structure
impl crate::Writable for SMPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMPR to value 0
impl crate::Resettable for SMPRrs {}
