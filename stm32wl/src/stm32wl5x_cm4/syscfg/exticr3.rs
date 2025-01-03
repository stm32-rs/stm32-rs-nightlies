///Register `EXTICR3` reader
pub type R = crate::R<EXTICR3rs>;
///Register `EXTICR3` writer
pub type W = crate::W<EXTICR3rs>;
/**EXTI 8 configuration bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI8 {
    ///0: Select PA8 as the source input for the EXTI8 external interrupt
    Pa8 = 0,
    ///1: Select PB8 as the source input for the EXTI8 external interrupt
    Pb8 = 1,
}
impl From<EXTI8> for u8 {
    #[inline(always)]
    fn from(variant: EXTI8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI8 {
    type Ux = u8;
}
impl crate::IsEnum for EXTI8 {}
///Field `EXTI8` reader - EXTI 8 configuration bits
pub type EXTI8_R = crate::FieldReader<EXTI8>;
impl EXTI8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI8> {
        match self.bits {
            0 => Some(EXTI8::Pa8),
            1 => Some(EXTI8::Pb8),
            _ => None,
        }
    }
    ///Select PA8 as the source input for the EXTI8 external interrupt
    #[inline(always)]
    pub fn is_pa8(&self) -> bool {
        *self == EXTI8::Pa8
    }
    ///Select PB8 as the source input for the EXTI8 external interrupt
    #[inline(always)]
    pub fn is_pb8(&self) -> bool {
        *self == EXTI8::Pb8
    }
}
///Field `EXTI8` writer - EXTI 8 configuration bits
pub type EXTI8_W<'a, REG> = crate::FieldWriter<'a, REG, 3, EXTI8>;
impl<'a, REG> EXTI8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select PA8 as the source input for the EXTI8 external interrupt
    #[inline(always)]
    pub fn pa8(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8::Pa8)
    }
    ///Select PB8 as the source input for the EXTI8 external interrupt
    #[inline(always)]
    pub fn pb8(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8::Pb8)
    }
}
/**EXTI 9 configuration bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI9 {
    ///0: Select PA9 as the source input for the EXTI9 external interrupt
    Pa9 = 0,
    ///1: Select PB9 as the source input for the EXTI9 external interrupt
    Pb9 = 1,
}
impl From<EXTI9> for u8 {
    #[inline(always)]
    fn from(variant: EXTI9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI9 {
    type Ux = u8;
}
impl crate::IsEnum for EXTI9 {}
///Field `EXTI9` reader - EXTI 9 configuration bits
pub type EXTI9_R = crate::FieldReader<EXTI9>;
impl EXTI9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI9> {
        match self.bits {
            0 => Some(EXTI9::Pa9),
            1 => Some(EXTI9::Pb9),
            _ => None,
        }
    }
    ///Select PA9 as the source input for the EXTI9 external interrupt
    #[inline(always)]
    pub fn is_pa9(&self) -> bool {
        *self == EXTI9::Pa9
    }
    ///Select PB9 as the source input for the EXTI9 external interrupt
    #[inline(always)]
    pub fn is_pb9(&self) -> bool {
        *self == EXTI9::Pb9
    }
}
///Field `EXTI9` writer - EXTI 9 configuration bits
pub type EXTI9_W<'a, REG> = crate::FieldWriter<'a, REG, 3, EXTI9>;
impl<'a, REG> EXTI9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select PA9 as the source input for the EXTI9 external interrupt
    #[inline(always)]
    pub fn pa9(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI9::Pa9)
    }
    ///Select PB9 as the source input for the EXTI9 external interrupt
    #[inline(always)]
    pub fn pb9(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI9::Pb9)
    }
}
/**EXTI 10 configuration bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI10 {
    ///0: Select PA10 as the source input for the EXTI10 external interrupt
    Pa10 = 0,
    ///1: Select PB10 as the source input for the EXTI10 external interrupt
    Pb10 = 1,
}
impl From<EXTI10> for u8 {
    #[inline(always)]
    fn from(variant: EXTI10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI10 {
    type Ux = u8;
}
impl crate::IsEnum for EXTI10 {}
///Field `EXTI10` reader - EXTI 10 configuration bits
pub type EXTI10_R = crate::FieldReader<EXTI10>;
impl EXTI10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI10> {
        match self.bits {
            0 => Some(EXTI10::Pa10),
            1 => Some(EXTI10::Pb10),
            _ => None,
        }
    }
    ///Select PA10 as the source input for the EXTI10 external interrupt
    #[inline(always)]
    pub fn is_pa10(&self) -> bool {
        *self == EXTI10::Pa10
    }
    ///Select PB10 as the source input for the EXTI10 external interrupt
    #[inline(always)]
    pub fn is_pb10(&self) -> bool {
        *self == EXTI10::Pb10
    }
}
///Field `EXTI10` writer - EXTI 10 configuration bits
pub type EXTI10_W<'a, REG> = crate::FieldWriter<'a, REG, 3, EXTI10>;
impl<'a, REG> EXTI10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select PA10 as the source input for the EXTI10 external interrupt
    #[inline(always)]
    pub fn pa10(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI10::Pa10)
    }
    ///Select PB10 as the source input for the EXTI10 external interrupt
    #[inline(always)]
    pub fn pb10(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI10::Pb10)
    }
}
/**EXTI 11 configuration bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI11 {
    ///0: Select PA11 as the source input for the EXTI11 external interrupt
    Pa11 = 0,
    ///1: Select PB11 as the source input for the EXTI11 external interrupt
    Pb11 = 1,
}
impl From<EXTI11> for u8 {
    #[inline(always)]
    fn from(variant: EXTI11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI11 {
    type Ux = u8;
}
impl crate::IsEnum for EXTI11 {}
///Field `EXTI11` reader - EXTI 11 configuration bits
pub type EXTI11_R = crate::FieldReader<EXTI11>;
impl EXTI11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI11> {
        match self.bits {
            0 => Some(EXTI11::Pa11),
            1 => Some(EXTI11::Pb11),
            _ => None,
        }
    }
    ///Select PA11 as the source input for the EXTI11 external interrupt
    #[inline(always)]
    pub fn is_pa11(&self) -> bool {
        *self == EXTI11::Pa11
    }
    ///Select PB11 as the source input for the EXTI11 external interrupt
    #[inline(always)]
    pub fn is_pb11(&self) -> bool {
        *self == EXTI11::Pb11
    }
}
///Field `EXTI11` writer - EXTI 11 configuration bits
pub type EXTI11_W<'a, REG> = crate::FieldWriter<'a, REG, 3, EXTI11>;
impl<'a, REG> EXTI11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select PA11 as the source input for the EXTI11 external interrupt
    #[inline(always)]
    pub fn pa11(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI11::Pa11)
    }
    ///Select PB11 as the source input for the EXTI11 external interrupt
    #[inline(always)]
    pub fn pb11(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI11::Pb11)
    }
}
impl R {
    ///Bits 0:2 - EXTI 8 configuration bits
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - EXTI 9 configuration bits
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - EXTI 10 configuration bits
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - EXTI 11 configuration bits
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTICR3")
            .field("exti11", &self.exti11())
            .field("exti10", &self.exti10())
            .field("exti9", &self.exti9())
            .field("exti8", &self.exti8())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - EXTI 8 configuration bits
    #[inline(always)]
    pub fn exti8(&mut self) -> EXTI8_W<EXTICR3rs> {
        EXTI8_W::new(self, 0)
    }
    ///Bits 4:6 - EXTI 9 configuration bits
    #[inline(always)]
    pub fn exti9(&mut self) -> EXTI9_W<EXTICR3rs> {
        EXTI9_W::new(self, 4)
    }
    ///Bits 8:10 - EXTI 10 configuration bits
    #[inline(always)]
    pub fn exti10(&mut self) -> EXTI10_W<EXTICR3rs> {
        EXTI10_W::new(self, 8)
    }
    ///Bits 12:14 - EXTI 11 configuration bits
    #[inline(always)]
    pub fn exti11(&mut self) -> EXTI11_W<EXTICR3rs> {
        EXTI11_W::new(self, 12)
    }
}
/**external interrupt configuration register 3

You can [`read`](crate::Reg::read) this register and get [`exticr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#SYSCFG:EXTICR3)*/
pub struct EXTICR3rs;
impl crate::RegisterSpec for EXTICR3rs {
    type Ux = u32;
}
///`read()` method returns [`exticr3::R`](R) reader structure
impl crate::Readable for EXTICR3rs {}
///`write(|w| ..)` method takes [`exticr3::W`](W) writer structure
impl crate::Writable for EXTICR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EXTICR3 to value 0
impl crate::Resettable for EXTICR3rs {
    const RESET_VALUE: u32 = 0;
}
