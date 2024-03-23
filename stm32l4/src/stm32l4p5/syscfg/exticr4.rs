#[doc = "Register `EXTICR4` reader"]
pub type R = crate::R<EXTICR4rs>;
#[doc = "Register `EXTICR4` writer"]
pub type W = crate::W<EXTICR4rs>;
#[doc = "EXTI12 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI12 {
    #[doc = "0: PA12 pin"]
    Pa12 = 0,
    #[doc = "1: PB12 pin"]
    Pb12 = 1,
    #[doc = "2: PC12 pin"]
    Pc12 = 2,
    #[doc = "3: PD12 pin"]
    Pd12 = 3,
    #[doc = "4: PE12 pin"]
    Pe12 = 4,
    #[doc = "5: PF12 pin"]
    Pf12 = 5,
    #[doc = "6: PG12 pin"]
    Pg12 = 6,
    #[doc = "7: PH12 pin"]
    Ph12 = 7,
}
impl From<EXTI12> for u8 {
    #[inline(always)]
    fn from(variant: EXTI12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI12 {
    type Ux = u8;
}
#[doc = "Field `EXTI12` reader - EXTI12 configuration bits"]
pub type EXTI12_R = crate::FieldReader<EXTI12>;
impl EXTI12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI12> {
        match self.bits {
            0 => Some(EXTI12::Pa12),
            1 => Some(EXTI12::Pb12),
            2 => Some(EXTI12::Pc12),
            3 => Some(EXTI12::Pd12),
            4 => Some(EXTI12::Pe12),
            5 => Some(EXTI12::Pf12),
            6 => Some(EXTI12::Pg12),
            7 => Some(EXTI12::Ph12),
            _ => None,
        }
    }
    #[doc = "PA12 pin"]
    #[inline(always)]
    pub fn is_pa12(&self) -> bool {
        *self == EXTI12::Pa12
    }
    #[doc = "PB12 pin"]
    #[inline(always)]
    pub fn is_pb12(&self) -> bool {
        *self == EXTI12::Pb12
    }
    #[doc = "PC12 pin"]
    #[inline(always)]
    pub fn is_pc12(&self) -> bool {
        *self == EXTI12::Pc12
    }
    #[doc = "PD12 pin"]
    #[inline(always)]
    pub fn is_pd12(&self) -> bool {
        *self == EXTI12::Pd12
    }
    #[doc = "PE12 pin"]
    #[inline(always)]
    pub fn is_pe12(&self) -> bool {
        *self == EXTI12::Pe12
    }
    #[doc = "PF12 pin"]
    #[inline(always)]
    pub fn is_pf12(&self) -> bool {
        *self == EXTI12::Pf12
    }
    #[doc = "PG12 pin"]
    #[inline(always)]
    pub fn is_pg12(&self) -> bool {
        *self == EXTI12::Pg12
    }
    #[doc = "PH12 pin"]
    #[inline(always)]
    pub fn is_ph12(&self) -> bool {
        *self == EXTI12::Ph12
    }
}
#[doc = "Field `EXTI12` writer - EXTI12 configuration bits"]
pub type EXTI12_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI12>;
impl<'a, REG> EXTI12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA12 pin"]
    #[inline(always)]
    pub fn pa12(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Pa12)
    }
    #[doc = "PB12 pin"]
    #[inline(always)]
    pub fn pb12(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Pb12)
    }
    #[doc = "PC12 pin"]
    #[inline(always)]
    pub fn pc12(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Pc12)
    }
    #[doc = "PD12 pin"]
    #[inline(always)]
    pub fn pd12(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Pd12)
    }
    #[doc = "PE12 pin"]
    #[inline(always)]
    pub fn pe12(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Pe12)
    }
    #[doc = "PF12 pin"]
    #[inline(always)]
    pub fn pf12(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Pf12)
    }
    #[doc = "PG12 pin"]
    #[inline(always)]
    pub fn pg12(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Pg12)
    }
    #[doc = "PH12 pin"]
    #[inline(always)]
    pub fn ph12(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Ph12)
    }
}
#[doc = "EXTI13 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI13 {
    #[doc = "0: PA13 pin"]
    Pa13 = 0,
    #[doc = "1: PB13 pin"]
    Pb13 = 1,
    #[doc = "2: PC13 pin"]
    Pc13 = 2,
    #[doc = "3: PD13 pin"]
    Pd13 = 3,
    #[doc = "4: PE13 pin"]
    Pe13 = 4,
    #[doc = "5: PF13 pin"]
    Pf13 = 5,
    #[doc = "6: PG13 pin"]
    Pg13 = 6,
    #[doc = "7: PH13 pin"]
    Ph13 = 7,
}
impl From<EXTI13> for u8 {
    #[inline(always)]
    fn from(variant: EXTI13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI13 {
    type Ux = u8;
}
#[doc = "Field `EXTI13` reader - EXTI13 configuration bits"]
pub type EXTI13_R = crate::FieldReader<EXTI13>;
impl EXTI13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI13> {
        match self.bits {
            0 => Some(EXTI13::Pa13),
            1 => Some(EXTI13::Pb13),
            2 => Some(EXTI13::Pc13),
            3 => Some(EXTI13::Pd13),
            4 => Some(EXTI13::Pe13),
            5 => Some(EXTI13::Pf13),
            6 => Some(EXTI13::Pg13),
            7 => Some(EXTI13::Ph13),
            _ => None,
        }
    }
    #[doc = "PA13 pin"]
    #[inline(always)]
    pub fn is_pa13(&self) -> bool {
        *self == EXTI13::Pa13
    }
    #[doc = "PB13 pin"]
    #[inline(always)]
    pub fn is_pb13(&self) -> bool {
        *self == EXTI13::Pb13
    }
    #[doc = "PC13 pin"]
    #[inline(always)]
    pub fn is_pc13(&self) -> bool {
        *self == EXTI13::Pc13
    }
    #[doc = "PD13 pin"]
    #[inline(always)]
    pub fn is_pd13(&self) -> bool {
        *self == EXTI13::Pd13
    }
    #[doc = "PE13 pin"]
    #[inline(always)]
    pub fn is_pe13(&self) -> bool {
        *self == EXTI13::Pe13
    }
    #[doc = "PF13 pin"]
    #[inline(always)]
    pub fn is_pf13(&self) -> bool {
        *self == EXTI13::Pf13
    }
    #[doc = "PG13 pin"]
    #[inline(always)]
    pub fn is_pg13(&self) -> bool {
        *self == EXTI13::Pg13
    }
    #[doc = "PH13 pin"]
    #[inline(always)]
    pub fn is_ph13(&self) -> bool {
        *self == EXTI13::Ph13
    }
}
#[doc = "Field `EXTI13` writer - EXTI13 configuration bits"]
pub type EXTI13_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI13>;
impl<'a, REG> EXTI13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA13 pin"]
    #[inline(always)]
    pub fn pa13(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI13::Pa13)
    }
    #[doc = "PB13 pin"]
    #[inline(always)]
    pub fn pb13(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI13::Pb13)
    }
    #[doc = "PC13 pin"]
    #[inline(always)]
    pub fn pc13(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI13::Pc13)
    }
    #[doc = "PD13 pin"]
    #[inline(always)]
    pub fn pd13(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI13::Pd13)
    }
    #[doc = "PE13 pin"]
    #[inline(always)]
    pub fn pe13(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI13::Pe13)
    }
    #[doc = "PF13 pin"]
    #[inline(always)]
    pub fn pf13(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI13::Pf13)
    }
    #[doc = "PG13 pin"]
    #[inline(always)]
    pub fn pg13(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI13::Pg13)
    }
    #[doc = "PH13 pin"]
    #[inline(always)]
    pub fn ph13(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI13::Ph13)
    }
}
#[doc = "EXTI14 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI14 {
    #[doc = "0: PA14 pin"]
    Pa14 = 0,
    #[doc = "1: PB14 pin"]
    Pb14 = 1,
    #[doc = "2: PC14 pin"]
    Pc14 = 2,
    #[doc = "3: PD14 pin"]
    Pd14 = 3,
    #[doc = "4: PE14 pin"]
    Pe14 = 4,
    #[doc = "5: PF14 pin"]
    Pf14 = 5,
    #[doc = "6: PG14 pin"]
    Pg14 = 6,
    #[doc = "7: PH14 pin"]
    Ph14 = 7,
}
impl From<EXTI14> for u8 {
    #[inline(always)]
    fn from(variant: EXTI14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI14 {
    type Ux = u8;
}
#[doc = "Field `EXTI14` reader - EXTI14 configuration bits"]
pub type EXTI14_R = crate::FieldReader<EXTI14>;
impl EXTI14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI14> {
        match self.bits {
            0 => Some(EXTI14::Pa14),
            1 => Some(EXTI14::Pb14),
            2 => Some(EXTI14::Pc14),
            3 => Some(EXTI14::Pd14),
            4 => Some(EXTI14::Pe14),
            5 => Some(EXTI14::Pf14),
            6 => Some(EXTI14::Pg14),
            7 => Some(EXTI14::Ph14),
            _ => None,
        }
    }
    #[doc = "PA14 pin"]
    #[inline(always)]
    pub fn is_pa14(&self) -> bool {
        *self == EXTI14::Pa14
    }
    #[doc = "PB14 pin"]
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        *self == EXTI14::Pb14
    }
    #[doc = "PC14 pin"]
    #[inline(always)]
    pub fn is_pc14(&self) -> bool {
        *self == EXTI14::Pc14
    }
    #[doc = "PD14 pin"]
    #[inline(always)]
    pub fn is_pd14(&self) -> bool {
        *self == EXTI14::Pd14
    }
    #[doc = "PE14 pin"]
    #[inline(always)]
    pub fn is_pe14(&self) -> bool {
        *self == EXTI14::Pe14
    }
    #[doc = "PF14 pin"]
    #[inline(always)]
    pub fn is_pf14(&self) -> bool {
        *self == EXTI14::Pf14
    }
    #[doc = "PG14 pin"]
    #[inline(always)]
    pub fn is_pg14(&self) -> bool {
        *self == EXTI14::Pg14
    }
    #[doc = "PH14 pin"]
    #[inline(always)]
    pub fn is_ph14(&self) -> bool {
        *self == EXTI14::Ph14
    }
}
#[doc = "Field `EXTI14` writer - EXTI14 configuration bits"]
pub type EXTI14_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI14>;
impl<'a, REG> EXTI14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA14 pin"]
    #[inline(always)]
    pub fn pa14(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI14::Pa14)
    }
    #[doc = "PB14 pin"]
    #[inline(always)]
    pub fn pb14(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI14::Pb14)
    }
    #[doc = "PC14 pin"]
    #[inline(always)]
    pub fn pc14(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI14::Pc14)
    }
    #[doc = "PD14 pin"]
    #[inline(always)]
    pub fn pd14(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI14::Pd14)
    }
    #[doc = "PE14 pin"]
    #[inline(always)]
    pub fn pe14(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI14::Pe14)
    }
    #[doc = "PF14 pin"]
    #[inline(always)]
    pub fn pf14(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI14::Pf14)
    }
    #[doc = "PG14 pin"]
    #[inline(always)]
    pub fn pg14(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI14::Pg14)
    }
    #[doc = "PH14 pin"]
    #[inline(always)]
    pub fn ph14(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI14::Ph14)
    }
}
#[doc = "EXTI15 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI15 {
    #[doc = "0: PA15 pin"]
    Pa15 = 0,
    #[doc = "1: PB15 pin"]
    Pb15 = 1,
    #[doc = "2: PC15 pin"]
    Pc15 = 2,
    #[doc = "3: PD15 pin"]
    Pd15 = 3,
    #[doc = "4: PE15 pin"]
    Pe15 = 4,
    #[doc = "5: PF15 pin"]
    Pf15 = 5,
    #[doc = "6: PG15 pin"]
    Pg15 = 6,
    #[doc = "7: PH15 pin"]
    Ph15 = 7,
}
impl From<EXTI15> for u8 {
    #[inline(always)]
    fn from(variant: EXTI15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI15 {
    type Ux = u8;
}
#[doc = "Field `EXTI15` reader - EXTI15 configuration bits"]
pub type EXTI15_R = crate::FieldReader<EXTI15>;
impl EXTI15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI15> {
        match self.bits {
            0 => Some(EXTI15::Pa15),
            1 => Some(EXTI15::Pb15),
            2 => Some(EXTI15::Pc15),
            3 => Some(EXTI15::Pd15),
            4 => Some(EXTI15::Pe15),
            5 => Some(EXTI15::Pf15),
            6 => Some(EXTI15::Pg15),
            7 => Some(EXTI15::Ph15),
            _ => None,
        }
    }
    #[doc = "PA15 pin"]
    #[inline(always)]
    pub fn is_pa15(&self) -> bool {
        *self == EXTI15::Pa15
    }
    #[doc = "PB15 pin"]
    #[inline(always)]
    pub fn is_pb15(&self) -> bool {
        *self == EXTI15::Pb15
    }
    #[doc = "PC15 pin"]
    #[inline(always)]
    pub fn is_pc15(&self) -> bool {
        *self == EXTI15::Pc15
    }
    #[doc = "PD15 pin"]
    #[inline(always)]
    pub fn is_pd15(&self) -> bool {
        *self == EXTI15::Pd15
    }
    #[doc = "PE15 pin"]
    #[inline(always)]
    pub fn is_pe15(&self) -> bool {
        *self == EXTI15::Pe15
    }
    #[doc = "PF15 pin"]
    #[inline(always)]
    pub fn is_pf15(&self) -> bool {
        *self == EXTI15::Pf15
    }
    #[doc = "PG15 pin"]
    #[inline(always)]
    pub fn is_pg15(&self) -> bool {
        *self == EXTI15::Pg15
    }
    #[doc = "PH15 pin"]
    #[inline(always)]
    pub fn is_ph15(&self) -> bool {
        *self == EXTI15::Ph15
    }
}
#[doc = "Field `EXTI15` writer - EXTI15 configuration bits"]
pub type EXTI15_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI15>;
impl<'a, REG> EXTI15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA15 pin"]
    #[inline(always)]
    pub fn pa15(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI15::Pa15)
    }
    #[doc = "PB15 pin"]
    #[inline(always)]
    pub fn pb15(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI15::Pb15)
    }
    #[doc = "PC15 pin"]
    #[inline(always)]
    pub fn pc15(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI15::Pc15)
    }
    #[doc = "PD15 pin"]
    #[inline(always)]
    pub fn pd15(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI15::Pd15)
    }
    #[doc = "PE15 pin"]
    #[inline(always)]
    pub fn pe15(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI15::Pe15)
    }
    #[doc = "PF15 pin"]
    #[inline(always)]
    pub fn pf15(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI15::Pf15)
    }
    #[doc = "PG15 pin"]
    #[inline(always)]
    pub fn pg15(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI15::Pg15)
    }
    #[doc = "PH15 pin"]
    #[inline(always)]
    pub fn ph15(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI15::Ph15)
    }
}
impl R {
    #[doc = "Bits 0:3 - EXTI12 configuration bits"]
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI13 configuration bits"]
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI14 configuration bits"]
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI15 configuration bits"]
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI12 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti12(&mut self) -> EXTI12_W<EXTICR4rs> {
        EXTI12_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTI13 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti13(&mut self) -> EXTI13_W<EXTICR4rs> {
        EXTI13_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTI14 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti14(&mut self) -> EXTI14_W<EXTICR4rs> {
        EXTI14_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTI15 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti15(&mut self) -> EXTI15_W<EXTICR4rs> {
        EXTI15_W::new(self, 12)
    }
}
#[doc = "external interrupt configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTICR4rs;
impl crate::RegisterSpec for EXTICR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exticr4::R`](R) reader structure"]
impl crate::Readable for EXTICR4rs {}
#[doc = "`write(|w| ..)` method takes [`exticr4::W`](W) writer structure"]
impl crate::Writable for EXTICR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTICR4 to value 0"]
impl crate::Resettable for EXTICR4rs {
    const RESET_VALUE: u32 = 0;
}
