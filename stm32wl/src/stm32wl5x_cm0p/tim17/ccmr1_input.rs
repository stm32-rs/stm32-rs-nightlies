///Register `CCMR1_Input` reader
pub type R = crate::R<CCMR1_INPUTrs>;
///Register `CCMR1_Input` writer
pub type W = crate::W<CCMR1_INPUTrs>;
/**Capture/Compare %s selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC1S {
    ///0: CCx channel is configured as output
    Output = 0,
    ///1: CCx channel is configured as input, ICx is mapped on TI1
    Ti1 = 1,
    ///2: CCx channel is configured as input, ICx is mapped on TI2
    Ti2 = 2,
    ///3: CCx channel is configured as input, ICx is mapped on TRC
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
impl crate::IsEnum for CC1S {}
///Field `CCS(1-1)` reader - Capture/Compare %s selection
pub type CCS_R = crate::FieldReader<CC1S>;
impl CCS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1S {
        match self.bits {
            0 => CC1S::Output,
            1 => CC1S::Ti1,
            2 => CC1S::Ti2,
            3 => CC1S::Trc,
            _ => unreachable!(),
        }
    }
    ///CCx channel is configured as output
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == CC1S::Output
    }
    ///CCx channel is configured as input, ICx is mapped on TI1
    #[inline(always)]
    pub fn is_ti1(&self) -> bool {
        *self == CC1S::Ti1
    }
    ///CCx channel is configured as input, ICx is mapped on TI2
    #[inline(always)]
    pub fn is_ti2(&self) -> bool {
        *self == CC1S::Ti2
    }
    ///CCx channel is configured as input, ICx is mapped on TRC
    #[inline(always)]
    pub fn is_trc(&self) -> bool {
        *self == CC1S::Trc
    }
}
///Field `CCS(1-1)` writer - Capture/Compare %s selection
pub type CCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC1S, crate::Safe>;
impl<'a, REG> CCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CCx channel is configured as output
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(CC1S::Output)
    }
    ///CCx channel is configured as input, ICx is mapped on TI1
    #[inline(always)]
    pub fn ti1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1S::Ti1)
    }
    ///CCx channel is configured as input, ICx is mapped on TI2
    #[inline(always)]
    pub fn ti2(self) -> &'a mut crate::W<REG> {
        self.variant(CC1S::Ti2)
    }
    ///CCx channel is configured as input, ICx is mapped on TRC
    #[inline(always)]
    pub fn trc(self) -> &'a mut crate::W<REG> {
        self.variant(CC1S::Trc)
    }
}
/**Input capture %s prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IC1PSC {
    ///0: CCx channel is configured as output
    Output = 0,
    ///1: Capture is done once every 2 events
    Capture2 = 1,
    ///2: Capture is done once every 4 events
    Capture4 = 2,
    ///3: Capture is done once every 8 events
    Capture8 = 3,
}
impl From<IC1PSC> for u8 {
    #[inline(always)]
    fn from(variant: IC1PSC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IC1PSC {
    type Ux = u8;
}
impl crate::IsEnum for IC1PSC {}
///Field `ICPSC(1-1)` reader - Input capture %s prescaler
pub type ICPSC_R = crate::FieldReader<IC1PSC>;
impl ICPSC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IC1PSC {
        match self.bits {
            0 => IC1PSC::Output,
            1 => IC1PSC::Capture2,
            2 => IC1PSC::Capture4,
            3 => IC1PSC::Capture8,
            _ => unreachable!(),
        }
    }
    ///CCx channel is configured as output
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == IC1PSC::Output
    }
    ///Capture is done once every 2 events
    #[inline(always)]
    pub fn is_capture_2(&self) -> bool {
        *self == IC1PSC::Capture2
    }
    ///Capture is done once every 4 events
    #[inline(always)]
    pub fn is_capture_4(&self) -> bool {
        *self == IC1PSC::Capture4
    }
    ///Capture is done once every 8 events
    #[inline(always)]
    pub fn is_capture_8(&self) -> bool {
        *self == IC1PSC::Capture8
    }
}
///Field `ICPSC(1-1)` writer - Input capture %s prescaler
pub type ICPSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IC1PSC, crate::Safe>;
impl<'a, REG> ICPSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CCx channel is configured as output
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(IC1PSC::Output)
    }
    ///Capture is done once every 2 events
    #[inline(always)]
    pub fn capture_2(self) -> &'a mut crate::W<REG> {
        self.variant(IC1PSC::Capture2)
    }
    ///Capture is done once every 4 events
    #[inline(always)]
    pub fn capture_4(self) -> &'a mut crate::W<REG> {
        self.variant(IC1PSC::Capture4)
    }
    ///Capture is done once every 8 events
    #[inline(always)]
    pub fn capture_8(self) -> &'a mut crate::W<REG> {
        self.variant(IC1PSC::Capture8)
    }
}
/**Input capture %s filter

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IC1F {
    ///0: No filter, sampling is done at fDTS
    NoFilter = 0,
    ///1: fSAMPLING=fCK_INT, N=2
    FckIntN2 = 1,
    ///2: fSAMPLING=fCK_INT, N=4
    FckIntN4 = 2,
    ///3: fSAMPLING=fCK_INT, N=8
    FckIntN8 = 3,
    ///4: fSAMPLING=fDTS/2, N=6
    FdtsDiv2N6 = 4,
    ///5: fSAMPLING=fDTS/2, N=8
    FdtsDiv2N8 = 5,
    ///6: fSAMPLING=fDTS/4, N=6
    FdtsDiv4N6 = 6,
    ///7: fSAMPLING=fDTS/4, N=8
    FdtsDiv4N8 = 7,
    ///8: fSAMPLING=fDTS/8, N=6
    FdtsDiv8N6 = 8,
    ///9: fSAMPLING=fDTS/8, N=8
    FdtsDiv8N8 = 9,
    ///10: fSAMPLING=fDTS/16, N=5
    FdtsDiv16N5 = 10,
    ///11: fSAMPLING=fDTS/16, N=6
    FdtsDiv16N6 = 11,
    ///12: fSAMPLING=fDTS/16, N=8
    FdtsDiv16N8 = 12,
    ///13: fSAMPLING=fDTS/32, N=5
    FdtsDiv32N5 = 13,
    ///14: fSAMPLING=fDTS/32, N=6
    FdtsDiv32N6 = 14,
    ///15: fSAMPLING=fDTS/32, N=8
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
impl crate::IsEnum for IC1F {}
///Field `ICF(1-1)` reader - Input capture %s filter
pub type ICF_R = crate::FieldReader<IC1F>;
impl ICF_R {
    ///Get enumerated values variant
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
    ///No filter, sampling is done at fDTS
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == IC1F::NoFilter
    }
    ///fSAMPLING=fCK_INT, N=2
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        *self == IC1F::FckIntN2
    }
    ///fSAMPLING=fCK_INT, N=4
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        *self == IC1F::FckIntN4
    }
    ///fSAMPLING=fCK_INT, N=8
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        *self == IC1F::FckIntN8
    }
    ///fSAMPLING=fDTS/2, N=6
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == IC1F::FdtsDiv2N6
    }
    ///fSAMPLING=fDTS/2, N=8
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == IC1F::FdtsDiv2N8
    }
    ///fSAMPLING=fDTS/4, N=6
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == IC1F::FdtsDiv4N6
    }
    ///fSAMPLING=fDTS/4, N=8
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == IC1F::FdtsDiv4N8
    }
    ///fSAMPLING=fDTS/8, N=6
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == IC1F::FdtsDiv8N6
    }
    ///fSAMPLING=fDTS/8, N=8
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == IC1F::FdtsDiv8N8
    }
    ///fSAMPLING=fDTS/16, N=5
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == IC1F::FdtsDiv16N5
    }
    ///fSAMPLING=fDTS/16, N=6
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == IC1F::FdtsDiv16N6
    }
    ///fSAMPLING=fDTS/16, N=8
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == IC1F::FdtsDiv16N8
    }
    ///fSAMPLING=fDTS/32, N=5
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == IC1F::FdtsDiv32N5
    }
    ///fSAMPLING=fDTS/32, N=6
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == IC1F::FdtsDiv32N6
    }
    ///fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == IC1F::FdtsDiv32N8
    }
}
///Field `ICF(1-1)` writer - Input capture %s filter
pub type ICF_W<'a, REG> = crate::FieldWriter<'a, REG, 4, IC1F, crate::Safe>;
impl<'a, REG> ICF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No filter, sampling is done at fDTS
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::NoFilter)
    }
    ///fSAMPLING=fCK_INT, N=2
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FckIntN2)
    }
    ///fSAMPLING=fCK_INT, N=4
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FckIntN4)
    }
    ///fSAMPLING=fCK_INT, N=8
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FckIntN8)
    }
    ///fSAMPLING=fDTS/2, N=6
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv2N6)
    }
    ///fSAMPLING=fDTS/2, N=8
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv2N8)
    }
    ///fSAMPLING=fDTS/4, N=6
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv4N6)
    }
    ///fSAMPLING=fDTS/4, N=8
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv4N8)
    }
    ///fSAMPLING=fDTS/8, N=6
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv8N6)
    }
    ///fSAMPLING=fDTS/8, N=8
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv8N8)
    }
    ///fSAMPLING=fDTS/16, N=5
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv16N5)
    }
    ///fSAMPLING=fDTS/16, N=6
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv16N6)
    }
    ///fSAMPLING=fDTS/16, N=8
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv16N8)
    }
    ///fSAMPLING=fDTS/32, N=5
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv32N5)
    }
    ///fSAMPLING=fDTS/32, N=6
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv32N6)
    }
    ///fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F::FdtsDiv32N8)
    }
}
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
    pub fn ccs(&mut self, n: u8) -> CCS_W<CCMR1_INPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CCS_W::new(self, n * 0)
    }
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    pub fn cc1s(&mut self) -> CCS_W<CCMR1_INPUTrs> {
        CCS_W::new(self, 0)
    }
    ///Input capture (1-1) prescaler
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `IC1PSC` field.</div>
    #[inline(always)]
    pub fn icpsc(&mut self, n: u8) -> ICPSC_W<CCMR1_INPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        ICPSC_W::new(self, n * 0 + 2)
    }
    ///Bits 2:3 - Input capture 1 prescaler
    #[inline(always)]
    pub fn ic1psc(&mut self) -> ICPSC_W<CCMR1_INPUTrs> {
        ICPSC_W::new(self, 2)
    }
    ///Input capture (1-1) filter
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `IC1F` field.</div>
    #[inline(always)]
    pub fn icf(&mut self, n: u8) -> ICF_W<CCMR1_INPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        ICF_W::new(self, n * 0 + 4)
    }
    ///Bits 4:7 - Input capture 1 filter
    #[inline(always)]
    pub fn ic1f(&mut self) -> ICF_W<CCMR1_INPUTrs> {
        ICF_W::new(self, 4)
    }
}
/**TIM16/TIM17 capture/compare mode register 1

You can [`read`](crate::Reg::read) this register and get [`ccmr1_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#TIM17:CCMR1_Input)*/
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
