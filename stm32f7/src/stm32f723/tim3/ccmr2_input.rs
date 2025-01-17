///Register `CCMR2_Input` reader
pub type R = crate::R<CCMR2_INPUTrs>;
///Register `CCMR2_Input` writer
pub type W = crate::W<CCMR2_INPUTrs>;
/**Capture/Compare %s selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC3S {
    ///1: CC3 channel is configured as input, IC3 is mapped on TI3
    Ti3 = 1,
    ///2: CC3 channel is configured as input, IC3 is mapped on TI4
    Ti4 = 2,
    ///3: CC3 channel is configured as input, IC3 is mapped on TRC
    Trc = 3,
}
impl From<CC3S> for u8 {
    #[inline(always)]
    fn from(variant: CC3S) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC3S {
    type Ux = u8;
}
impl crate::IsEnum for CC3S {}
///Field `CCS(3-4)` reader - Capture/Compare %s selection
pub type CCS_R = crate::FieldReader<CC3S>;
impl CCS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CC3S> {
        match self.bits {
            1 => Some(CC3S::Ti3),
            2 => Some(CC3S::Ti4),
            3 => Some(CC3S::Trc),
            _ => None,
        }
    }
    ///CC3 channel is configured as input, IC3 is mapped on TI3
    #[inline(always)]
    pub fn is_ti3(&self) -> bool {
        *self == CC3S::Ti3
    }
    ///CC3 channel is configured as input, IC3 is mapped on TI4
    #[inline(always)]
    pub fn is_ti4(&self) -> bool {
        *self == CC3S::Ti4
    }
    ///CC3 channel is configured as input, IC3 is mapped on TRC
    #[inline(always)]
    pub fn is_trc(&self) -> bool {
        *self == CC3S::Trc
    }
}
///Field `CCS(3-4)` writer - Capture/Compare %s selection
pub type CCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC3S>;
impl<'a, REG> CCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CC3 channel is configured as input, IC3 is mapped on TI3
    #[inline(always)]
    pub fn ti3(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S::Ti3)
    }
    ///CC3 channel is configured as input, IC3 is mapped on TI4
    #[inline(always)]
    pub fn ti4(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S::Ti4)
    }
    ///CC3 channel is configured as input, IC3 is mapped on TRC
    #[inline(always)]
    pub fn trc(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S::Trc)
    }
}
///Field `ICPSC(3-4)` reader - Input capture %s prescaler
pub type ICPSC_R = crate::FieldReader;
///Field `ICPSC(3-4)` writer - Input capture %s prescaler
pub type ICPSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
///Field `ICF(3-4)` reader - Input capture %s filter
pub type ICF_R = crate::FieldReader;
///Field `ICF(3-4)` writer - Input capture %s filter
pub type ICF_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    ///Capture/Compare (3-4) selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC3S` field.</div>
    #[inline(always)]
    pub fn ccs(&self, n: u8) -> CCS_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CCS_R::new(((self.bits >> (n * 8)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Capture/Compare (3-4) selection
    #[inline(always)]
    pub fn ccs_iter(&self) -> impl Iterator<Item = CCS_R> + '_ {
        (0..2).map(move |n| CCS_R::new(((self.bits >> (n * 8)) & 3) as u8))
    }
    ///Bits 0:1 - Capture/Compare 3 selection
    #[inline(always)]
    pub fn cc3s(&self) -> CCS_R {
        CCS_R::new((self.bits & 3) as u8)
    }
    ///Bits 8:9 - Capture/Compare 4 selection
    #[inline(always)]
    pub fn cc4s(&self) -> CCS_R {
        CCS_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Input capture (3-4) prescaler
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `IC3PSC` field.</div>
    #[inline(always)]
    pub fn icpsc(&self, n: u8) -> ICPSC_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ICPSC_R::new(((self.bits >> (n * 8 + 2)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Input capture (3-4) prescaler
    #[inline(always)]
    pub fn icpsc_iter(&self) -> impl Iterator<Item = ICPSC_R> + '_ {
        (0..2).map(move |n| ICPSC_R::new(((self.bits >> (n * 8 + 2)) & 3) as u8))
    }
    ///Bits 2:3 - Input capture 3 prescaler
    #[inline(always)]
    pub fn ic3psc(&self) -> ICPSC_R {
        ICPSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 10:11 - Input capture 4 prescaler
    #[inline(always)]
    pub fn ic4psc(&self) -> ICPSC_R {
        ICPSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Input capture (3-4) filter
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `IC3F` field.</div>
    #[inline(always)]
    pub fn icf(&self, n: u8) -> ICF_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ICF_R::new(((self.bits >> (n * 8 + 4)) & 0x0f) as u8)
    }
    ///Iterator for array of:
    ///Input capture (3-4) filter
    #[inline(always)]
    pub fn icf_iter(&self) -> impl Iterator<Item = ICF_R> + '_ {
        (0..2).map(move |n| ICF_R::new(((self.bits >> (n * 8 + 4)) & 0x0f) as u8))
    }
    ///Bits 4:7 - Input capture 3 filter
    #[inline(always)]
    pub fn ic3f(&self) -> ICF_R {
        ICF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 12:15 - Input capture 4 filter
    #[inline(always)]
    pub fn ic4f(&self) -> ICF_R {
        ICF_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR2_Input")
            .field("ic3f", &self.ic3f())
            .field("ic4f", &self.ic4f())
            .field("ic3psc", &self.ic3psc())
            .field("ic4psc", &self.ic4psc())
            .field("cc3s", &self.cc3s())
            .field("cc4s", &self.cc4s())
            .finish()
    }
}
impl W {
    ///Capture/Compare (3-4) selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC3S` field.</div>
    #[inline(always)]
    pub fn ccs(&mut self, n: u8) -> CCS_W<CCMR2_INPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CCS_W::new(self, n * 8)
    }
    ///Bits 0:1 - Capture/Compare 3 selection
    #[inline(always)]
    pub fn cc3s(&mut self) -> CCS_W<CCMR2_INPUTrs> {
        CCS_W::new(self, 0)
    }
    ///Bits 8:9 - Capture/Compare 4 selection
    #[inline(always)]
    pub fn cc4s(&mut self) -> CCS_W<CCMR2_INPUTrs> {
        CCS_W::new(self, 8)
    }
    ///Input capture (3-4) prescaler
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `IC3PSC` field.</div>
    #[inline(always)]
    pub fn icpsc(&mut self, n: u8) -> ICPSC_W<CCMR2_INPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ICPSC_W::new(self, n * 8 + 2)
    }
    ///Bits 2:3 - Input capture 3 prescaler
    #[inline(always)]
    pub fn ic3psc(&mut self) -> ICPSC_W<CCMR2_INPUTrs> {
        ICPSC_W::new(self, 2)
    }
    ///Bits 10:11 - Input capture 4 prescaler
    #[inline(always)]
    pub fn ic4psc(&mut self) -> ICPSC_W<CCMR2_INPUTrs> {
        ICPSC_W::new(self, 10)
    }
    ///Input capture (3-4) filter
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `IC3F` field.</div>
    #[inline(always)]
    pub fn icf(&mut self, n: u8) -> ICF_W<CCMR2_INPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ICF_W::new(self, n * 8 + 4)
    }
    ///Bits 4:7 - Input capture 3 filter
    #[inline(always)]
    pub fn ic3f(&mut self) -> ICF_W<CCMR2_INPUTrs> {
        ICF_W::new(self, 4)
    }
    ///Bits 12:15 - Input capture 4 filter
    #[inline(always)]
    pub fn ic4f(&mut self) -> ICF_W<CCMR2_INPUTrs> {
        ICF_W::new(self, 12)
    }
}
/**capture/compare mode register 2 (input mode)

You can [`read`](crate::Reg::read) this register and get [`ccmr2_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F723.html#TIM3:CCMR2_Input)*/
pub struct CCMR2_INPUTrs;
impl crate::RegisterSpec for CCMR2_INPUTrs {
    type Ux = u32;
}
///`read()` method returns [`ccmr2_input::R`](R) reader structure
impl crate::Readable for CCMR2_INPUTrs {}
///`write(|w| ..)` method takes [`ccmr2_input::W`](W) writer structure
impl crate::Writable for CCMR2_INPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCMR2_Input to value 0
impl crate::Resettable for CCMR2_INPUTrs {
    const RESET_VALUE: u32 = 0;
}
