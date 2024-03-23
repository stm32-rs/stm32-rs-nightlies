#[doc = "Register `EXTICR2` reader"]
pub type R = crate::R<EXTICR2rs>;
#[doc = "Register `EXTICR2` writer"]
pub type W = crate::W<EXTICR2rs>;
#[doc = "EXTI x configuration (x = 4 to 7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI4 {
    #[doc = "0: Select PA4 as the source input for the EXTI4 external interrupt"]
    Pa4 = 0,
    #[doc = "1: Select PB4 as the source input for the EXTI4 external interrupt"]
    Pb4 = 1,
    #[doc = "2: Select PC4 as the source input for the EXTI4 external interrupt"]
    Pc4 = 2,
    #[doc = "3: Select PD4 as the source input for the EXTI4 external interrupt"]
    Pd4 = 3,
    #[doc = "4: Select PE4 as the source input for the EXTI4 external interrupt"]
    Pe4 = 4,
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
#[doc = "Field `EXTI4` reader - EXTI x configuration (x = 4 to 7)"]
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
            _ => None,
        }
    }
    #[doc = "Select PA4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == EXTI4::Pa4
    }
    #[doc = "Select PB4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn is_pb4(&self) -> bool {
        *self == EXTI4::Pb4
    }
    #[doc = "Select PC4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn is_pc4(&self) -> bool {
        *self == EXTI4::Pc4
    }
    #[doc = "Select PD4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn is_pd4(&self) -> bool {
        *self == EXTI4::Pd4
    }
    #[doc = "Select PE4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn is_pe4(&self) -> bool {
        *self == EXTI4::Pe4
    }
}
#[doc = "Field `EXTI4` writer - EXTI x configuration (x = 4 to 7)"]
pub type EXTI4_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI4>;
impl<'a, REG> EXTI4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select PA4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Pa4)
    }
    #[doc = "Select PB4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn pb4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Pb4)
    }
    #[doc = "Select PC4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn pc4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Pc4)
    }
    #[doc = "Select PD4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn pd4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Pd4)
    }
    #[doc = "Select PE4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn pe4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Pe4)
    }
}
#[doc = "EXTI x configuration (x = 4 to 7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI5 {
    #[doc = "0: Select PA5 as the source input for the EXTI5 external interrupt"]
    Pa5 = 0,
    #[doc = "1: Select PB5 as the source input for the EXTI5 external interrupt"]
    Pb5 = 1,
    #[doc = "2: Select PC5 as the source input for the EXTI5 external interrupt"]
    Pc5 = 2,
    #[doc = "3: Select PD5 as the source input for the EXTI5 external interrupt"]
    Pd5 = 3,
    #[doc = "4: Select PE5 as the source input for the EXTI5 external interrupt"]
    Pe5 = 4,
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
#[doc = "Field `EXTI5` reader - EXTI x configuration (x = 4 to 7)"]
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
            _ => None,
        }
    }
    #[doc = "Select PA5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == EXTI5::Pa5
    }
    #[doc = "Select PB5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn is_pb5(&self) -> bool {
        *self == EXTI5::Pb5
    }
    #[doc = "Select PC5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        *self == EXTI5::Pc5
    }
    #[doc = "Select PD5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn is_pd5(&self) -> bool {
        *self == EXTI5::Pd5
    }
    #[doc = "Select PE5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn is_pe5(&self) -> bool {
        *self == EXTI5::Pe5
    }
}
#[doc = "Field `EXTI5` writer - EXTI x configuration (x = 4 to 7)"]
pub type EXTI5_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI5>;
impl<'a, REG> EXTI5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select PA5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5::Pa5)
    }
    #[doc = "Select PB5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn pb5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5::Pb5)
    }
    #[doc = "Select PC5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn pc5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5::Pc5)
    }
    #[doc = "Select PD5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn pd5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5::Pd5)
    }
    #[doc = "Select PE5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn pe5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5::Pe5)
    }
}
#[doc = "EXTI x configuration (x = 4 to 7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI6 {
    #[doc = "0: Select PA6 as the source input for the EXTI6 external interrupt"]
    Pa6 = 0,
    #[doc = "1: Select PB6 as the source input for the EXTI6 external interrupt"]
    Pb6 = 1,
    #[doc = "2: Select PC6 as the source input for the EXTI6 external interrupt"]
    Pc6 = 2,
    #[doc = "3: Select PD6 as the source input for the EXTI6 external interrupt"]
    Pd6 = 3,
    #[doc = "4: Select PE6 as the source input for the EXTI6 external interrupt"]
    Pe6 = 4,
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
#[doc = "Field `EXTI6` reader - EXTI x configuration (x = 4 to 7)"]
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
            _ => None,
        }
    }
    #[doc = "Select PA6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn is_pa6(&self) -> bool {
        *self == EXTI6::Pa6
    }
    #[doc = "Select PB6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn is_pb6(&self) -> bool {
        *self == EXTI6::Pb6
    }
    #[doc = "Select PC6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn is_pc6(&self) -> bool {
        *self == EXTI6::Pc6
    }
    #[doc = "Select PD6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn is_pd6(&self) -> bool {
        *self == EXTI6::Pd6
    }
    #[doc = "Select PE6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn is_pe6(&self) -> bool {
        *self == EXTI6::Pe6
    }
}
#[doc = "Field `EXTI6` writer - EXTI x configuration (x = 4 to 7)"]
pub type EXTI6_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI6>;
impl<'a, REG> EXTI6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select PA6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn pa6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6::Pa6)
    }
    #[doc = "Select PB6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn pb6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6::Pb6)
    }
    #[doc = "Select PC6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn pc6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6::Pc6)
    }
    #[doc = "Select PD6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn pd6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6::Pd6)
    }
    #[doc = "Select PE6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn pe6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6::Pe6)
    }
}
#[doc = "EXTI x configuration (x = 4 to 7)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI7 {
    #[doc = "0: Select PA7 as the source input for the EXTI7 external interrupt"]
    Pa7 = 0,
    #[doc = "1: Select PB7 as the source input for the EXTI7 external interrupt"]
    Pb7 = 1,
    #[doc = "2: Select PC7 as the source input for the EXTI7 external interrupt"]
    Pc7 = 2,
    #[doc = "3: Select PD7 as the source input for the EXTI7 external interrupt"]
    Pd7 = 3,
    #[doc = "4: Select PE7 as the source input for the EXTI7 external interrupt"]
    Pe7 = 4,
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
#[doc = "Field `EXTI7` reader - EXTI x configuration (x = 4 to 7)"]
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
            _ => None,
        }
    }
    #[doc = "Select PA7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        *self == EXTI7::Pa7
    }
    #[doc = "Select PB7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn is_pb7(&self) -> bool {
        *self == EXTI7::Pb7
    }
    #[doc = "Select PC7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn is_pc7(&self) -> bool {
        *self == EXTI7::Pc7
    }
    #[doc = "Select PD7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn is_pd7(&self) -> bool {
        *self == EXTI7::Pd7
    }
    #[doc = "Select PE7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn is_pe7(&self) -> bool {
        *self == EXTI7::Pe7
    }
}
#[doc = "Field `EXTI7` writer - EXTI x configuration (x = 4 to 7)"]
pub type EXTI7_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI7>;
impl<'a, REG> EXTI7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select PA7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn pa7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7::Pa7)
    }
    #[doc = "Select PB7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn pb7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7::Pb7)
    }
    #[doc = "Select PC7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn pc7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7::Pc7)
    }
    #[doc = "Select PD7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn pd7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7::Pd7)
    }
    #[doc = "Select PE7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn pe7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7::Pe7)
    }
}
impl R {
    #[doc = "Bits 0:3 - EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    #[must_use]
    pub fn exti4(&mut self) -> EXTI4_W<EXTICR2rs> {
        EXTI4_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    #[must_use]
    pub fn exti5(&mut self) -> EXTI5_W<EXTICR2rs> {
        EXTI5_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTI x configuration (x = 4 to 7)"]
    #[inline(always)]
    #[must_use]
    pub fn exti6(&mut self) -> EXTI6_W<EXTICR2rs> {
        EXTI6_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTI x configuration (x = 4 to 7)"]
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
