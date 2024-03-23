#[doc = "Register `SMPR2` reader"]
pub type R = crate::R<SMPR2rs>;
#[doc = "Register `SMPR2` writer"]
pub type W = crate::W<SMPR2rs>;
#[doc = "Sample time bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SMPX_X {
    #[doc = "0: 1.5 ADC clock cycles"]
    Cycles1_5 = 0,
    #[doc = "1: 7.5 ADC clock cycles"]
    Cycles7_5 = 1,
    #[doc = "2: 13.5 ADC clock cycles"]
    Cycles13_5 = 2,
    #[doc = "3: 28.5 ADC clock cycles"]
    Cycles28_5 = 3,
    #[doc = "4: 41.5 ADC clock cycles"]
    Cycles41_5 = 4,
    #[doc = "5: 55.5 ADC clock cycles"]
    Cycles55_5 = 5,
    #[doc = "6: 71.5 ADC clock cycles"]
    Cycles71_5 = 6,
    #[doc = "7: 239.5 ADC clock cycles"]
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
#[doc = "Field `SMPx_x` reader - Sample time bits"]
pub type SMPX_X_R = crate::FieldReader<SMPX_X>;
impl SMPX_X_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SMPX_X::Cycles1_5
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SMPX_X::Cycles7_5
    }
    #[doc = "13.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles13_5(&self) -> bool {
        *self == SMPX_X::Cycles13_5
    }
    #[doc = "28.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles28_5(&self) -> bool {
        *self == SMPX_X::Cycles28_5
    }
    #[doc = "41.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles41_5(&self) -> bool {
        *self == SMPX_X::Cycles41_5
    }
    #[doc = "55.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles55_5(&self) -> bool {
        *self == SMPX_X::Cycles55_5
    }
    #[doc = "71.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles71_5(&self) -> bool {
        *self == SMPX_X::Cycles71_5
    }
    #[doc = "239.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_cycles239_5(&self) -> bool {
        *self == SMPX_X::Cycles239_5
    }
}
#[doc = "Field `SMPx_x` writer - Sample time bits"]
pub type SMPX_X_W<'a, REG> = crate::FieldWriter<'a, REG, 32, SMPX_X>;
impl<'a, REG> SMPX_X_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles1_5)
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles7_5)
    }
    #[doc = "13.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles13_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles13_5)
    }
    #[doc = "28.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles28_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles28_5)
    }
    #[doc = "41.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles41_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles41_5)
    }
    #[doc = "55.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles55_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles55_5)
    }
    #[doc = "71.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles71_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles71_5)
    }
    #[doc = "239.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles239_5(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles239_5)
    }
}
impl R {
    #[doc = "Bits 0:31 - Sample time bits"]
    #[inline(always)]
    pub fn smpx_x(&self) -> SMPX_X_R {
        SMPX_X_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Sample time bits"]
    #[inline(always)]
    #[must_use]
    pub fn smpx_x(&mut self) -> SMPX_X_W<SMPR2rs> {
        SMPX_X_W::new(self, 0)
    }
}
#[doc = "sample time register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMPR2rs;
impl crate::RegisterSpec for SMPR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpr2::R`](R) reader structure"]
impl crate::Readable for SMPR2rs {}
#[doc = "`write(|w| ..)` method takes [`smpr2::W`](W) writer structure"]
impl crate::Writable for SMPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMPR2 to value 0"]
impl crate::Resettable for SMPR2rs {
    const RESET_VALUE: u32 = 0;
}
