///Register `EXTICR4` reader
pub type R = crate::R<EXTICR4rs>;
///Register `EXTICR4` writer
pub type W = crate::W<EXTICR4rs>;
/**EXTI12

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI12 {
    ///0: Select PA12 as the source input for the EXTI12 external interrupt
    Pa12 = 0,
    ///1: Select PB12 as the source input for the EXTI12 external interrupt
    Pb12 = 1,
    ///2: Select PC12 as the source input for the EXTI12 external interrupt
    Pc12 = 2,
    ///3: Select PD12 as the source input for the EXTI12 external interrupt
    Pd12 = 3,
    ///4: Select PE12 as the source input for the EXTI12 external interrupt
    Pe12 = 4,
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
impl crate::IsEnum for EXTI12 {}
///Field `EXTI12` reader - EXTI12
pub type EXTI12_R = crate::FieldReader<EXTI12>;
impl EXTI12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI12> {
        match self.bits {
            0 => Some(EXTI12::Pa12),
            1 => Some(EXTI12::Pb12),
            2 => Some(EXTI12::Pc12),
            3 => Some(EXTI12::Pd12),
            4 => Some(EXTI12::Pe12),
            _ => None,
        }
    }
    ///Select PA12 as the source input for the EXTI12 external interrupt
    #[inline(always)]
    pub fn is_pa12(&self) -> bool {
        *self == EXTI12::Pa12
    }
    ///Select PB12 as the source input for the EXTI12 external interrupt
    #[inline(always)]
    pub fn is_pb12(&self) -> bool {
        *self == EXTI12::Pb12
    }
    ///Select PC12 as the source input for the EXTI12 external interrupt
    #[inline(always)]
    pub fn is_pc12(&self) -> bool {
        *self == EXTI12::Pc12
    }
    ///Select PD12 as the source input for the EXTI12 external interrupt
    #[inline(always)]
    pub fn is_pd12(&self) -> bool {
        *self == EXTI12::Pd12
    }
    ///Select PE12 as the source input for the EXTI12 external interrupt
    #[inline(always)]
    pub fn is_pe12(&self) -> bool {
        *self == EXTI12::Pe12
    }
}
///Field `EXTI12` writer - EXTI12
pub type EXTI12_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI12>;
impl<'a, REG> EXTI12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select PA12 as the source input for the EXTI12 external interrupt
    #[inline(always)]
    pub fn pa12(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Pa12)
    }
    ///Select PB12 as the source input for the EXTI12 external interrupt
    #[inline(always)]
    pub fn pb12(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Pb12)
    }
    ///Select PC12 as the source input for the EXTI12 external interrupt
    #[inline(always)]
    pub fn pc12(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Pc12)
    }
    ///Select PD12 as the source input for the EXTI12 external interrupt
    #[inline(always)]
    pub fn pd12(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Pd12)
    }
    ///Select PE12 as the source input for the EXTI12 external interrupt
    #[inline(always)]
    pub fn pe12(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Pe12)
    }
}
/**EXTI13

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI13 {
    ///0: Select PA13 as the source input for the EXTI13 external interrupt
    Pa13 = 0,
    ///1: Select PB13 as the source input for the EXTI13 external interrupt
    Pb13 = 1,
    ///2: Select PC13 as the source input for the EXTI13 external interrupt
    Pc13 = 2,
    ///3: Select PD13 as the source input for the EXTI13 external interrupt
    Pd13 = 3,
    ///4: Select PE13 as the source input for the EXTI13 external interrupt
    Pe13 = 4,
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
impl crate::IsEnum for EXTI13 {}
///Field `EXTI13` reader - EXTI13
pub type EXTI13_R = crate::FieldReader<EXTI13>;
impl EXTI13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI13> {
        match self.bits {
            0 => Some(EXTI13::Pa13),
            1 => Some(EXTI13::Pb13),
            2 => Some(EXTI13::Pc13),
            3 => Some(EXTI13::Pd13),
            4 => Some(EXTI13::Pe13),
            _ => None,
        }
    }
    ///Select PA13 as the source input for the EXTI13 external interrupt
    #[inline(always)]
    pub fn is_pa13(&self) -> bool {
        *self == EXTI13::Pa13
    }
    ///Select PB13 as the source input for the EXTI13 external interrupt
    #[inline(always)]
    pub fn is_pb13(&self) -> bool {
        *self == EXTI13::Pb13
    }
    ///Select PC13 as the source input for the EXTI13 external interrupt
    #[inline(always)]
    pub fn is_pc13(&self) -> bool {
        *self == EXTI13::Pc13
    }
    ///Select PD13 as the source input for the EXTI13 external interrupt
    #[inline(always)]
    pub fn is_pd13(&self) -> bool {
        *self == EXTI13::Pd13
    }
    ///Select PE13 as the source input for the EXTI13 external interrupt
    #[inline(always)]
    pub fn is_pe13(&self) -> bool {
        *self == EXTI13::Pe13
    }
}
///Field `EXTI13` writer - EXTI13
pub type EXTI13_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI13>;
impl<'a, REG> EXTI13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select PA13 as the source input for the EXTI13 external interrupt
    #[inline(always)]
    pub fn pa13(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI13::Pa13)
    }
    ///Select PB13 as the source input for the EXTI13 external interrupt
    #[inline(always)]
    pub fn pb13(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI13::Pb13)
    }
    ///Select PC13 as the source input for the EXTI13 external interrupt
    #[inline(always)]
    pub fn pc13(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI13::Pc13)
    }
    ///Select PD13 as the source input for the EXTI13 external interrupt
    #[inline(always)]
    pub fn pd13(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI13::Pd13)
    }
    ///Select PE13 as the source input for the EXTI13 external interrupt
    #[inline(always)]
    pub fn pe13(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI13::Pe13)
    }
}
/**EXTI14

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI14 {
    ///0: Select PA14 as the source input for the EXTI14 external interrupt
    Pa14 = 0,
    ///1: Select PB14 as the source input for the EXTI14 external interrupt
    Pb14 = 1,
    ///2: Select PC14 as the source input for the EXTI14 external interrupt
    Pc14 = 2,
    ///3: Select PD14 as the source input for the EXTI14 external interrupt
    Pd14 = 3,
    ///4: Select PE14 as the source input for the EXTI14 external interrupt
    Pe14 = 4,
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
impl crate::IsEnum for EXTI14 {}
///Field `EXTI14` reader - EXTI14
pub type EXTI14_R = crate::FieldReader<EXTI14>;
impl EXTI14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI14> {
        match self.bits {
            0 => Some(EXTI14::Pa14),
            1 => Some(EXTI14::Pb14),
            2 => Some(EXTI14::Pc14),
            3 => Some(EXTI14::Pd14),
            4 => Some(EXTI14::Pe14),
            _ => None,
        }
    }
    ///Select PA14 as the source input for the EXTI14 external interrupt
    #[inline(always)]
    pub fn is_pa14(&self) -> bool {
        *self == EXTI14::Pa14
    }
    ///Select PB14 as the source input for the EXTI14 external interrupt
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        *self == EXTI14::Pb14
    }
    ///Select PC14 as the source input for the EXTI14 external interrupt
    #[inline(always)]
    pub fn is_pc14(&self) -> bool {
        *self == EXTI14::Pc14
    }
    ///Select PD14 as the source input for the EXTI14 external interrupt
    #[inline(always)]
    pub fn is_pd14(&self) -> bool {
        *self == EXTI14::Pd14
    }
    ///Select PE14 as the source input for the EXTI14 external interrupt
    #[inline(always)]
    pub fn is_pe14(&self) -> bool {
        *self == EXTI14::Pe14
    }
}
///Field `EXTI14` writer - EXTI14
pub type EXTI14_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI14>;
impl<'a, REG> EXTI14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select PA14 as the source input for the EXTI14 external interrupt
    #[inline(always)]
    pub fn pa14(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI14::Pa14)
    }
    ///Select PB14 as the source input for the EXTI14 external interrupt
    #[inline(always)]
    pub fn pb14(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI14::Pb14)
    }
    ///Select PC14 as the source input for the EXTI14 external interrupt
    #[inline(always)]
    pub fn pc14(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI14::Pc14)
    }
    ///Select PD14 as the source input for the EXTI14 external interrupt
    #[inline(always)]
    pub fn pd14(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI14::Pd14)
    }
    ///Select PE14 as the source input for the EXTI14 external interrupt
    #[inline(always)]
    pub fn pe14(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI14::Pe14)
    }
}
/**EXTI x configuration (x = 12 to 15)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI15 {
    ///0: Select PA15 as the source input for the EXTI15 external interrupt
    Pa15 = 0,
    ///1: Select PB15 as the source input for the EXTI15 external interrupt
    Pb15 = 1,
    ///2: Select PC15 as the source input for the EXTI15 external interrupt
    Pc15 = 2,
    ///3: Select PD15 as the source input for the EXTI15 external interrupt
    Pd15 = 3,
    ///4: Select PE15 as the source input for the EXTI15 external interrupt
    Pe15 = 4,
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
impl crate::IsEnum for EXTI15 {}
///Field `EXTI15` reader - EXTI x configuration (x = 12 to 15)
pub type EXTI15_R = crate::FieldReader<EXTI15>;
impl EXTI15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI15> {
        match self.bits {
            0 => Some(EXTI15::Pa15),
            1 => Some(EXTI15::Pb15),
            2 => Some(EXTI15::Pc15),
            3 => Some(EXTI15::Pd15),
            4 => Some(EXTI15::Pe15),
            _ => None,
        }
    }
    ///Select PA15 as the source input for the EXTI15 external interrupt
    #[inline(always)]
    pub fn is_pa15(&self) -> bool {
        *self == EXTI15::Pa15
    }
    ///Select PB15 as the source input for the EXTI15 external interrupt
    #[inline(always)]
    pub fn is_pb15(&self) -> bool {
        *self == EXTI15::Pb15
    }
    ///Select PC15 as the source input for the EXTI15 external interrupt
    #[inline(always)]
    pub fn is_pc15(&self) -> bool {
        *self == EXTI15::Pc15
    }
    ///Select PD15 as the source input for the EXTI15 external interrupt
    #[inline(always)]
    pub fn is_pd15(&self) -> bool {
        *self == EXTI15::Pd15
    }
    ///Select PE15 as the source input for the EXTI15 external interrupt
    #[inline(always)]
    pub fn is_pe15(&self) -> bool {
        *self == EXTI15::Pe15
    }
}
///Field `EXTI15` writer - EXTI x configuration (x = 12 to 15)
pub type EXTI15_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI15>;
impl<'a, REG> EXTI15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select PA15 as the source input for the EXTI15 external interrupt
    #[inline(always)]
    pub fn pa15(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI15::Pa15)
    }
    ///Select PB15 as the source input for the EXTI15 external interrupt
    #[inline(always)]
    pub fn pb15(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI15::Pb15)
    }
    ///Select PC15 as the source input for the EXTI15 external interrupt
    #[inline(always)]
    pub fn pc15(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI15::Pc15)
    }
    ///Select PD15 as the source input for the EXTI15 external interrupt
    #[inline(always)]
    pub fn pd15(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI15::Pd15)
    }
    ///Select PE15 as the source input for the EXTI15 external interrupt
    #[inline(always)]
    pub fn pe15(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI15::Pe15)
    }
}
impl R {
    ///Bits 0:3 - EXTI12
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - EXTI13
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - EXTI14
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - EXTI x configuration (x = 12 to 15)
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTICR4")
            .field("exti15", &self.exti15())
            .field("exti14", &self.exti14())
            .field("exti13", &self.exti13())
            .field("exti12", &self.exti12())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - EXTI12
    #[inline(always)]
    pub fn exti12(&mut self) -> EXTI12_W<EXTICR4rs> {
        EXTI12_W::new(self, 0)
    }
    ///Bits 4:7 - EXTI13
    #[inline(always)]
    pub fn exti13(&mut self) -> EXTI13_W<EXTICR4rs> {
        EXTI13_W::new(self, 4)
    }
    ///Bits 8:11 - EXTI14
    #[inline(always)]
    pub fn exti14(&mut self) -> EXTI14_W<EXTICR4rs> {
        EXTI14_W::new(self, 8)
    }
    ///Bits 12:15 - EXTI x configuration (x = 12 to 15)
    #[inline(always)]
    pub fn exti15(&mut self) -> EXTI15_W<EXTICR4rs> {
        EXTI15_W::new(self, 12)
    }
}
/**external interrupt configuration register 4

You can [`read`](crate::Reg::read) this register and get [`exticr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#SYSCFG:EXTICR4)*/
pub struct EXTICR4rs;
impl crate::RegisterSpec for EXTICR4rs {
    type Ux = u32;
}
///`read()` method returns [`exticr4::R`](R) reader structure
impl crate::Readable for EXTICR4rs {}
///`write(|w| ..)` method takes [`exticr4::W`](W) writer structure
impl crate::Writable for EXTICR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EXTICR4 to value 0
impl crate::Resettable for EXTICR4rs {
    const RESET_VALUE: u32 = 0;
}
