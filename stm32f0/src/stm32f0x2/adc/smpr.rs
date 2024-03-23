#[doc = "Register `SMPR` reader"]
pub type R = crate::R<SMPRrs>;
#[doc = "Register `SMPR` writer"]
pub type W = crate::W<SMPRrs>;
#[doc = "Sampling time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP {
    #[doc = "0: 1.5 cycles"]
    Cycles1_5 = 0,
    #[doc = "1: 7.5 cycles"]
    Cycles7_5 = 1,
    #[doc = "2: 13.5 cycles"]
    Cycles13_5 = 2,
    #[doc = "3: 28.5 cycles"]
    Cycles28_5 = 3,
    #[doc = "4: 41.5 cycles"]
    Cycles41_5 = 4,
    #[doc = "5: 55.5 cycles"]
    Cycles55_5 = 5,
    #[doc = "6: 71.5 cycles"]
    Cycles71_5 = 6,
    #[doc = "7: 239.5 cycles"]
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
#[doc = "Field `SMP` reader - Sampling time selection"]
pub type SMP_R = crate::FieldReader<SMP>;
impl SMP_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "1.5 cycles"]
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SMP::Cycles1_5
    }
    #[doc = "7.5 cycles"]
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SMP::Cycles7_5
    }
    #[doc = "13.5 cycles"]
    #[inline(always)]
    pub fn is_cycles13_5(&self) -> bool {
        *self == SMP::Cycles13_5
    }
    #[doc = "28.5 cycles"]
    #[inline(always)]
    pub fn is_cycles28_5(&self) -> bool {
        *self == SMP::Cycles28_5
    }
    #[doc = "41.5 cycles"]
    #[inline(always)]
    pub fn is_cycles41_5(&self) -> bool {
        *self == SMP::Cycles41_5
    }
    #[doc = "55.5 cycles"]
    #[inline(always)]
    pub fn is_cycles55_5(&self) -> bool {
        *self == SMP::Cycles55_5
    }
    #[doc = "71.5 cycles"]
    #[inline(always)]
    pub fn is_cycles71_5(&self) -> bool {
        *self == SMP::Cycles71_5
    }
    #[doc = "239.5 cycles"]
    #[inline(always)]
    pub fn is_cycles239_5(&self) -> bool {
        *self == SMP::Cycles239_5
    }
}
#[doc = "Field `SMP` writer - Sampling time selection"]
pub type SMP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SMP>;
impl<'a, REG> SMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.5 cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP::Cycles1_5)
    }
    #[doc = "7.5 cycles"]
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP::Cycles7_5)
    }
    #[doc = "13.5 cycles"]
    #[inline(always)]
    pub fn cycles13_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP::Cycles13_5)
    }
    #[doc = "28.5 cycles"]
    #[inline(always)]
    pub fn cycles28_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP::Cycles28_5)
    }
    #[doc = "41.5 cycles"]
    #[inline(always)]
    pub fn cycles41_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP::Cycles41_5)
    }
    #[doc = "55.5 cycles"]
    #[inline(always)]
    pub fn cycles55_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP::Cycles55_5)
    }
    #[doc = "71.5 cycles"]
    #[inline(always)]
    pub fn cycles71_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP::Cycles71_5)
    }
    #[doc = "239.5 cycles"]
    #[inline(always)]
    pub fn cycles239_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP::Cycles239_5)
    }
}
impl R {
    #[doc = "Bits 0:2 - Sampling time selection"]
    #[inline(always)]
    pub fn smp(&self) -> SMP_R {
        SMP_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp(&mut self) -> SMP_W<SMPRrs> {
        SMP_W::new(self, 0)
    }
}
#[doc = "sampling time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
