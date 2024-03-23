#[doc = "Register `ACR` reader"]
pub type R = crate::R<ACRrs>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<ACRrs>;
#[doc = "Latency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LATENCY {
    #[doc = "0: 0 wait states"]
    Ws0 = 0,
    #[doc = "1: 1 wait states"]
    Ws1 = 1,
    #[doc = "2: 2 wait states"]
    Ws2 = 2,
    #[doc = "3: 3 wait states"]
    Ws3 = 3,
    #[doc = "4: 4 wait states"]
    Ws4 = 4,
    #[doc = "5: 5 wait states"]
    Ws5 = 5,
    #[doc = "6: 6 wait states"]
    Ws6 = 6,
    #[doc = "7: 7 wait states"]
    Ws7 = 7,
    #[doc = "8: 8 wait states"]
    Ws8 = 8,
    #[doc = "9: 9 wait states"]
    Ws9 = 9,
    #[doc = "10: 10 wait states"]
    Ws10 = 10,
    #[doc = "11: 11 wait states"]
    Ws11 = 11,
    #[doc = "12: 12 wait states"]
    Ws12 = 12,
    #[doc = "13: 13 wait states"]
    Ws13 = 13,
    #[doc = "14: 14 wait states"]
    Ws14 = 14,
    #[doc = "15: 15 wait states"]
    Ws15 = 15,
}
impl From<LATENCY> for u8 {
    #[inline(always)]
    fn from(variant: LATENCY) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LATENCY {
    type Ux = u8;
}
#[doc = "Field `LATENCY` reader - Latency"]
pub type LATENCY_R = crate::FieldReader<LATENCY>;
impl LATENCY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LATENCY {
        match self.bits {
            0 => LATENCY::Ws0,
            1 => LATENCY::Ws1,
            2 => LATENCY::Ws2,
            3 => LATENCY::Ws3,
            4 => LATENCY::Ws4,
            5 => LATENCY::Ws5,
            6 => LATENCY::Ws6,
            7 => LATENCY::Ws7,
            8 => LATENCY::Ws8,
            9 => LATENCY::Ws9,
            10 => LATENCY::Ws10,
            11 => LATENCY::Ws11,
            12 => LATENCY::Ws12,
            13 => LATENCY::Ws13,
            14 => LATENCY::Ws14,
            15 => LATENCY::Ws15,
            _ => unreachable!(),
        }
    }
    #[doc = "0 wait states"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == LATENCY::Ws0
    }
    #[doc = "1 wait states"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == LATENCY::Ws1
    }
    #[doc = "2 wait states"]
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == LATENCY::Ws2
    }
    #[doc = "3 wait states"]
    #[inline(always)]
    pub fn is_ws3(&self) -> bool {
        *self == LATENCY::Ws3
    }
    #[doc = "4 wait states"]
    #[inline(always)]
    pub fn is_ws4(&self) -> bool {
        *self == LATENCY::Ws4
    }
    #[doc = "5 wait states"]
    #[inline(always)]
    pub fn is_ws5(&self) -> bool {
        *self == LATENCY::Ws5
    }
    #[doc = "6 wait states"]
    #[inline(always)]
    pub fn is_ws6(&self) -> bool {
        *self == LATENCY::Ws6
    }
    #[doc = "7 wait states"]
    #[inline(always)]
    pub fn is_ws7(&self) -> bool {
        *self == LATENCY::Ws7
    }
    #[doc = "8 wait states"]
    #[inline(always)]
    pub fn is_ws8(&self) -> bool {
        *self == LATENCY::Ws8
    }
    #[doc = "9 wait states"]
    #[inline(always)]
    pub fn is_ws9(&self) -> bool {
        *self == LATENCY::Ws9
    }
    #[doc = "10 wait states"]
    #[inline(always)]
    pub fn is_ws10(&self) -> bool {
        *self == LATENCY::Ws10
    }
    #[doc = "11 wait states"]
    #[inline(always)]
    pub fn is_ws11(&self) -> bool {
        *self == LATENCY::Ws11
    }
    #[doc = "12 wait states"]
    #[inline(always)]
    pub fn is_ws12(&self) -> bool {
        *self == LATENCY::Ws12
    }
    #[doc = "13 wait states"]
    #[inline(always)]
    pub fn is_ws13(&self) -> bool {
        *self == LATENCY::Ws13
    }
    #[doc = "14 wait states"]
    #[inline(always)]
    pub fn is_ws14(&self) -> bool {
        *self == LATENCY::Ws14
    }
    #[doc = "15 wait states"]
    #[inline(always)]
    pub fn is_ws15(&self) -> bool {
        *self == LATENCY::Ws15
    }
}
#[doc = "Field `LATENCY` writer - Latency"]
pub type LATENCY_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, LATENCY>;
impl<'a, REG> LATENCY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 wait states"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws0)
    }
    #[doc = "1 wait states"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws1)
    }
    #[doc = "2 wait states"]
    #[inline(always)]
    pub fn ws2(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws2)
    }
    #[doc = "3 wait states"]
    #[inline(always)]
    pub fn ws3(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws3)
    }
    #[doc = "4 wait states"]
    #[inline(always)]
    pub fn ws4(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws4)
    }
    #[doc = "5 wait states"]
    #[inline(always)]
    pub fn ws5(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws5)
    }
    #[doc = "6 wait states"]
    #[inline(always)]
    pub fn ws6(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws6)
    }
    #[doc = "7 wait states"]
    #[inline(always)]
    pub fn ws7(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws7)
    }
    #[doc = "8 wait states"]
    #[inline(always)]
    pub fn ws8(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws8)
    }
    #[doc = "9 wait states"]
    #[inline(always)]
    pub fn ws9(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws9)
    }
    #[doc = "10 wait states"]
    #[inline(always)]
    pub fn ws10(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws10)
    }
    #[doc = "11 wait states"]
    #[inline(always)]
    pub fn ws11(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws11)
    }
    #[doc = "12 wait states"]
    #[inline(always)]
    pub fn ws12(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws12)
    }
    #[doc = "13 wait states"]
    #[inline(always)]
    pub fn ws13(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws13)
    }
    #[doc = "14 wait states"]
    #[inline(always)]
    pub fn ws14(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws14)
    }
    #[doc = "15 wait states"]
    #[inline(always)]
    pub fn ws15(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws15)
    }
}
#[doc = "Prefetch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRFTEN {
    #[doc = "0: Prefetch is disabled"]
    Disabled = 0,
    #[doc = "1: Prefetch is enabled"]
    Enabled = 1,
}
impl From<PRFTEN> for bool {
    #[inline(always)]
    fn from(variant: PRFTEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRFTEN` reader - Prefetch enable"]
pub type PRFTEN_R = crate::BitReader<PRFTEN>;
impl PRFTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRFTEN {
        match self.bits {
            false => PRFTEN::Disabled,
            true => PRFTEN::Enabled,
        }
    }
    #[doc = "Prefetch is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTEN::Disabled
    }
    #[doc = "Prefetch is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTEN::Enabled
    }
}
#[doc = "Field `PRFTEN` writer - Prefetch enable"]
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG, PRFTEN>;
impl<'a, REG> PRFTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Prefetch is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN::Disabled)
    }
    #[doc = "Prefetch is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN::Enabled)
    }
}
#[doc = "ART Accelerator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARTEN {
    #[doc = "0: ART Accelerator is disabled"]
    Disabled = 0,
    #[doc = "1: ART Accelerator is enabled"]
    Enabled = 1,
}
impl From<ARTEN> for bool {
    #[inline(always)]
    fn from(variant: ARTEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARTEN` reader - ART Accelerator Enable"]
pub type ARTEN_R = crate::BitReader<ARTEN>;
impl ARTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ARTEN {
        match self.bits {
            false => ARTEN::Disabled,
            true => ARTEN::Enabled,
        }
    }
    #[doc = "ART Accelerator is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARTEN::Disabled
    }
    #[doc = "ART Accelerator is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARTEN::Enabled
    }
}
#[doc = "Field `ARTEN` writer - ART Accelerator Enable"]
pub type ARTEN_W<'a, REG> = crate::BitWriter<'a, REG, ARTEN>;
impl<'a, REG> ARTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ART Accelerator is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ARTEN::Disabled)
    }
    #[doc = "ART Accelerator is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ARTEN::Enabled)
    }
}
#[doc = "ART Accelerator reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARTRST {
    #[doc = "0: Accelerator is not reset"]
    NotReset = 0,
    #[doc = "1: Accelerator is reset"]
    Reset = 1,
}
impl From<ARTRST> for bool {
    #[inline(always)]
    fn from(variant: ARTRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARTRST` reader - ART Accelerator reset"]
pub type ARTRST_R = crate::BitReader<ARTRST>;
impl ARTRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ARTRST {
        match self.bits {
            false => ARTRST::NotReset,
            true => ARTRST::Reset,
        }
    }
    #[doc = "Accelerator is not reset"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == ARTRST::NotReset
    }
    #[doc = "Accelerator is reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ARTRST::Reset
    }
}
#[doc = "Field `ARTRST` writer - ART Accelerator reset"]
pub type ARTRST_W<'a, REG> = crate::BitWriter<'a, REG, ARTRST>;
impl<'a, REG> ARTRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Accelerator is not reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut crate::W<REG> {
        self.variant(ARTRST::NotReset)
    }
    #[doc = "Accelerator is reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(ARTRST::Reset)
    }
}
impl R {
    #[doc = "Bits 0:3 - Latency"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ART Accelerator Enable"]
    #[inline(always)]
    pub fn arten(&self) -> ARTEN_R {
        ARTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - ART Accelerator reset"]
    #[inline(always)]
    pub fn artrst(&self) -> ARTRST_R {
        ARTRST_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Latency"]
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LATENCY_W<ACRrs> {
        LATENCY_W::new(self, 0)
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn prften(&mut self) -> PRFTEN_W<ACRrs> {
        PRFTEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - ART Accelerator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arten(&mut self) -> ARTEN_W<ACRrs> {
        ARTEN_W::new(self, 9)
    }
    #[doc = "Bit 11 - ART Accelerator reset"]
    #[inline(always)]
    #[must_use]
    pub fn artrst(&mut self) -> ARTRST_W<ACRrs> {
        ARTRST_W::new(self, 11)
    }
}
#[doc = "Flash access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACRrs;
impl crate::RegisterSpec for ACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for ACRrs {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for ACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for ACRrs {
    const RESET_VALUE: u32 = 0;
}
