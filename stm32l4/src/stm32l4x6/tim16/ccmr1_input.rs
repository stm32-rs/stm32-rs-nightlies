///Register `CCMR1_Input` reader
pub type R = crate::R<CCMR1_INPUTrs>;
///Register `CCMR1_Input` writer
pub type W = crate::W<CCMR1_INPUTrs>;
///Field `CCS(1-1)` reader - Capture/Compare %s selection
pub type CCS_R = crate::FieldReader;
///Field `CCS(1-1)` writer - Capture/Compare %s selection
pub type CCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ICPSC(1-1)` reader - Input capture %s prescaler
pub type ICPSC_R = crate::FieldReader;
///Field `ICPSC(1-1)` writer - Input capture %s prescaler
pub type ICPSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ICF(1-1)` reader - Input capture %s filter
pub type ICF_R = crate::FieldReader;
///Field `ICF(1-1)` writer - Input capture %s filter
pub type ICF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Capture/Compare (1-1) selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1S` field.</div>
    #[inline(always)]
    pub fn ccs(&self, n: u8) -> CCS_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCS_R::new(((self.bits >> (n * 0)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Capture/Compare (1-1) selection
    #[inline(always)]
    pub fn ccs_iter(&self) -> impl Iterator<Item = CCS_R> + '_ {
        (0..1).map(move |n| CCS_R::new(((self.bits >> (n * 0)) & 3) as u8))
    }
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    pub fn cc1s(&self) -> CCS_R {
        CCS_R::new((self.bits & 3) as u8)
    }
    ///Input capture (1-1) prescaler
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `IC1PSC` field.</div>
    #[inline(always)]
    pub fn icpsc(&self, n: u8) -> ICPSC_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        ICPSC_R::new(((self.bits >> (n * 0 + 2)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Input capture (1-1) prescaler
    #[inline(always)]
    pub fn icpsc_iter(&self) -> impl Iterator<Item = ICPSC_R> + '_ {
        (0..1).map(move |n| ICPSC_R::new(((self.bits >> (n * 0 + 2)) & 3) as u8))
    }
    ///Bits 2:3 - Input capture 1 prescaler
    #[inline(always)]
    pub fn ic1psc(&self) -> ICPSC_R {
        ICPSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Input capture (1-1) filter
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `IC1F` field.</div>
    #[inline(always)]
    pub fn icf(&self, n: u8) -> ICF_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        ICF_R::new(((self.bits >> (n * 0 + 4)) & 0x0f) as u8)
    }
    ///Iterator for array of:
    ///Input capture (1-1) filter
    #[inline(always)]
    pub fn icf_iter(&self) -> impl Iterator<Item = ICF_R> + '_ {
        (0..1).map(move |n| ICF_R::new(((self.bits >> (n * 0 + 4)) & 0x0f) as u8))
    }
    ///Bits 4:7 - Input capture 1 filter
    #[inline(always)]
    pub fn ic1f(&self) -> ICF_R {
        ICF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR1_Input")
            .field("ic1f", &self.ic1f())
            .field("ic1psc", &self.ic1psc())
            .field("cc1s", &self.cc1s())
            .finish()
    }
}
impl W {
    ///Capture/Compare (1-1) selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1S` field.</div>
    #[inline(always)]
    #[must_use]
    pub fn ccs(&mut self, n: u8) -> CCS_W<CCMR1_INPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCS_W::new(self, n * 0)
    }
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CCS_W<CCMR1_INPUTrs> {
        CCS_W::new(self, 0)
    }
    ///Input capture (1-1) prescaler
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `IC1PSC` field.</div>
    #[inline(always)]
    #[must_use]
    pub fn icpsc(&mut self, n: u8) -> ICPSC_W<CCMR1_INPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        ICPSC_W::new(self, n * 0 + 2)
    }
    ///Bits 2:3 - Input capture 1 prescaler
    #[inline(always)]
    #[must_use]
    pub fn ic1psc(&mut self) -> ICPSC_W<CCMR1_INPUTrs> {
        ICPSC_W::new(self, 2)
    }
    ///Input capture (1-1) filter
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `IC1F` field.</div>
    #[inline(always)]
    #[must_use]
    pub fn icf(&mut self, n: u8) -> ICF_W<CCMR1_INPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        ICF_W::new(self, n * 0 + 4)
    }
    ///Bits 4:7 - Input capture 1 filter
    #[inline(always)]
    #[must_use]
    pub fn ic1f(&mut self) -> ICF_W<CCMR1_INPUTrs> {
        ICF_W::new(self, 4)
    }
}
/**capture/compare mode register 1 (input mode)

You can [`read`](crate::Reg::read) this register and get [`ccmr1_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x6.html#TIM16:CCMR1_Input)*/
pub struct CCMR1_INPUTrs;
impl crate::RegisterSpec for CCMR1_INPUTrs {
    type Ux = u32;
}
///`read()` method returns [`ccmr1_input::R`](R) reader structure
impl crate::Readable for CCMR1_INPUTrs {}
///`write(|w| ..)` method takes [`ccmr1_input::W`](W) writer structure
impl crate::Writable for CCMR1_INPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCMR1_Input to value 0
impl crate::Resettable for CCMR1_INPUTrs {
    const RESET_VALUE: u32 = 0;
}
