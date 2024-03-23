#[doc = "Register `CCMR2_Input` reader"]
pub type R = crate::R<CCMR2_INPUTrs>;
#[doc = "Register `CCMR2_Input` writer"]
pub type W = crate::W<CCMR2_INPUTrs>;
#[doc = "Capture/Compare %s selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC3S {
    #[doc = "0: CCx channel is configured as output"]
    Output = 0,
    #[doc = "1: CCx channel is configured as input, ICx is mapped on TI1"]
    Ti1 = 1,
    #[doc = "2: CCx channel is configured as input, ICx is mapped on TI2"]
    Ti2 = 2,
    #[doc = "3: CCx channel is configured as input, ICx is mapped on TRC"]
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
#[doc = "Field `CCS(3-4)` reader - Capture/Compare %s selection"]
pub type CCS_R = crate::FieldReader<CC3S>;
impl CCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC3S {
        match self.bits {
            0 => CC3S::Output,
            1 => CC3S::Ti1,
            2 => CC3S::Ti2,
            3 => CC3S::Trc,
            _ => unreachable!(),
        }
    }
    #[doc = "CCx channel is configured as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == CC3S::Output
    }
    #[doc = "CCx channel is configured as input, ICx is mapped on TI1"]
    #[inline(always)]
    pub fn is_ti1(&self) -> bool {
        *self == CC3S::Ti1
    }
    #[doc = "CCx channel is configured as input, ICx is mapped on TI2"]
    #[inline(always)]
    pub fn is_ti2(&self) -> bool {
        *self == CC3S::Ti2
    }
    #[doc = "CCx channel is configured as input, ICx is mapped on TRC"]
    #[inline(always)]
    pub fn is_trc(&self) -> bool {
        *self == CC3S::Trc
    }
}
#[doc = "Field `CCS(3-4)` writer - Capture/Compare %s selection"]
pub type CCS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CC3S>;
impl<'a, REG> CCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCx channel is configured as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S::Output)
    }
    #[doc = "CCx channel is configured as input, ICx is mapped on TI1"]
    #[inline(always)]
    pub fn ti1(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S::Ti1)
    }
    #[doc = "CCx channel is configured as input, ICx is mapped on TI2"]
    #[inline(always)]
    pub fn ti2(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S::Ti2)
    }
    #[doc = "CCx channel is configured as input, ICx is mapped on TRC"]
    #[inline(always)]
    pub fn trc(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S::Trc)
    }
}
#[doc = "Input capture %s prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IC3PSC {
    #[doc = "0: CCx channel is configured as output"]
    Output = 0,
    #[doc = "1: Capture is done once every 2 events"]
    Capture2 = 1,
    #[doc = "2: Capture is done once every 4 events"]
    Capture4 = 2,
    #[doc = "3: Capture is done once every 8 events"]
    Capture8 = 3,
}
impl From<IC3PSC> for u8 {
    #[inline(always)]
    fn from(variant: IC3PSC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IC3PSC {
    type Ux = u8;
}
#[doc = "Field `ICPSC(3-4)` reader - Input capture %s prescaler"]
pub type ICPSC_R = crate::FieldReader<IC3PSC>;
impl ICPSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IC3PSC {
        match self.bits {
            0 => IC3PSC::Output,
            1 => IC3PSC::Capture2,
            2 => IC3PSC::Capture4,
            3 => IC3PSC::Capture8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCx channel is configured as output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == IC3PSC::Output
    }
    #[doc = "Capture is done once every 2 events"]
    #[inline(always)]
    pub fn is_capture2(&self) -> bool {
        *self == IC3PSC::Capture2
    }
    #[doc = "Capture is done once every 4 events"]
    #[inline(always)]
    pub fn is_capture4(&self) -> bool {
        *self == IC3PSC::Capture4
    }
    #[doc = "Capture is done once every 8 events"]
    #[inline(always)]
    pub fn is_capture8(&self) -> bool {
        *self == IC3PSC::Capture8
    }
}
#[doc = "Field `ICPSC(3-4)` writer - Input capture %s prescaler"]
pub type ICPSC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, IC3PSC>;
impl<'a, REG> ICPSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCx channel is configured as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(IC3PSC::Output)
    }
    #[doc = "Capture is done once every 2 events"]
    #[inline(always)]
    pub fn capture2(self) -> &'a mut crate::W<REG> {
        self.variant(IC3PSC::Capture2)
    }
    #[doc = "Capture is done once every 4 events"]
    #[inline(always)]
    pub fn capture4(self) -> &'a mut crate::W<REG> {
        self.variant(IC3PSC::Capture4)
    }
    #[doc = "Capture is done once every 8 events"]
    #[inline(always)]
    pub fn capture8(self) -> &'a mut crate::W<REG> {
        self.variant(IC3PSC::Capture8)
    }
}
#[doc = "Input capture %s filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IC3F {
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
impl From<IC3F> for u8 {
    #[inline(always)]
    fn from(variant: IC3F) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IC3F {
    type Ux = u8;
}
#[doc = "Field `ICF(3-4)` reader - Input capture %s filter"]
pub type ICF_R = crate::FieldReader<IC3F>;
impl ICF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IC3F {
        match self.bits {
            0 => IC3F::NoFilter,
            1 => IC3F::FckIntN2,
            2 => IC3F::FckIntN4,
            3 => IC3F::FckIntN8,
            4 => IC3F::FdtsDiv2N6,
            5 => IC3F::FdtsDiv2N8,
            6 => IC3F::FdtsDiv4N6,
            7 => IC3F::FdtsDiv4N8,
            8 => IC3F::FdtsDiv8N6,
            9 => IC3F::FdtsDiv8N8,
            10 => IC3F::FdtsDiv16N5,
            11 => IC3F::FdtsDiv16N6,
            12 => IC3F::FdtsDiv16N8,
            13 => IC3F::FdtsDiv32N5,
            14 => IC3F::FdtsDiv32N6,
            15 => IC3F::FdtsDiv32N8,
            _ => unreachable!(),
        }
    }
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == IC3F::NoFilter
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        *self == IC3F::FckIntN2
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        *self == IC3F::FckIntN4
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        *self == IC3F::FckIntN8
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == IC3F::FdtsDiv2N6
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == IC3F::FdtsDiv2N8
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == IC3F::FdtsDiv4N6
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == IC3F::FdtsDiv4N8
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == IC3F::FdtsDiv8N6
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == IC3F::FdtsDiv8N8
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == IC3F::FdtsDiv16N5
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == IC3F::FdtsDiv16N6
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == IC3F::FdtsDiv16N8
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == IC3F::FdtsDiv32N5
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == IC3F::FdtsDiv32N6
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == IC3F::FdtsDiv32N8
    }
}
#[doc = "Field `ICF(3-4)` writer - Input capture %s filter"]
pub type ICF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, IC3F>;
impl<'a, REG> ICF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut crate::W<REG> {
        self.variant(IC3F::NoFilter)
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut crate::W<REG> {
        self.variant(IC3F::FckIntN2)
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut crate::W<REG> {
        self.variant(IC3F::FckIntN4)
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC3F::FckIntN8)
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut crate::W<REG> {
        self.variant(IC3F::FdtsDiv2N6)
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC3F::FdtsDiv2N8)
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut crate::W<REG> {
        self.variant(IC3F::FdtsDiv4N6)
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC3F::FdtsDiv4N8)
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut crate::W<REG> {
        self.variant(IC3F::FdtsDiv8N6)
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC3F::FdtsDiv8N8)
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut crate::W<REG> {
        self.variant(IC3F::FdtsDiv16N5)
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut crate::W<REG> {
        self.variant(IC3F::FdtsDiv16N6)
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC3F::FdtsDiv16N8)
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut crate::W<REG> {
        self.variant(IC3F::FdtsDiv32N5)
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut crate::W<REG> {
        self.variant(IC3F::FdtsDiv32N6)
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC3F::FdtsDiv32N8)
    }
}
impl R {
    #[doc = "Capture/Compare (3-4) selection"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC3S` field"]
    #[inline(always)]
    pub fn ccs(&self, n: u8) -> CCS_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CCS_R::new(((self.bits >> (n * 8)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Capture/Compare (3-4) selection"]
    #[inline(always)]
    pub fn ccs_iter(&self) -> impl Iterator<Item = CCS_R> + '_ {
        (0..2).map(move |n| CCS_R::new(((self.bits >> (n * 8)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - Capture/Compare 3 selection"]
    #[inline(always)]
    pub fn cc3s(&self) -> CCS_R {
        CCS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline(always)]
    pub fn cc4s(&self) -> CCS_R {
        CCS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Input capture (3-4) prescaler"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `IC3PSC` field"]
    #[inline(always)]
    pub fn icpsc(&self, n: u8) -> ICPSC_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ICPSC_R::new(((self.bits >> (n * 8 + 2)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Input capture (3-4) prescaler"]
    #[inline(always)]
    pub fn icpsc_iter(&self) -> impl Iterator<Item = ICPSC_R> + '_ {
        (0..2).map(move |n| ICPSC_R::new(((self.bits >> (n * 8 + 2)) & 3) as u8))
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler"]
    #[inline(always)]
    pub fn ic3psc(&self) -> ICPSC_R {
        ICPSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler"]
    #[inline(always)]
    pub fn ic4psc(&self) -> ICPSC_R {
        ICPSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Input capture (3-4) filter"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `IC3F` field"]
    #[inline(always)]
    pub fn icf(&self, n: u8) -> ICF_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ICF_R::new(((self.bits >> (n * 8 + 4)) & 0x0f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Input capture (3-4) filter"]
    #[inline(always)]
    pub fn icf_iter(&self) -> impl Iterator<Item = ICF_R> + '_ {
        (0..2).map(move |n| ICF_R::new(((self.bits >> (n * 8 + 4)) & 0x0f) as u8))
    }
    #[doc = "Bits 4:7 - Input capture 3 filter"]
    #[inline(always)]
    pub fn ic3f(&self) -> ICF_R {
        ICF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Input capture 4 filter"]
    #[inline(always)]
    pub fn ic4f(&self) -> ICF_R {
        ICF_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Capture/Compare (3-4) selection"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC3S` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccs(&mut self, n: u8) -> CCS_W<CCMR2_INPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CCS_W::new(self, n * 8)
    }
    #[doc = "Bits 0:1 - Capture/Compare 3 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc3s(&mut self) -> CCS_W<CCMR2_INPUTrs> {
        CCS_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc4s(&mut self) -> CCS_W<CCMR2_INPUTrs> {
        CCS_W::new(self, 8)
    }
    #[doc = "Input capture (3-4) prescaler"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `IC3PSC` field"]
    #[inline(always)]
    #[must_use]
    pub fn icpsc(&mut self, n: u8) -> ICPSC_W<CCMR2_INPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ICPSC_W::new(self, n * 8 + 2)
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ic3psc(&mut self) -> ICPSC_W<CCMR2_INPUTrs> {
        ICPSC_W::new(self, 2)
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ic4psc(&mut self) -> ICPSC_W<CCMR2_INPUTrs> {
        ICPSC_W::new(self, 10)
    }
    #[doc = "Input capture (3-4) filter"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `IC3F` field"]
    #[inline(always)]
    #[must_use]
    pub fn icf(&mut self, n: u8) -> ICF_W<CCMR2_INPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ICF_W::new(self, n * 8 + 4)
    }
    #[doc = "Bits 4:7 - Input capture 3 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ic3f(&mut self) -> ICF_W<CCMR2_INPUTrs> {
        ICF_W::new(self, 4)
    }
    #[doc = "Bits 12:15 - Input capture 4 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ic4f(&mut self) -> ICF_W<CCMR2_INPUTrs> {
        ICF_W::new(self, 12)
    }
}
#[doc = "capture/compare mode register 2 (input mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr2_input::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr2_input::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR2_INPUTrs;
impl crate::RegisterSpec for CCMR2_INPUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr2_input::R`](R) reader structure"]
impl crate::Readable for CCMR2_INPUTrs {}
#[doc = "`write(|w| ..)` method takes [`ccmr2_input::W`](W) writer structure"]
impl crate::Writable for CCMR2_INPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMR2_Input to value 0"]
impl crate::Resettable for CCMR2_INPUTrs {
    const RESET_VALUE: u32 = 0;
}
