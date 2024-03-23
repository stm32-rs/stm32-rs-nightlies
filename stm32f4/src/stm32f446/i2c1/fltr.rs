#[doc = "Register `FLTR` reader"]
pub type R = crate::R<FLTRrs>;
#[doc = "Register `FLTR` writer"]
pub type W = crate::W<FLTRrs>;
#[doc = "Digital noise filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DNF {
    #[doc = "0: Digital filter disabled"]
    NoFilter = 0,
    #[doc = "1: Digital filter enabled and filtering capability up to 1 tI2CCLK"]
    Filter1 = 1,
    #[doc = "2: Digital filter enabled and filtering capability up to 2 tI2CCLK"]
    Filter2 = 2,
    #[doc = "3: Digital filter enabled and filtering capability up to 3 tI2CCLK"]
    Filter3 = 3,
    #[doc = "4: Digital filter enabled and filtering capability up to 4 tI2CCLK"]
    Filter4 = 4,
    #[doc = "5: Digital filter enabled and filtering capability up to 5 tI2CCLK"]
    Filter5 = 5,
    #[doc = "6: Digital filter enabled and filtering capability up to 6 tI2CCLK"]
    Filter6 = 6,
    #[doc = "7: Digital filter enabled and filtering capability up to 7 tI2CCLK"]
    Filter7 = 7,
    #[doc = "8: Digital filter enabled and filtering capability up to 8 tI2CCLK"]
    Filter8 = 8,
    #[doc = "9: Digital filter enabled and filtering capability up to 9 tI2CCLK"]
    Filter9 = 9,
    #[doc = "10: Digital filter enabled and filtering capability up to 10 tI2CCLK"]
    Filter10 = 10,
    #[doc = "11: Digital filter enabled and filtering capability up to 11 tI2CCLK"]
    Filter11 = 11,
    #[doc = "12: Digital filter enabled and filtering capability up to 12 tI2CCLK"]
    Filter12 = 12,
    #[doc = "13: Digital filter enabled and filtering capability up to 13 tI2CCLK"]
    Filter13 = 13,
    #[doc = "14: Digital filter enabled and filtering capability up to 14 tI2CCLK"]
    Filter14 = 14,
    #[doc = "15: Digital filter enabled and filtering capability up to 15 tI2CCLK"]
    Filter15 = 15,
}
impl From<DNF> for u8 {
    #[inline(always)]
    fn from(variant: DNF) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DNF {
    type Ux = u8;
}
#[doc = "Field `DNF` reader - Digital noise filter"]
pub type DNF_R = crate::FieldReader<DNF>;
impl DNF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DNF {
        match self.bits {
            0 => DNF::NoFilter,
            1 => DNF::Filter1,
            2 => DNF::Filter2,
            3 => DNF::Filter3,
            4 => DNF::Filter4,
            5 => DNF::Filter5,
            6 => DNF::Filter6,
            7 => DNF::Filter7,
            8 => DNF::Filter8,
            9 => DNF::Filter9,
            10 => DNF::Filter10,
            11 => DNF::Filter11,
            12 => DNF::Filter12,
            13 => DNF::Filter13,
            14 => DNF::Filter14,
            15 => DNF::Filter15,
            _ => unreachable!(),
        }
    }
    #[doc = "Digital filter disabled"]
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == DNF::NoFilter
    }
    #[doc = "Digital filter enabled and filtering capability up to 1 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter1(&self) -> bool {
        *self == DNF::Filter1
    }
    #[doc = "Digital filter enabled and filtering capability up to 2 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter2(&self) -> bool {
        *self == DNF::Filter2
    }
    #[doc = "Digital filter enabled and filtering capability up to 3 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter3(&self) -> bool {
        *self == DNF::Filter3
    }
    #[doc = "Digital filter enabled and filtering capability up to 4 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter4(&self) -> bool {
        *self == DNF::Filter4
    }
    #[doc = "Digital filter enabled and filtering capability up to 5 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter5(&self) -> bool {
        *self == DNF::Filter5
    }
    #[doc = "Digital filter enabled and filtering capability up to 6 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter6(&self) -> bool {
        *self == DNF::Filter6
    }
    #[doc = "Digital filter enabled and filtering capability up to 7 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter7(&self) -> bool {
        *self == DNF::Filter7
    }
    #[doc = "Digital filter enabled and filtering capability up to 8 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter8(&self) -> bool {
        *self == DNF::Filter8
    }
    #[doc = "Digital filter enabled and filtering capability up to 9 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter9(&self) -> bool {
        *self == DNF::Filter9
    }
    #[doc = "Digital filter enabled and filtering capability up to 10 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter10(&self) -> bool {
        *self == DNF::Filter10
    }
    #[doc = "Digital filter enabled and filtering capability up to 11 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter11(&self) -> bool {
        *self == DNF::Filter11
    }
    #[doc = "Digital filter enabled and filtering capability up to 12 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter12(&self) -> bool {
        *self == DNF::Filter12
    }
    #[doc = "Digital filter enabled and filtering capability up to 13 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter13(&self) -> bool {
        *self == DNF::Filter13
    }
    #[doc = "Digital filter enabled and filtering capability up to 14 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter14(&self) -> bool {
        *self == DNF::Filter14
    }
    #[doc = "Digital filter enabled and filtering capability up to 15 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter15(&self) -> bool {
        *self == DNF::Filter15
    }
}
#[doc = "Field `DNF` writer - Digital noise filter"]
pub type DNF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, DNF>;
impl<'a, REG> DNF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Digital filter disabled"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::NoFilter)
    }
    #[doc = "Digital filter enabled and filtering capability up to 1 tI2CCLK"]
    #[inline(always)]
    pub fn filter1(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter1)
    }
    #[doc = "Digital filter enabled and filtering capability up to 2 tI2CCLK"]
    #[inline(always)]
    pub fn filter2(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter2)
    }
    #[doc = "Digital filter enabled and filtering capability up to 3 tI2CCLK"]
    #[inline(always)]
    pub fn filter3(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter3)
    }
    #[doc = "Digital filter enabled and filtering capability up to 4 tI2CCLK"]
    #[inline(always)]
    pub fn filter4(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter4)
    }
    #[doc = "Digital filter enabled and filtering capability up to 5 tI2CCLK"]
    #[inline(always)]
    pub fn filter5(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter5)
    }
    #[doc = "Digital filter enabled and filtering capability up to 6 tI2CCLK"]
    #[inline(always)]
    pub fn filter6(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter6)
    }
    #[doc = "Digital filter enabled and filtering capability up to 7 tI2CCLK"]
    #[inline(always)]
    pub fn filter7(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter7)
    }
    #[doc = "Digital filter enabled and filtering capability up to 8 tI2CCLK"]
    #[inline(always)]
    pub fn filter8(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter8)
    }
    #[doc = "Digital filter enabled and filtering capability up to 9 tI2CCLK"]
    #[inline(always)]
    pub fn filter9(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter9)
    }
    #[doc = "Digital filter enabled and filtering capability up to 10 tI2CCLK"]
    #[inline(always)]
    pub fn filter10(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter10)
    }
    #[doc = "Digital filter enabled and filtering capability up to 11 tI2CCLK"]
    #[inline(always)]
    pub fn filter11(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter11)
    }
    #[doc = "Digital filter enabled and filtering capability up to 12 tI2CCLK"]
    #[inline(always)]
    pub fn filter12(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter12)
    }
    #[doc = "Digital filter enabled and filtering capability up to 13 tI2CCLK"]
    #[inline(always)]
    pub fn filter13(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter13)
    }
    #[doc = "Digital filter enabled and filtering capability up to 14 tI2CCLK"]
    #[inline(always)]
    pub fn filter14(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter14)
    }
    #[doc = "Digital filter enabled and filtering capability up to 15 tI2CCLK"]
    #[inline(always)]
    pub fn filter15(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter15)
    }
}
#[doc = "Analog noise filter OFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANOFF {
    #[doc = "0: Analog noise filter enabled"]
    Enabled = 0,
    #[doc = "1: Analog noise filter disabled"]
    Disabled = 1,
}
impl From<ANOFF> for bool {
    #[inline(always)]
    fn from(variant: ANOFF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANOFF` reader - Analog noise filter OFF"]
pub type ANOFF_R = crate::BitReader<ANOFF>;
impl ANOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ANOFF {
        match self.bits {
            false => ANOFF::Enabled,
            true => ANOFF::Disabled,
        }
    }
    #[doc = "Analog noise filter enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ANOFF::Enabled
    }
    #[doc = "Analog noise filter disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ANOFF::Disabled
    }
}
#[doc = "Field `ANOFF` writer - Analog noise filter OFF"]
pub type ANOFF_W<'a, REG> = crate::BitWriter<'a, REG, ANOFF>;
impl<'a, REG> ANOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog noise filter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ANOFF::Enabled)
    }
    #[doc = "Analog noise filter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ANOFF::Disabled)
    }
}
impl R {
    #[doc = "Bits 0:3 - Digital noise filter"]
    #[inline(always)]
    pub fn dnf(&self) -> DNF_R {
        DNF_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Analog noise filter OFF"]
    #[inline(always)]
    pub fn anoff(&self) -> ANOFF_R {
        ANOFF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Digital noise filter"]
    #[inline(always)]
    #[must_use]
    pub fn dnf(&mut self) -> DNF_W<FLTRrs> {
        DNF_W::new(self, 0)
    }
    #[doc = "Bit 4 - Analog noise filter OFF"]
    #[inline(always)]
    #[must_use]
    pub fn anoff(&mut self) -> ANOFF_W<FLTRrs> {
        ANOFF_W::new(self, 4)
    }
}
#[doc = "I2C FLTR register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLTRrs;
impl crate::RegisterSpec for FLTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltr::R`](R) reader structure"]
impl crate::Readable for FLTRrs {}
#[doc = "`write(|w| ..)` method takes [`fltr::W`](W) writer structure"]
impl crate::Writable for FLTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLTR to value 0"]
impl crate::Resettable for FLTRrs {
    const RESET_VALUE: u32 = 0;
}
