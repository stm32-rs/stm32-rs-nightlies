#[doc = "Register `CCMR1_Input` reader"]
pub type R = crate::R<CCMR1_INPUTrs>;
#[doc = "Register `CCMR1_Input` writer"]
pub type W = crate::W<CCMR1_INPUTrs>;
#[doc = "Field `CCS(1-1)` reader - Capture/Compare %s selection"]
pub type CCS_R = crate::FieldReader;
#[doc = "Field `CCS(1-1)` writer - Capture/Compare %s selection"]
pub type CCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ICPSC(1-1)` reader - Input capture %s prescaler"]
pub type ICPSC_R = crate::FieldReader;
#[doc = "Field `ICPSC(1-1)` writer - Input capture %s prescaler"]
pub type ICPSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ICF(1-1)` reader - Input capture %s filter"]
pub type ICF_R = crate::FieldReader;
#[doc = "Field `ICF(1-1)` writer - Input capture %s filter"]
pub type ICF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Capture/Compare (1-1) selection"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1S` field"]
    #[inline(always)]
    pub fn ccs(&self, n: u8) -> CCS_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCS_R::new(((self.bits >> (n * 0)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Capture/Compare (1-1) selection"]
    #[inline(always)]
    pub fn ccs_iter(&self) -> impl Iterator<Item = CCS_R> + '_ {
        (0..1).map(move |n| CCS_R::new(((self.bits >> (n * 0)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1s(&self) -> CCS_R {
        CCS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Input capture (1-1) prescaler"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `IC1PSC` field"]
    #[inline(always)]
    pub fn icpsc(&self, n: u8) -> ICPSC_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        ICPSC_R::new(((self.bits >> (n * 0 + 2)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Input capture (1-1) prescaler"]
    #[inline(always)]
    pub fn icpsc_iter(&self) -> impl Iterator<Item = ICPSC_R> + '_ {
        (0..1).map(move |n| ICPSC_R::new(((self.bits >> (n * 0 + 2)) & 3) as u8))
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline(always)]
    pub fn ic1psc(&self) -> ICPSC_R {
        ICPSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Input capture (1-1) filter"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `IC1F` field"]
    #[inline(always)]
    pub fn icf(&self, n: u8) -> ICF_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        ICF_R::new(((self.bits >> (n * 0 + 4)) & 0x0f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Input capture (1-1) filter"]
    #[inline(always)]
    pub fn icf_iter(&self) -> impl Iterator<Item = ICF_R> + '_ {
        (0..1).map(move |n| ICF_R::new(((self.bits >> (n * 0 + 4)) & 0x0f) as u8))
    }
    #[doc = "Bits 4:7 - Input capture 1 filter"]
    #[inline(always)]
    pub fn ic1f(&self) -> ICF_R {
        ICF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Capture/Compare (1-1) selection"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1S` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccs(&mut self, n: u8) -> CCS_W<CCMR1_INPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCS_W::new(self, n * 0)
    }
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CCS_W<CCMR1_INPUTrs> {
        CCS_W::new(self, 0)
    }
    #[doc = "Input capture (1-1) prescaler"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `IC1PSC` field"]
    #[inline(always)]
    #[must_use]
    pub fn icpsc(&mut self, n: u8) -> ICPSC_W<CCMR1_INPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        ICPSC_W::new(self, n * 0 + 2)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ic1psc(&mut self) -> ICPSC_W<CCMR1_INPUTrs> {
        ICPSC_W::new(self, 2)
    }
    #[doc = "Input capture (1-1) filter"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `IC1F` field"]
    #[inline(always)]
    #[must_use]
    pub fn icf(&mut self, n: u8) -> ICF_W<CCMR1_INPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        ICF_W::new(self, n * 0 + 4)
    }
    #[doc = "Bits 4:7 - Input capture 1 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ic1f(&mut self) -> ICF_W<CCMR1_INPUTrs> {
        ICF_W::new(self, 4)
    }
}
#[doc = "capture/compare mode register (input mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr1_input::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr1_input::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR1_INPUTrs;
impl crate::RegisterSpec for CCMR1_INPUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr1_input::R`](R) reader structure"]
impl crate::Readable for CCMR1_INPUTrs {}
#[doc = "`write(|w| ..)` method takes [`ccmr1_input::W`](W) writer structure"]
impl crate::Writable for CCMR1_INPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMR1_Input to value 0"]
impl crate::Resettable for CCMR1_INPUTrs {
    const RESET_VALUE: u32 = 0;
}
