#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "PREDIV division factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PREDIV {
    #[doc = "0: PREDIV input clock not divided"]
    Div1 = 0,
    #[doc = "1: PREDIV input clock divided by 2"]
    Div2 = 1,
    #[doc = "2: PREDIV input clock divided by 3"]
    Div3 = 2,
    #[doc = "3: PREDIV input clock divided by 4"]
    Div4 = 3,
    #[doc = "4: PREDIV input clock divided by 5"]
    Div5 = 4,
    #[doc = "5: PREDIV input clock divided by 6"]
    Div6 = 5,
    #[doc = "6: PREDIV input clock divided by 7"]
    Div7 = 6,
    #[doc = "7: PREDIV input clock divided by 8"]
    Div8 = 7,
    #[doc = "8: PREDIV input clock divided by 9"]
    Div9 = 8,
    #[doc = "9: PREDIV input clock divided by 10"]
    Div10 = 9,
    #[doc = "10: PREDIV input clock divided by 11"]
    Div11 = 10,
    #[doc = "11: PREDIV input clock divided by 12"]
    Div12 = 11,
    #[doc = "12: PREDIV input clock divided by 13"]
    Div13 = 12,
    #[doc = "13: PREDIV input clock divided by 14"]
    Div14 = 13,
    #[doc = "14: PREDIV input clock divided by 15"]
    Div15 = 14,
    #[doc = "15: PREDIV input clock divided by 16"]
    Div16 = 15,
}
impl From<PREDIV> for u8 {
    #[inline(always)]
    fn from(variant: PREDIV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PREDIV {
    type Ux = u8;
}
#[doc = "Field `PREDIV` reader - PREDIV division factor"]
pub type PREDIV_R = crate::FieldReader<PREDIV>;
impl PREDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PREDIV {
        match self.bits {
            0 => PREDIV::Div1,
            1 => PREDIV::Div2,
            2 => PREDIV::Div3,
            3 => PREDIV::Div4,
            4 => PREDIV::Div5,
            5 => PREDIV::Div6,
            6 => PREDIV::Div7,
            7 => PREDIV::Div8,
            8 => PREDIV::Div9,
            9 => PREDIV::Div10,
            10 => PREDIV::Div11,
            11 => PREDIV::Div12,
            12 => PREDIV::Div13,
            13 => PREDIV::Div14,
            14 => PREDIV::Div15,
            15 => PREDIV::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "PREDIV input clock not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PREDIV::Div1
    }
    #[doc = "PREDIV input clock divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PREDIV::Div2
    }
    #[doc = "PREDIV input clock divided by 3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PREDIV::Div3
    }
    #[doc = "PREDIV input clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PREDIV::Div4
    }
    #[doc = "PREDIV input clock divided by 5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PREDIV::Div5
    }
    #[doc = "PREDIV input clock divided by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PREDIV::Div6
    }
    #[doc = "PREDIV input clock divided by 7"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PREDIV::Div7
    }
    #[doc = "PREDIV input clock divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PREDIV::Div8
    }
    #[doc = "PREDIV input clock divided by 9"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PREDIV::Div9
    }
    #[doc = "PREDIV input clock divided by 10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PREDIV::Div10
    }
    #[doc = "PREDIV input clock divided by 11"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PREDIV::Div11
    }
    #[doc = "PREDIV input clock divided by 12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PREDIV::Div12
    }
    #[doc = "PREDIV input clock divided by 13"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PREDIV::Div13
    }
    #[doc = "PREDIV input clock divided by 14"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PREDIV::Div14
    }
    #[doc = "PREDIV input clock divided by 15"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PREDIV::Div15
    }
    #[doc = "PREDIV input clock divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PREDIV::Div16
    }
}
#[doc = "Field `PREDIV` writer - PREDIV division factor"]
pub type PREDIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, PREDIV>;
impl<'a, REG> PREDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PREDIV input clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div1)
    }
    #[doc = "PREDIV input clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div2)
    }
    #[doc = "PREDIV input clock divided by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div3)
    }
    #[doc = "PREDIV input clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div4)
    }
    #[doc = "PREDIV input clock divided by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div5)
    }
    #[doc = "PREDIV input clock divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div6)
    }
    #[doc = "PREDIV input clock divided by 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div7)
    }
    #[doc = "PREDIV input clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div8)
    }
    #[doc = "PREDIV input clock divided by 9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div9)
    }
    #[doc = "PREDIV input clock divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div10)
    }
    #[doc = "PREDIV input clock divided by 11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div11)
    }
    #[doc = "PREDIV input clock divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div12)
    }
    #[doc = "PREDIV input clock divided by 13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div13)
    }
    #[doc = "PREDIV input clock divided by 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div14)
    }
    #[doc = "PREDIV input clock divided by 15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div15)
    }
    #[doc = "PREDIV input clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div16)
    }
}
#[doc = "ADC1 and ADC2 prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC12PRES {
    #[doc = "0: No clock"]
    NoClock = 0,
    #[doc = "16: PLL clock not divided"]
    Div1 = 16,
    #[doc = "17: PLL clock divided by 2"]
    Div2 = 17,
    #[doc = "18: PLL clock divided by 4"]
    Div4 = 18,
    #[doc = "19: PLL clock divided by 6"]
    Div6 = 19,
    #[doc = "20: PLL clock divided by 8"]
    Div8 = 20,
    #[doc = "21: PLL clock divided by 10"]
    Div10 = 21,
    #[doc = "22: PLL clock divided by 12"]
    Div12 = 22,
    #[doc = "23: PLL clock divided by 16"]
    Div16 = 23,
    #[doc = "24: PLL clock divided by 32"]
    Div32 = 24,
    #[doc = "25: PLL clock divided by 64"]
    Div64 = 25,
    #[doc = "26: PLL clock divided by 128"]
    Div128 = 26,
    #[doc = "27: PLL clock divided by 256"]
    Div256 = 27,
}
impl From<ADC12PRES> for u8 {
    #[inline(always)]
    fn from(variant: ADC12PRES) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC12PRES {
    type Ux = u8;
}
#[doc = "Field `ADC12PRES` reader - ADC1 and ADC2 prescaler"]
pub type ADC12PRES_R = crate::FieldReader<ADC12PRES>;
impl ADC12PRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADC12PRES> {
        match self.bits {
            0 => Some(ADC12PRES::NoClock),
            16 => Some(ADC12PRES::Div1),
            17 => Some(ADC12PRES::Div2),
            18 => Some(ADC12PRES::Div4),
            19 => Some(ADC12PRES::Div6),
            20 => Some(ADC12PRES::Div8),
            21 => Some(ADC12PRES::Div10),
            22 => Some(ADC12PRES::Div12),
            23 => Some(ADC12PRES::Div16),
            24 => Some(ADC12PRES::Div32),
            25 => Some(ADC12PRES::Div64),
            26 => Some(ADC12PRES::Div128),
            27 => Some(ADC12PRES::Div256),
            _ => None,
        }
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == ADC12PRES::NoClock
    }
    #[doc = "PLL clock not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == ADC12PRES::Div1
    }
    #[doc = "PLL clock divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ADC12PRES::Div2
    }
    #[doc = "PLL clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ADC12PRES::Div4
    }
    #[doc = "PLL clock divided by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == ADC12PRES::Div6
    }
    #[doc = "PLL clock divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ADC12PRES::Div8
    }
    #[doc = "PLL clock divided by 10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == ADC12PRES::Div10
    }
    #[doc = "PLL clock divided by 12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == ADC12PRES::Div12
    }
    #[doc = "PLL clock divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == ADC12PRES::Div16
    }
    #[doc = "PLL clock divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == ADC12PRES::Div32
    }
    #[doc = "PLL clock divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == ADC12PRES::Div64
    }
    #[doc = "PLL clock divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == ADC12PRES::Div128
    }
    #[doc = "PLL clock divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == ADC12PRES::Div256
    }
}
#[doc = "Field `ADC12PRES` writer - ADC1 and ADC2 prescaler"]
pub type ADC12PRES_W<'a, REG> = crate::FieldWriter<'a, REG, 5, ADC12PRES>;
impl<'a, REG> ADC12PRES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12PRES::NoClock)
    }
    #[doc = "PLL clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12PRES::Div1)
    }
    #[doc = "PLL clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12PRES::Div2)
    }
    #[doc = "PLL clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12PRES::Div4)
    }
    #[doc = "PLL clock divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12PRES::Div6)
    }
    #[doc = "PLL clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12PRES::Div8)
    }
    #[doc = "PLL clock divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12PRES::Div10)
    }
    #[doc = "PLL clock divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12PRES::Div12)
    }
    #[doc = "PLL clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12PRES::Div16)
    }
    #[doc = "PLL clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12PRES::Div32)
    }
    #[doc = "PLL clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12PRES::Div64)
    }
    #[doc = "PLL clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12PRES::Div128)
    }
    #[doc = "PLL clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(ADC12PRES::Div256)
    }
}
impl R {
    #[doc = "Bits 0:3 - PREDIV division factor"]
    #[inline(always)]
    pub fn prediv(&self) -> PREDIV_R {
        PREDIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - ADC1 and ADC2 prescaler"]
    #[inline(always)]
    pub fn adc12pres(&self) -> ADC12PRES_R {
        ADC12PRES_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PREDIV division factor"]
    #[inline(always)]
    #[must_use]
    pub fn prediv(&mut self) -> PREDIV_W<CFGR2rs> {
        PREDIV_W::new(self, 0)
    }
    #[doc = "Bits 4:8 - ADC1 and ADC2 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn adc12pres(&mut self) -> ADC12PRES_W<CFGR2rs> {
        ADC12PRES_W::new(self, 4)
    }
}
#[doc = "Clock configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
