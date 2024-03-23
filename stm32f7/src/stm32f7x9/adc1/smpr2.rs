#[doc = "Register `SMPR2` reader"]
pub type R = crate::R<SMPR2rs>;
#[doc = "Register `SMPR2` writer"]
pub type W = crate::W<SMPR2rs>;
#[doc = "Sample time bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SMPX_X {
    #[doc = "0: 3 cycles"]
    Cycles3 = 0,
    #[doc = "1: 15 cycles"]
    Cycles15 = 1,
    #[doc = "2: 28 cycles"]
    Cycles28 = 2,
    #[doc = "3: 56 cycles"]
    Cycles56 = 3,
    #[doc = "4: 84 cycles"]
    Cycles84 = 4,
    #[doc = "5: 112 cycles"]
    Cycles112 = 5,
    #[doc = "6: 144 cycles"]
    Cycles144 = 6,
    #[doc = "7: 480 cycles"]
    Cycles480 = 7,
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
            0 => Some(SMPX_X::Cycles3),
            1 => Some(SMPX_X::Cycles15),
            2 => Some(SMPX_X::Cycles28),
            3 => Some(SMPX_X::Cycles56),
            4 => Some(SMPX_X::Cycles84),
            5 => Some(SMPX_X::Cycles112),
            6 => Some(SMPX_X::Cycles144),
            7 => Some(SMPX_X::Cycles480),
            _ => None,
        }
    }
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn is_cycles3(&self) -> bool {
        *self == SMPX_X::Cycles3
    }
    #[doc = "15 cycles"]
    #[inline(always)]
    pub fn is_cycles15(&self) -> bool {
        *self == SMPX_X::Cycles15
    }
    #[doc = "28 cycles"]
    #[inline(always)]
    pub fn is_cycles28(&self) -> bool {
        *self == SMPX_X::Cycles28
    }
    #[doc = "56 cycles"]
    #[inline(always)]
    pub fn is_cycles56(&self) -> bool {
        *self == SMPX_X::Cycles56
    }
    #[doc = "84 cycles"]
    #[inline(always)]
    pub fn is_cycles84(&self) -> bool {
        *self == SMPX_X::Cycles84
    }
    #[doc = "112 cycles"]
    #[inline(always)]
    pub fn is_cycles112(&self) -> bool {
        *self == SMPX_X::Cycles112
    }
    #[doc = "144 cycles"]
    #[inline(always)]
    pub fn is_cycles144(&self) -> bool {
        *self == SMPX_X::Cycles144
    }
    #[doc = "480 cycles"]
    #[inline(always)]
    pub fn is_cycles480(&self) -> bool {
        *self == SMPX_X::Cycles480
    }
}
#[doc = "Field `SMPx_x` writer - Sample time bits"]
pub type SMPX_X_W<'a, REG> = crate::FieldWriter<'a, REG, 32, SMPX_X>;
impl<'a, REG> SMPX_X_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn cycles3(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles3)
    }
    #[doc = "15 cycles"]
    #[inline(always)]
    pub fn cycles15(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles15)
    }
    #[doc = "28 cycles"]
    #[inline(always)]
    pub fn cycles28(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles28)
    }
    #[doc = "56 cycles"]
    #[inline(always)]
    pub fn cycles56(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles56)
    }
    #[doc = "84 cycles"]
    #[inline(always)]
    pub fn cycles84(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles84)
    }
    #[doc = "112 cycles"]
    #[inline(always)]
    pub fn cycles112(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles112)
    }
    #[doc = "144 cycles"]
    #[inline(always)]
    pub fn cycles144(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles144)
    }
    #[doc = "480 cycles"]
    #[inline(always)]
    pub fn cycles480(self) -> &'a mut crate::W<REG> {
        self.variant(SMPX_X::Cycles480)
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
