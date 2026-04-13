///Register `CCMR3_Input` reader
pub type R = crate::R<CCMR3_INPUTrs>;
///Register `CCMR3_Input` writer
pub type W = crate::W<CCMR3_INPUTrs>;
/**Input capture %s prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICPRESCALER {
    ///0: No prescaler, capture is done each time an edge is detected on the capture input
    NoPrescaler = 0,
    ///1: Capture is done once every 2 events
    TwoEvents = 1,
    ///2: Capture is done once every 4 events
    FourEvents = 2,
    ///3: Capture is done once every 8 events
    EightEvents = 3,
}
impl From<ICPRESCALER> for u8 {
    #[inline(always)]
    fn from(variant: ICPRESCALER) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ICPRESCALER {
    type Ux = u8;
}
impl crate::IsEnum for ICPRESCALER {}
///Field `ICPSC(5-6)` reader - Input capture %s prescaler
pub type ICPSC_R = crate::FieldReader<ICPRESCALER>;
impl ICPSC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ICPRESCALER {
        match self.bits {
            0 => ICPRESCALER::NoPrescaler,
            1 => ICPRESCALER::TwoEvents,
            2 => ICPRESCALER::FourEvents,
            3 => ICPRESCALER::EightEvents,
            _ => unreachable!(),
        }
    }
    ///No prescaler, capture is done each time an edge is detected on the capture input
    #[inline(always)]
    pub fn is_no_prescaler(&self) -> bool {
        *self == ICPRESCALER::NoPrescaler
    }
    ///Capture is done once every 2 events
    #[inline(always)]
    pub fn is_two_events(&self) -> bool {
        *self == ICPRESCALER::TwoEvents
    }
    ///Capture is done once every 4 events
    #[inline(always)]
    pub fn is_four_events(&self) -> bool {
        *self == ICPRESCALER::FourEvents
    }
    ///Capture is done once every 8 events
    #[inline(always)]
    pub fn is_eight_events(&self) -> bool {
        *self == ICPRESCALER::EightEvents
    }
}
///Field `ICPSC(5-6)` writer - Input capture %s prescaler
pub type ICPSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ICPRESCALER, crate::Safe>;
impl<'a, REG> ICPSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No prescaler, capture is done each time an edge is detected on the capture input
    #[inline(always)]
    pub fn no_prescaler(self) -> &'a mut crate::W<REG> {
        self.variant(ICPRESCALER::NoPrescaler)
    }
    ///Capture is done once every 2 events
    #[inline(always)]
    pub fn two_events(self) -> &'a mut crate::W<REG> {
        self.variant(ICPRESCALER::TwoEvents)
    }
    ///Capture is done once every 4 events
    #[inline(always)]
    pub fn four_events(self) -> &'a mut crate::W<REG> {
        self.variant(ICPRESCALER::FourEvents)
    }
    ///Capture is done once every 8 events
    #[inline(always)]
    pub fn eight_events(self) -> &'a mut crate::W<REG> {
        self.variant(ICPRESCALER::EightEvents)
    }
}
/**Input capture %s filter

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICFILTER {
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
impl From<ICFILTER> for u8 {
    #[inline(always)]
    fn from(variant: ICFILTER) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ICFILTER {
    type Ux = u8;
}
impl crate::IsEnum for ICFILTER {}
///Field `ICF(5-6)` reader - Input capture %s filter
pub type ICF_R = crate::FieldReader<ICFILTER>;
impl ICF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ICFILTER {
        match self.bits {
            0 => ICFILTER::NoFilter,
            1 => ICFILTER::FckIntN2,
            2 => ICFILTER::FckIntN4,
            3 => ICFILTER::FckIntN8,
            4 => ICFILTER::FdtsDiv2N6,
            5 => ICFILTER::FdtsDiv2N8,
            6 => ICFILTER::FdtsDiv4N6,
            7 => ICFILTER::FdtsDiv4N8,
            8 => ICFILTER::FdtsDiv8N6,
            9 => ICFILTER::FdtsDiv8N8,
            10 => ICFILTER::FdtsDiv16N5,
            11 => ICFILTER::FdtsDiv16N6,
            12 => ICFILTER::FdtsDiv16N8,
            13 => ICFILTER::FdtsDiv32N5,
            14 => ICFILTER::FdtsDiv32N6,
            15 => ICFILTER::FdtsDiv32N8,
            _ => unreachable!(),
        }
    }
    ///No filter, sampling is done at fDTS
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == ICFILTER::NoFilter
    }
    ///fSAMPLING=fCK_INT, N=2
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        *self == ICFILTER::FckIntN2
    }
    ///fSAMPLING=fCK_INT, N=4
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        *self == ICFILTER::FckIntN4
    }
    ///fSAMPLING=fCK_INT, N=8
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        *self == ICFILTER::FckIntN8
    }
    ///fSAMPLING=fDTS/2, N=6
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == ICFILTER::FdtsDiv2N6
    }
    ///fSAMPLING=fDTS/2, N=8
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == ICFILTER::FdtsDiv2N8
    }
    ///fSAMPLING=fDTS/4, N=6
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == ICFILTER::FdtsDiv4N6
    }
    ///fSAMPLING=fDTS/4, N=8
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == ICFILTER::FdtsDiv4N8
    }
    ///fSAMPLING=fDTS/8, N=6
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == ICFILTER::FdtsDiv8N6
    }
    ///fSAMPLING=fDTS/8, N=8
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == ICFILTER::FdtsDiv8N8
    }
    ///fSAMPLING=fDTS/16, N=5
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == ICFILTER::FdtsDiv16N5
    }
    ///fSAMPLING=fDTS/16, N=6
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == ICFILTER::FdtsDiv16N6
    }
    ///fSAMPLING=fDTS/16, N=8
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == ICFILTER::FdtsDiv16N8
    }
    ///fSAMPLING=fDTS/32, N=5
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == ICFILTER::FdtsDiv32N5
    }
    ///fSAMPLING=fDTS/32, N=6
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == ICFILTER::FdtsDiv32N6
    }
    ///fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == ICFILTER::FdtsDiv32N8
    }
}
///Field `ICF(5-6)` writer - Input capture %s filter
pub type ICF_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ICFILTER, crate::Safe>;
impl<'a, REG> ICF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No filter, sampling is done at fDTS
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut crate::W<REG> {
        self.variant(ICFILTER::NoFilter)
    }
    ///fSAMPLING=fCK_INT, N=2
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut crate::W<REG> {
        self.variant(ICFILTER::FckIntN2)
    }
    ///fSAMPLING=fCK_INT, N=4
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut crate::W<REG> {
        self.variant(ICFILTER::FckIntN4)
    }
    ///fSAMPLING=fCK_INT, N=8
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ICFILTER::FckIntN8)
    }
    ///fSAMPLING=fDTS/2, N=6
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut crate::W<REG> {
        self.variant(ICFILTER::FdtsDiv2N6)
    }
    ///fSAMPLING=fDTS/2, N=8
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ICFILTER::FdtsDiv2N8)
    }
    ///fSAMPLING=fDTS/4, N=6
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut crate::W<REG> {
        self.variant(ICFILTER::FdtsDiv4N6)
    }
    ///fSAMPLING=fDTS/4, N=8
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ICFILTER::FdtsDiv4N8)
    }
    ///fSAMPLING=fDTS/8, N=6
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut crate::W<REG> {
        self.variant(ICFILTER::FdtsDiv8N6)
    }
    ///fSAMPLING=fDTS/8, N=8
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ICFILTER::FdtsDiv8N8)
    }
    ///fSAMPLING=fDTS/16, N=5
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut crate::W<REG> {
        self.variant(ICFILTER::FdtsDiv16N5)
    }
    ///fSAMPLING=fDTS/16, N=6
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut crate::W<REG> {
        self.variant(ICFILTER::FdtsDiv16N6)
    }
    ///fSAMPLING=fDTS/16, N=8
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ICFILTER::FdtsDiv16N8)
    }
    ///fSAMPLING=fDTS/32, N=5
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut crate::W<REG> {
        self.variant(ICFILTER::FdtsDiv32N5)
    }
    ///fSAMPLING=fDTS/32, N=6
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut crate::W<REG> {
        self.variant(ICFILTER::FdtsDiv32N6)
    }
    ///fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut crate::W<REG> {
        self.variant(ICFILTER::FdtsDiv32N8)
    }
}
impl R {
    ///Input capture (5-6) prescaler
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `IC5PSC` field.</div>
    #[inline(always)]
    pub fn icpsc(&self, n: u8) -> ICPSC_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ICPSC_R::new(((self.bits >> (n * 8 + 2)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Input capture (5-6) prescaler
    #[inline(always)]
    pub fn icpsc_iter(&self) -> impl Iterator<Item = ICPSC_R> + '_ {
        (0..2).map(move |n| ICPSC_R::new(((self.bits >> (n * 8 + 2)) & 3) as u8))
    }
    ///Bits 2:3 - Input capture 5 prescaler
    #[inline(always)]
    pub fn ic5psc(&self) -> ICPSC_R {
        ICPSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 10:11 - Input capture 6 prescaler
    #[inline(always)]
    pub fn ic6psc(&self) -> ICPSC_R {
        ICPSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Input capture (5-6) filter
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `IC5F` field.</div>
    #[inline(always)]
    pub fn icf(&self, n: u8) -> ICF_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ICF_R::new(((self.bits >> (n * 8 + 4)) & 0x0f) as u8)
    }
    ///Iterator for array of:
    ///Input capture (5-6) filter
    #[inline(always)]
    pub fn icf_iter(&self) -> impl Iterator<Item = ICF_R> + '_ {
        (0..2).map(move |n| ICF_R::new(((self.bits >> (n * 8 + 4)) & 0x0f) as u8))
    }
    ///Bits 4:7 - Input capture 5 filter
    #[inline(always)]
    pub fn ic5f(&self) -> ICF_R {
        ICF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 12:15 - Input capture 6 filter
    #[inline(always)]
    pub fn ic6f(&self) -> ICF_R {
        ICF_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR3_Input")
            .field("ic5psc", &self.ic5psc())
            .field("ic6psc", &self.ic6psc())
            .field("ic5f", &self.ic5f())
            .field("ic6f", &self.ic6f())
            .finish()
    }
}
impl W {
    ///Input capture (5-6) prescaler
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `IC5PSC` field.</div>
    #[inline(always)]
    pub fn icpsc(&mut self, n: u8) -> ICPSC_W<'_, CCMR3_INPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ICPSC_W::new(self, n * 8 + 2)
    }
    ///Bits 2:3 - Input capture 5 prescaler
    #[inline(always)]
    pub fn ic5psc(&mut self) -> ICPSC_W<'_, CCMR3_INPUTrs> {
        ICPSC_W::new(self, 2)
    }
    ///Bits 10:11 - Input capture 6 prescaler
    #[inline(always)]
    pub fn ic6psc(&mut self) -> ICPSC_W<'_, CCMR3_INPUTrs> {
        ICPSC_W::new(self, 10)
    }
    ///Input capture (5-6) filter
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `IC5F` field.</div>
    #[inline(always)]
    pub fn icf(&mut self, n: u8) -> ICF_W<'_, CCMR3_INPUTrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ICF_W::new(self, n * 8 + 4)
    }
    ///Bits 4:7 - Input capture 5 filter
    #[inline(always)]
    pub fn ic5f(&mut self) -> ICF_W<'_, CCMR3_INPUTrs> {
        ICF_W::new(self, 4)
    }
    ///Bits 12:15 - Input capture 6 filter
    #[inline(always)]
    pub fn ic6f(&mut self) -> ICF_W<'_, CCMR3_INPUTrs> {
        ICF_W::new(self, 12)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`ccmr3_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr3_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#TIM1:CCMR3_Input)*/
pub struct CCMR3_INPUTrs;
impl crate::RegisterSpec for CCMR3_INPUTrs {
    type Ux = u32;
}
///`read()` method returns [`ccmr3_input::R`](R) reader structure
impl crate::Readable for CCMR3_INPUTrs {}
///`write(|w| ..)` method takes [`ccmr3_input::W`](W) writer structure
impl crate::Writable for CCMR3_INPUTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCMR3_Input to value 0
impl crate::Resettable for CCMR3_INPUTrs {}
