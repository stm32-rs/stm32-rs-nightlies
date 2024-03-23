#[doc = "Register `CCMR1_Input` reader"]
pub type R = crate::R<CCMR1_INPUTrs>;
#[doc = "Register `CCMR1_Input` writer"]
pub type W = crate::W<CCMR1_INPUTrs>;
#[doc = "Capture/Compare %s selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC1S {
    #[doc = "1: CC1 channel is configured as input, IC1 is mapped on TI1"]
    Ti1 = 1,
    #[doc = "2: CC1 channel is configured as input, IC1 is mapped on TI2"]
    Ti2 = 2,
    #[doc = "3: CC1 channel is configured as input, IC1 is mapped on TRC"]
    Trc = 3,
}
impl From<CC1S> for u8 {
    #[inline(always)]
    fn from(variant: CC1S) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC1S {
    type Ux = u8;
}
#[doc = "Field `CCS(1-2)` reader - Capture/Compare %s selection"]
pub type CCS_R = crate::FieldReader<CC1S>;
impl CCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CC1S> {
        match self.bits {
            1 => Some(CC1S::Ti1),
            2 => Some(CC1S::Ti2),
            3 => Some(CC1S::Trc),
            _ => None,
        }
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI1"]
    #[inline(always)]
    pub fn is_ti1(&self) -> bool {
        *self == CC1S::Ti1
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI2"]
    #[inline(always)]
    pub fn is_ti2(&self) -> bool {
        *self == CC1S::Ti2
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TRC"]
    #[inline(always)]
    pub fn is_trc(&self) -> bool {
        *self == CC1S::Trc
    }
}
#[doc = "Field `CCS(1-2)` writer - Capture/Compare %s selection"]
pub type CCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC1S>;
impl<'a, REG> CCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI1"]
    #[inline(always)]
    pub fn ti1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1S::Ti1)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI2"]
    #[inline(always)]
    pub fn ti2(self) -> &'a mut crate::W<REG> {
        self.variant(CC1S::Ti2)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TRC"]
    #[inline(always)]
    pub fn trc(self) -> &'a mut crate::W<REG> {
        self.variant(CC1S::Trc)
    }
}
#[doc = "Field `ICPSC(1-2)` reader - Input capture %s prescaler"]
pub type ICPSC_R = crate::FieldReader;
#[doc = "Field `ICPSC(1-2)` writer - Input capture %s prescaler"]
pub type ICPSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Input capture %s filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IC1F {
    #[doc = "0: No filter, sampling is done at fDTS"]
    NoFilter = 0,
    #[doc = "1: fSAMPLING=fCK_INT, N=2"]
    FckIntN2 = 1,
    #[doc = "2: fSAMPLING=fCK_INT, N=4"]
    FckIntN4 = 2,
    #[doc = "3: fSAMPLING=fCK_INT, N=8"]
    FckIntN8 = 3,
    #[doc = "4: fSAMPLING=fDTS/2, N=6"]
    FdtsDiv2N6 = 4,
    #[doc = "5: fSAMPLING=fDTS/2, N=8"]
    FdtsDiv2N8 = 5,
    #[doc = "6: fSAMPLING=fDTS/4, N=6"]
    FdtsDiv4N6 = 6,
    #[doc = "7: fSAMPLING=fDTS/4, N=8"]
    FdtsDiv4N8 = 7,
    #[doc = "8: fSAMPLING=fDTS/8, N=6"]
    FdtsDiv8N6 = 8,
    #[doc = "9: fSAMPLING=fDTS/8, N=8"]
    FdtsDiv8N8 = 9,
    #[doc = "10: fSAMPLING=fDTS/16, N=5"]
    FdtsDiv16N5 = 10,
    #[doc = "11: fSAMPLING=fDTS/16, N=6"]
    FdtsDiv16N6 = 11,
    #[doc = "12: fSAMPLING=fDTS/16, N=8"]
    FdtsDiv16N8 = 12,
    #[doc = "13: fSAMPLING=fDTS/32, N=5"]
    FdtsDiv32N5 = 13,
    #[doc = "14: fSAMPLING=fDTS/32, N=6"]
    FdtsDiv32N6 = 14,
    #[doc = "15: fSAMPLING=fDTS/32, N=8"]
    FdtsDiv32N8 = 15,
}
impl From<IC1F> for u8 {
    #[inline(always)]
    fn from(variant: IC1F) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IC1F {
    type Ux = u8;
}
#[doc = "Field `ICF(1-2)` reader - Input capture %s filter"]
pub type ICF_R = crate::FieldReader<IC1F>;
impl ICF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IC1F {
        match self.bits {
            0 => IC1F::NoFilter,
            1 => IC1F::FckIntN2,
            2 => IC1F::FckIntN4,
            3 => IC1F::FckIntN8,
            4 => IC1F::FdtsDiv2N6,
            5 => IC1F::FdtsDiv2N8,
            6 => IC1F::FdtsDiv4N6,
            7 => IC1F::FdtsDiv4N8,
            8 => IC1F::FdtsDiv8N6,
            9 => IC1F::FdtsDiv8N8,
            10 => IC1F::FdtsDiv16N5,
            11 => IC1F::FdtsDiv16N6,
            12 => IC1F::FdtsDiv16N8,
            13 => IC1F::FdtsDiv32N5,
            14 => IC1F::FdtsDiv32N6,
            15 => IC1F::FdtsDiv32N8,
            _ => unreachable!(),
        }
    }
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == IC1F::NoFilter
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        *self == IC1F::FckIntN2
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        *self == IC1F::FckIntN4
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        *self == IC1F::FckIntN8
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == IC1F::FdtsDiv2N6
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == IC1F::FdtsDiv2N8
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == IC1F::FdtsDiv4N6
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == IC1F::FdtsDiv4N8
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == IC1F::FdtsDiv8N6
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == IC1F::FdtsDiv8N8
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == IC1F::FdtsDiv16N5
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == IC1F::FdtsDiv16N6
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == IC1F::FdtsDiv16N8
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == IC1F::FdtsDiv32N5
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == IC1F::FdtsDiv32N6
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == IC1F::FdtsDiv32N8
    }
}
#[doc = "Field `ICF(1-2)` writer - Input capture %s filter"]
pub type ICF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, IC1F>;
impl<'a, REG> ICF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::NoFilter)
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FckIntN2)
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FckIntN4)
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FckIntN8)
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv2N6)
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv2N8)
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv4N6)
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv4N8)
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv8N6)
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv8N8)
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv16N5)
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv16N6)
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv16N8)
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv32N5)
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv32N6)
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv32N8)
    }
}
impl R {
    #[doc = "Capture/Compare (1-2) selection"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1S` field"]
    #[inline(always)]
    pub fn ccs(&self, n: u8) -> CCS_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CCS_R::new(((self.bits >> (n * 8)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Capture/Compare (1-2) selection"]
    #[inline(always)]
    pub fn ccs_iter(&self) -> impl Iterator<Item = CCS_R> + '_ {
        (0..2).map(move |n| CCS_R::new(((self.bits >> (n * 8)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1s(&self) -> CCS_R {
        CCS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection"]
    #[inline(always)]
    pub fn cc2s(&self) -> CCS_R {
        CCS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Input capture (1-2) prescaler"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `IC1PSC` field"]
    #[inline(always)]
    pub fn icpsc(&self, n: u8) -> ICPSC_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ICPSC_R::new(((self.bits >> (n * 8 + 2)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Input capture (1-2) prescaler"]
    #[inline(always)]
    pub fn icpsc_iter(&self) -> impl Iterator<Item = ICPSC_R> + '_ {
        (0..2).map(move |n| ICPSC_R::new(((self.bits >> (n * 8 + 2)) & 3) as u8))
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline(always)]
    pub fn ic1psc(&self) -> ICPSC_R {
        ICPSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler"]
    #[inline(always)]
    pub fn ic2psc(&self) -> ICPSC_R {
        ICPSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Input capture (1-2) filter"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `IC1F` field"]
    #[inline(always)]
    pub fn icf(&self, n: u8) -> ICF_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ICF_R::new(((self.bits >> (n * 11 + 4)) & 0x0f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Input capture (1-2) filter"]
    #[inline(always)]
    pub fn icf_iter(&self) -> impl Iterator<Item = ICF_R> + '_ {
        (0..2).map(move |n| ICF_R::new(((self.bits >> (n * 11 + 4)) & 0x0f) as u8))
    }
    #[doc = "Bits 4:7 - Input capture 1 filter"]
    #[inline(always)]
    pub fn ic1f(&self) -> ICF_R {
        ICF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 15:18 - Input capture 2 filter"]
    #[inline(always)]
    pub fn ic2f(&self) -> ICF_R {
        ICF_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Capture/Compare (1-2) selection"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1S` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccs(&mut self, n: u8) -> CCS_W<CCMR1_INPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CCS_W::new(self, n * 8)
    }
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CCS_W<CCMR1_INPUTrs> {
        CCS_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc2s(&mut self) -> CCS_W<CCMR1_INPUTrs> {
        CCS_W::new(self, 8)
    }
    #[doc = "Input capture (1-2) prescaler"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `IC1PSC` field"]
    #[inline(always)]
    #[must_use]
    pub fn icpsc(&mut self, n: u8) -> ICPSC_W<CCMR1_INPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ICPSC_W::new(self, n * 8 + 2)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ic1psc(&mut self) -> ICPSC_W<CCMR1_INPUTrs> {
        ICPSC_W::new(self, 2)
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ic2psc(&mut self) -> ICPSC_W<CCMR1_INPUTrs> {
        ICPSC_W::new(self, 10)
    }
    #[doc = "Input capture (1-2) filter"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `IC1F` field"]
    #[inline(always)]
    #[must_use]
    pub fn icf(&mut self, n: u8) -> ICF_W<CCMR1_INPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ICF_W::new(self, n * 11 + 4)
    }
    #[doc = "Bits 4:7 - Input capture 1 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ic1f(&mut self) -> ICF_W<CCMR1_INPUTrs> {
        ICF_W::new(self, 4)
    }
    #[doc = "Bits 15:18 - Input capture 2 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ic2f(&mut self) -> ICF_W<CCMR1_INPUTrs> {
        ICF_W::new(self, 15)
    }
}
#[doc = "capture/compare mode register 1 (input mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr1_input::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr1_input::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
