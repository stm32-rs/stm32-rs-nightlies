#[doc = "Register `CHSELR0` reader"]
pub type R = crate::R<CHSELR0rs>;
#[doc = "Register `CHSELR0` writer"]
pub type W = crate::W<CHSELR0rs>;
#[doc = "CHSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum CHSEL {
    #[doc = "0: Input Channel is not selected for conversion"]
    NotSelected = 0,
    #[doc = "1: Input Channel is selected for conversion"]
    Selected = 1,
}
impl From<CHSEL> for u32 {
    #[inline(always)]
    fn from(variant: CHSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHSEL {
    type Ux = u32;
}
#[doc = "Field `CHSEL` reader - CHSEL"]
pub type CHSEL_R = crate::FieldReader<CHSEL>;
impl CHSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CHSEL> {
        match self.bits {
            0 => Some(CHSEL::NotSelected),
            1 => Some(CHSEL::Selected),
            _ => None,
        }
    }
    #[doc = "Input Channel is not selected for conversion"]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == CHSEL::NotSelected
    }
    #[doc = "Input Channel is selected for conversion"]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == CHSEL::Selected
    }
}
#[doc = "Field `CHSEL` writer - CHSEL"]
pub type CHSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 18, CHSEL>;
impl<'a, REG> CHSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Input Channel is not selected for conversion"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL::NotSelected)
    }
    #[doc = "Input Channel is selected for conversion"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL::Selected)
    }
}
impl R {
    #[doc = "Bits 0:17 - CHSEL"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - CHSEL"]
    #[inline(always)]
    #[must_use]
    pub fn chsel(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 0)
    }
}
#[doc = "channel selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chselr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chselr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHSELR0rs;
impl crate::RegisterSpec for CHSELR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chselr0::R`](R) reader structure"]
impl crate::Readable for CHSELR0rs {}
#[doc = "`write(|w| ..)` method takes [`chselr0::W`](W) writer structure"]
impl crate::Writable for CHSELR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHSELR0 to value 0"]
impl crate::Resettable for CHSELR0rs {
    const RESET_VALUE: u32 = 0;
}
