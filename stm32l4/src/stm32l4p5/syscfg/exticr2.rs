#[doc = "Register `EXTICR2` reader"]
pub type R = crate::R<EXTICR2rs>;
#[doc = "Register `EXTICR2` writer"]
pub type W = crate::W<EXTICR2rs>;
#[doc = "EXTI 4 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI4 {
    #[doc = "0: PA4 pin"]
    Pa4 = 0,
    #[doc = "1: PB4 pin"]
    Pb4 = 1,
    #[doc = "2: PC4 pin"]
    Pc4 = 2,
    #[doc = "3: PD4 pin"]
    Pd4 = 3,
    #[doc = "4: PE4 pin"]
    Pe4 = 4,
    #[doc = "5: PF4 pin"]
    Pf4 = 5,
    #[doc = "6: PG4 pin"]
    Pg4 = 6,
    #[doc = "7: PH4 pin"]
    Ph4 = 7,
    #[doc = "8: PI4 pin"]
    Pi4 = 8,
}
impl From<EXTI4> for u8 {
    #[inline(always)]
    fn from(variant: EXTI4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI4 {
    type Ux = u8;
}
#[doc = "Field `EXTI4` reader - EXTI 4 configuration bits"]
pub type EXTI4_R = crate::FieldReader<EXTI4>;
impl EXTI4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI4> {
        match self.bits {
            0 => Some(EXTI4::Pa4),
            1 => Some(EXTI4::Pb4),
            2 => Some(EXTI4::Pc4),
            3 => Some(EXTI4::Pd4),
            4 => Some(EXTI4::Pe4),
            5 => Some(EXTI4::Pf4),
            6 => Some(EXTI4::Pg4),
            7 => Some(EXTI4::Ph4),
            8 => Some(EXTI4::Pi4),
            _ => None,
        }
    }
    #[doc = "PA4 pin"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == EXTI4::Pa4
    }
    #[doc = "PB4 pin"]
    #[inline(always)]
    pub fn is_pb4(&self) -> bool {
        *self == EXTI4::Pb4
    }
    #[doc = "PC4 pin"]
    #[inline(always)]
    pub fn is_pc4(&self) -> bool {
        *self == EXTI4::Pc4
    }
    #[doc = "PD4 pin"]
    #[inline(always)]
    pub fn is_pd4(&self) -> bool {
        *self == EXTI4::Pd4
    }
    #[doc = "PE4 pin"]
    #[inline(always)]
    pub fn is_pe4(&self) -> bool {
        *self == EXTI4::Pe4
    }
    #[doc = "PF4 pin"]
    #[inline(always)]
    pub fn is_pf4(&self) -> bool {
        *self == EXTI4::Pf4
    }
    #[doc = "PG4 pin"]
    #[inline(always)]
    pub fn is_pg4(&self) -> bool {
        *self == EXTI4::Pg4
    }
    #[doc = "PH4 pin"]
    #[inline(always)]
    pub fn is_ph4(&self) -> bool {
        *self == EXTI4::Ph4
    }
    #[doc = "PI4 pin"]
    #[inline(always)]
    pub fn is_pi4(&self) -> bool {
        *self == EXTI4::Pi4
    }
}
#[doc = "Field `EXTI4` writer - EXTI 4 configuration bits"]
pub type EXTI4_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI4>;
impl<'a, REG> EXTI4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA4 pin"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Pa4)
    }
    #[doc = "PB4 pin"]
    #[inline(always)]
    pub fn pb4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Pb4)
    }
    #[doc = "PC4 pin"]
    #[inline(always)]
    pub fn pc4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Pc4)
    }
    #[doc = "PD4 pin"]
    #[inline(always)]
    pub fn pd4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Pd4)
    }
    #[doc = "PE4 pin"]
    #[inline(always)]
    pub fn pe4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Pe4)
    }
    #[doc = "PF4 pin"]
    #[inline(always)]
    pub fn pf4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Pf4)
    }
    #[doc = "PG4 pin"]
    #[inline(always)]
    pub fn pg4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Pg4)
    }
    #[doc = "PH4 pin"]
    #[inline(always)]
    pub fn ph4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Ph4)
    }
    #[doc = "PI4 pin"]
    #[inline(always)]
    pub fn pi4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Pi4)
    }
}
#[doc = "EXTI 5 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI5 {
    #[doc = "0: PA5 pin"]
    Pa5 = 0,
    #[doc = "1: PB5 pin"]
    Pb5 = 1,
    #[doc = "2: PC5 pin"]
    Pc5 = 2,
    #[doc = "3: PD5 pin"]
    Pd5 = 3,
    #[doc = "4: PE5 pin"]
    Pe5 = 4,
    #[doc = "5: PF5 pin"]
    Pf5 = 5,
    #[doc = "6: PG5 pin"]
    Pg5 = 6,
    #[doc = "7: PH5 pin"]
    Ph5 = 7,
    #[doc = "8: PI5 pin"]
    Pi5 = 8,
}
impl From<EXTI5> for u8 {
    #[inline(always)]
    fn from(variant: EXTI5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI5 {
    type Ux = u8;
}
#[doc = "Field `EXTI5` reader - EXTI 5 configuration bits"]
pub type EXTI5_R = crate::FieldReader<EXTI5>;
impl EXTI5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI5> {
        match self.bits {
            0 => Some(EXTI5::Pa5),
            1 => Some(EXTI5::Pb5),
            2 => Some(EXTI5::Pc5),
            3 => Some(EXTI5::Pd5),
            4 => Some(EXTI5::Pe5),
            5 => Some(EXTI5::Pf5),
            6 => Some(EXTI5::Pg5),
            7 => Some(EXTI5::Ph5),
            8 => Some(EXTI5::Pi5),
            _ => None,
        }
    }
    #[doc = "PA5 pin"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == EXTI5::Pa5
    }
    #[doc = "PB5 pin"]
    #[inline(always)]
    pub fn is_pb5(&self) -> bool {
        *self == EXTI5::Pb5
    }
    #[doc = "PC5 pin"]
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        *self == EXTI5::Pc5
    }
    #[doc = "PD5 pin"]
    #[inline(always)]
    pub fn is_pd5(&self) -> bool {
        *self == EXTI5::Pd5
    }
    #[doc = "PE5 pin"]
    #[inline(always)]
    pub fn is_pe5(&self) -> bool {
        *self == EXTI5::Pe5
    }
    #[doc = "PF5 pin"]
    #[inline(always)]
    pub fn is_pf5(&self) -> bool {
        *self == EXTI5::Pf5
    }
    #[doc = "PG5 pin"]
    #[inline(always)]
    pub fn is_pg5(&self) -> bool {
        *self == EXTI5::Pg5
    }
    #[doc = "PH5 pin"]
    #[inline(always)]
    pub fn is_ph5(&self) -> bool {
        *self == EXTI5::Ph5
    }
    #[doc = "PI5 pin"]
    #[inline(always)]
    pub fn is_pi5(&self) -> bool {
        *self == EXTI5::Pi5
    }
}
#[doc = "Field `EXTI5` writer - EXTI 5 configuration bits"]
pub type EXTI5_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI5>;
impl<'a, REG> EXTI5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA5 pin"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5::Pa5)
    }
    #[doc = "PB5 pin"]
    #[inline(always)]
    pub fn pb5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5::Pb5)
    }
    #[doc = "PC5 pin"]
    #[inline(always)]
    pub fn pc5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5::Pc5)
    }
    #[doc = "PD5 pin"]
    #[inline(always)]
    pub fn pd5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5::Pd5)
    }
    #[doc = "PE5 pin"]
    #[inline(always)]
    pub fn pe5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5::Pe5)
    }
    #[doc = "PF5 pin"]
    #[inline(always)]
    pub fn pf5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5::Pf5)
    }
    #[doc = "PG5 pin"]
    #[inline(always)]
    pub fn pg5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5::Pg5)
    }
    #[doc = "PH5 pin"]
    #[inline(always)]
    pub fn ph5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5::Ph5)
    }
    #[doc = "PI5 pin"]
    #[inline(always)]
    pub fn pi5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5::Pi5)
    }
}
#[doc = "EXTI 6 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI6 {
    #[doc = "0: PA6 pin"]
    Pa6 = 0,
    #[doc = "1: PB6 pin"]
    Pb6 = 1,
    #[doc = "2: PC6 pin"]
    Pc6 = 2,
    #[doc = "3: PD6 pin"]
    Pd6 = 3,
    #[doc = "4: PE6 pin"]
    Pe6 = 4,
    #[doc = "5: PF6 pin"]
    Pf6 = 5,
    #[doc = "6: PG6 pin"]
    Pg6 = 6,
    #[doc = "7: PH6 pin"]
    Ph6 = 7,
    #[doc = "8: PI6 pin"]
    Pi6 = 8,
}
impl From<EXTI6> for u8 {
    #[inline(always)]
    fn from(variant: EXTI6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI6 {
    type Ux = u8;
}
#[doc = "Field `EXTI6` reader - EXTI 6 configuration bits"]
pub type EXTI6_R = crate::FieldReader<EXTI6>;
impl EXTI6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI6> {
        match self.bits {
            0 => Some(EXTI6::Pa6),
            1 => Some(EXTI6::Pb6),
            2 => Some(EXTI6::Pc6),
            3 => Some(EXTI6::Pd6),
            4 => Some(EXTI6::Pe6),
            5 => Some(EXTI6::Pf6),
            6 => Some(EXTI6::Pg6),
            7 => Some(EXTI6::Ph6),
            8 => Some(EXTI6::Pi6),
            _ => None,
        }
    }
    #[doc = "PA6 pin"]
    #[inline(always)]
    pub fn is_pa6(&self) -> bool {
        *self == EXTI6::Pa6
    }
    #[doc = "PB6 pin"]
    #[inline(always)]
    pub fn is_pb6(&self) -> bool {
        *self == EXTI6::Pb6
    }
    #[doc = "PC6 pin"]
    #[inline(always)]
    pub fn is_pc6(&self) -> bool {
        *self == EXTI6::Pc6
    }
    #[doc = "PD6 pin"]
    #[inline(always)]
    pub fn is_pd6(&self) -> bool {
        *self == EXTI6::Pd6
    }
    #[doc = "PE6 pin"]
    #[inline(always)]
    pub fn is_pe6(&self) -> bool {
        *self == EXTI6::Pe6
    }
    #[doc = "PF6 pin"]
    #[inline(always)]
    pub fn is_pf6(&self) -> bool {
        *self == EXTI6::Pf6
    }
    #[doc = "PG6 pin"]
    #[inline(always)]
    pub fn is_pg6(&self) -> bool {
        *self == EXTI6::Pg6
    }
    #[doc = "PH6 pin"]
    #[inline(always)]
    pub fn is_ph6(&self) -> bool {
        *self == EXTI6::Ph6
    }
    #[doc = "PI6 pin"]
    #[inline(always)]
    pub fn is_pi6(&self) -> bool {
        *self == EXTI6::Pi6
    }
}
#[doc = "Field `EXTI6` writer - EXTI 6 configuration bits"]
pub type EXTI6_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI6>;
impl<'a, REG> EXTI6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA6 pin"]
    #[inline(always)]
    pub fn pa6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6::Pa6)
    }
    #[doc = "PB6 pin"]
    #[inline(always)]
    pub fn pb6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6::Pb6)
    }
    #[doc = "PC6 pin"]
    #[inline(always)]
    pub fn pc6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6::Pc6)
    }
    #[doc = "PD6 pin"]
    #[inline(always)]
    pub fn pd6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6::Pd6)
    }
    #[doc = "PE6 pin"]
    #[inline(always)]
    pub fn pe6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6::Pe6)
    }
    #[doc = "PF6 pin"]
    #[inline(always)]
    pub fn pf6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6::Pf6)
    }
    #[doc = "PG6 pin"]
    #[inline(always)]
    pub fn pg6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6::Pg6)
    }
    #[doc = "PH6 pin"]
    #[inline(always)]
    pub fn ph6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6::Ph6)
    }
    #[doc = "PI6 pin"]
    #[inline(always)]
    pub fn pi6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6::Pi6)
    }
}
#[doc = "EXTI 7 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI7 {
    #[doc = "0: PA7 pin"]
    Pa7 = 0,
    #[doc = "1: PB7 pin"]
    Pb7 = 1,
    #[doc = "2: PC7 pin"]
    Pc7 = 2,
    #[doc = "3: PD7 pin"]
    Pd7 = 3,
    #[doc = "4: PE7 pin"]
    Pe7 = 4,
    #[doc = "5: PF7 pin"]
    Pf7 = 5,
    #[doc = "6: PG7 pin"]
    Pg7 = 6,
    #[doc = "7: PH7 pin"]
    Ph7 = 7,
    #[doc = "8: PI7 pin"]
    Pi7 = 8,
}
impl From<EXTI7> for u8 {
    #[inline(always)]
    fn from(variant: EXTI7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI7 {
    type Ux = u8;
}
#[doc = "Field `EXTI7` reader - EXTI 7 configuration bits"]
pub type EXTI7_R = crate::FieldReader<EXTI7>;
impl EXTI7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI7> {
        match self.bits {
            0 => Some(EXTI7::Pa7),
            1 => Some(EXTI7::Pb7),
            2 => Some(EXTI7::Pc7),
            3 => Some(EXTI7::Pd7),
            4 => Some(EXTI7::Pe7),
            5 => Some(EXTI7::Pf7),
            6 => Some(EXTI7::Pg7),
            7 => Some(EXTI7::Ph7),
            8 => Some(EXTI7::Pi7),
            _ => None,
        }
    }
    #[doc = "PA7 pin"]
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        *self == EXTI7::Pa7
    }
    #[doc = "PB7 pin"]
    #[inline(always)]
    pub fn is_pb7(&self) -> bool {
        *self == EXTI7::Pb7
    }
    #[doc = "PC7 pin"]
    #[inline(always)]
    pub fn is_pc7(&self) -> bool {
        *self == EXTI7::Pc7
    }
    #[doc = "PD7 pin"]
    #[inline(always)]
    pub fn is_pd7(&self) -> bool {
        *self == EXTI7::Pd7
    }
    #[doc = "PE7 pin"]
    #[inline(always)]
    pub fn is_pe7(&self) -> bool {
        *self == EXTI7::Pe7
    }
    #[doc = "PF7 pin"]
    #[inline(always)]
    pub fn is_pf7(&self) -> bool {
        *self == EXTI7::Pf7
    }
    #[doc = "PG7 pin"]
    #[inline(always)]
    pub fn is_pg7(&self) -> bool {
        *self == EXTI7::Pg7
    }
    #[doc = "PH7 pin"]
    #[inline(always)]
    pub fn is_ph7(&self) -> bool {
        *self == EXTI7::Ph7
    }
    #[doc = "PI7 pin"]
    #[inline(always)]
    pub fn is_pi7(&self) -> bool {
        *self == EXTI7::Pi7
    }
}
#[doc = "Field `EXTI7` writer - EXTI 7 configuration bits"]
pub type EXTI7_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI7>;
impl<'a, REG> EXTI7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA7 pin"]
    #[inline(always)]
    pub fn pa7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7::Pa7)
    }
    #[doc = "PB7 pin"]
    #[inline(always)]
    pub fn pb7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7::Pb7)
    }
    #[doc = "PC7 pin"]
    #[inline(always)]
    pub fn pc7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7::Pc7)
    }
    #[doc = "PD7 pin"]
    #[inline(always)]
    pub fn pd7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7::Pd7)
    }
    #[doc = "PE7 pin"]
    #[inline(always)]
    pub fn pe7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7::Pe7)
    }
    #[doc = "PF7 pin"]
    #[inline(always)]
    pub fn pf7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7::Pf7)
    }
    #[doc = "PG7 pin"]
    #[inline(always)]
    pub fn pg7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7::Pg7)
    }
    #[doc = "PH7 pin"]
    #[inline(always)]
    pub fn ph7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7::Ph7)
    }
    #[doc = "PI7 pin"]
    #[inline(always)]
    pub fn pi7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7::Pi7)
    }
}
impl R {
    #[doc = "Bits 0:3 - EXTI 4 configuration bits"]
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 5 configuration bits"]
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 6 configuration bits"]
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 7 configuration bits"]
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 4 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti4(&mut self) -> EXTI4_W<EXTICR2rs> {
        EXTI4_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTI 5 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti5(&mut self) -> EXTI5_W<EXTICR2rs> {
        EXTI5_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTI 6 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti6(&mut self) -> EXTI6_W<EXTICR2rs> {
        EXTI6_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTI 7 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti7(&mut self) -> EXTI7_W<EXTICR2rs> {
        EXTI7_W::new(self, 12)
    }
}
#[doc = "external interrupt configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTICR2rs;
impl crate::RegisterSpec for EXTICR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exticr2::R`](R) reader structure"]
impl crate::Readable for EXTICR2rs {}
#[doc = "`write(|w| ..)` method takes [`exticr2::W`](W) writer structure"]
impl crate::Writable for EXTICR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTICR2 to value 0"]
impl crate::Resettable for EXTICR2rs {
    const RESET_VALUE: u32 = 0;
}
