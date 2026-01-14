///Register `EXTI_EXTICR4` reader
pub type R = crate::R<EXTI_EXTICR4rs>;
///Register `EXTI_EXTICR4` writer
pub type W = crate::W<EXTI_EXTICR4rs>;
/**EXTI12 GPIO port selection These bits are written by software to select the source input for EXTI12 external interrupt. Others reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI12 {
    ///0: PA\[12\] pin
    B0x00 = 0,
    ///1: PB\[12\] pin
    B0x01 = 1,
    ///2: PC\[12\] pin
    B0x02 = 2,
    ///3: PD\[12\] pin
    B0x03 = 3,
    ///5: PF\[12\] pin
    B0x05 = 5,
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
///Field `EXTI12` reader - EXTI12 GPIO port selection These bits are written by software to select the source input for EXTI12 external interrupt. Others reserved
pub type EXTI12_R = crate::FieldReader<EXTI12>;
impl EXTI12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI12> {
        match self.bits {
            0 => Some(EXTI12::B0x00),
            1 => Some(EXTI12::B0x01),
            2 => Some(EXTI12::B0x02),
            3 => Some(EXTI12::B0x03),
            5 => Some(EXTI12::B0x05),
            _ => None,
        }
    }
    ///PA\[12\] pin
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == EXTI12::B0x00
    }
    ///PB\[12\] pin
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == EXTI12::B0x01
    }
    ///PC\[12\] pin
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == EXTI12::B0x02
    }
    ///PD\[12\] pin
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == EXTI12::B0x03
    }
    ///PF\[12\] pin
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == EXTI12::B0x05
    }
}
///Field `EXTI12` writer - EXTI12 GPIO port selection These bits are written by software to select the source input for EXTI12 external interrupt. Others reserved
pub type EXTI12_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI12>;
impl<'a, REG> EXTI12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA\[12\] pin
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::B0x00)
    }
    ///PB\[12\] pin
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::B0x01)
    }
    ///PC\[12\] pin
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::B0x02)
    }
    ///PD\[12\] pin
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::B0x03)
    }
    ///PF\[12\] pin
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::B0x05)
    }
}
/**EXTI13 GPIO port selection These bits are written by software to select the source input for EXTI13 external interrupt. Others reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI13 {
    ///0: PA\[13\] pin
    B0x00 = 0,
    ///1: PB\[13\] pin
    B0x01 = 1,
    ///2: PC\[13\] pin
    B0x02 = 2,
    ///3: PD\[13\] pin
    B0x03 = 3,
    ///5: PF\[13\] pin
    B0x05 = 5,
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
///Field `EXTI13` reader - EXTI13 GPIO port selection These bits are written by software to select the source input for EXTI13 external interrupt. Others reserved
pub type EXTI13_R = crate::FieldReader<EXTI13>;
impl EXTI13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI13> {
        match self.bits {
            0 => Some(EXTI13::B0x00),
            1 => Some(EXTI13::B0x01),
            2 => Some(EXTI13::B0x02),
            3 => Some(EXTI13::B0x03),
            5 => Some(EXTI13::B0x05),
            _ => None,
        }
    }
    ///PA\[13\] pin
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == EXTI13::B0x00
    }
    ///PB\[13\] pin
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == EXTI13::B0x01
    }
    ///PC\[13\] pin
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == EXTI13::B0x02
    }
    ///PD\[13\] pin
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == EXTI13::B0x03
    }
    ///PF\[13\] pin
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == EXTI13::B0x05
    }
}
///Field `EXTI13` writer - EXTI13 GPIO port selection These bits are written by software to select the source input for EXTI13 external interrupt. Others reserved
pub type EXTI13_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI13>;
impl<'a, REG> EXTI13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA\[13\] pin
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI13::B0x00)
    }
    ///PB\[13\] pin
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI13::B0x01)
    }
    ///PC\[13\] pin
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI13::B0x02)
    }
    ///PD\[13\] pin
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI13::B0x03)
    }
    ///PF\[13\] pin
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI13::B0x05)
    }
}
/**EXTI14 GPIO port selection These bits are written by software to select the source input for EXTI14 external interrupt. Others reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI14 {
    ///0: PA\[14\] pin
    B0x00 = 0,
    ///1: PB\[14\] pin
    B0x01 = 1,
    ///2: PC\[14\] pin
    B0x02 = 2,
    ///3: PD\[14\] pin
    B0x03 = 3,
    ///5: PF\[14\] pin
    B0x05 = 5,
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
///Field `EXTI14` reader - EXTI14 GPIO port selection These bits are written by software to select the source input for EXTI14 external interrupt. Others reserved
pub type EXTI14_R = crate::FieldReader<EXTI14>;
impl EXTI14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI14> {
        match self.bits {
            0 => Some(EXTI14::B0x00),
            1 => Some(EXTI14::B0x01),
            2 => Some(EXTI14::B0x02),
            3 => Some(EXTI14::B0x03),
            5 => Some(EXTI14::B0x05),
            _ => None,
        }
    }
    ///PA\[14\] pin
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == EXTI14::B0x00
    }
    ///PB\[14\] pin
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == EXTI14::B0x01
    }
    ///PC\[14\] pin
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == EXTI14::B0x02
    }
    ///PD\[14\] pin
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == EXTI14::B0x03
    }
    ///PF\[14\] pin
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == EXTI14::B0x05
    }
}
///Field `EXTI14` writer - EXTI14 GPIO port selection These bits are written by software to select the source input for EXTI14 external interrupt. Others reserved
pub type EXTI14_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI14>;
impl<'a, REG> EXTI14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA\[14\] pin
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI14::B0x00)
    }
    ///PB\[14\] pin
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI14::B0x01)
    }
    ///PC\[14\] pin
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI14::B0x02)
    }
    ///PD\[14\] pin
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI14::B0x03)
    }
    ///PF\[14\] pin
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI14::B0x05)
    }
}
/**EXTI15 GPIO port selection These bits are written by software to select the source input for EXTI15 external interrupt. Others reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI15 {
    ///0: PA\[15\] pin
    B0x00 = 0,
    ///1: PB\[15\] pin
    B0x01 = 1,
    ///2: PC\[15\] pin
    B0x02 = 2,
    ///3: PD\[15\] pin
    B0x03 = 3,
    ///5: PF\[15\] pin
    B0x05 = 5,
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
///Field `EXTI15` reader - EXTI15 GPIO port selection These bits are written by software to select the source input for EXTI15 external interrupt. Others reserved
pub type EXTI15_R = crate::FieldReader<EXTI15>;
impl EXTI15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI15> {
        match self.bits {
            0 => Some(EXTI15::B0x00),
            1 => Some(EXTI15::B0x01),
            2 => Some(EXTI15::B0x02),
            3 => Some(EXTI15::B0x03),
            5 => Some(EXTI15::B0x05),
            _ => None,
        }
    }
    ///PA\[15\] pin
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == EXTI15::B0x00
    }
    ///PB\[15\] pin
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == EXTI15::B0x01
    }
    ///PC\[15\] pin
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == EXTI15::B0x02
    }
    ///PD\[15\] pin
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == EXTI15::B0x03
    }
    ///PF\[15\] pin
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == EXTI15::B0x05
    }
}
///Field `EXTI15` writer - EXTI15 GPIO port selection These bits are written by software to select the source input for EXTI15 external interrupt. Others reserved
pub type EXTI15_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI15>;
impl<'a, REG> EXTI15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA\[15\] pin
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI15::B0x00)
    }
    ///PB\[15\] pin
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI15::B0x01)
    }
    ///PC\[15\] pin
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI15::B0x02)
    }
    ///PD\[15\] pin
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI15::B0x03)
    }
    ///PF\[15\] pin
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI15::B0x05)
    }
}
impl R {
    ///Bits 0:7 - EXTI12 GPIO port selection These bits are written by software to select the source input for EXTI12 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - EXTI13 GPIO port selection These bits are written by software to select the source input for EXTI13 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - EXTI14 GPIO port selection These bits are written by software to select the source input for EXTI14 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - EXTI15 GPIO port selection These bits are written by software to select the source input for EXTI15 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI_EXTICR4")
            .field("exti12", &self.exti12())
            .field("exti13", &self.exti13())
            .field("exti14", &self.exti14())
            .field("exti15", &self.exti15())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - EXTI12 GPIO port selection These bits are written by software to select the source input for EXTI12 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti12(&mut self) -> EXTI12_W<'_, EXTI_EXTICR4rs> {
        EXTI12_W::new(self, 0)
    }
    ///Bits 8:15 - EXTI13 GPIO port selection These bits are written by software to select the source input for EXTI13 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti13(&mut self) -> EXTI13_W<'_, EXTI_EXTICR4rs> {
        EXTI13_W::new(self, 8)
    }
    ///Bits 16:23 - EXTI14 GPIO port selection These bits are written by software to select the source input for EXTI14 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti14(&mut self) -> EXTI14_W<'_, EXTI_EXTICR4rs> {
        EXTI14_W::new(self, 16)
    }
    ///Bits 24:31 - EXTI15 GPIO port selection These bits are written by software to select the source input for EXTI15 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti15(&mut self) -> EXTI15_W<'_, EXTI_EXTICR4rs> {
        EXTI15_W::new(self, 24)
    }
}
/**EXTI external interrupt selection register

You can [`read`](crate::Reg::read) this register and get [`exti_exticr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_exticr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#EXTI:EXTI_EXTICR4)*/
pub struct EXTI_EXTICR4rs;
impl crate::RegisterSpec for EXTI_EXTICR4rs {
    type Ux = u32;
}
///`read()` method returns [`exti_exticr4::R`](R) reader structure
impl crate::Readable for EXTI_EXTICR4rs {}
///`write(|w| ..)` method takes [`exti_exticr4::W`](W) writer structure
impl crate::Writable for EXTI_EXTICR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTI_EXTICR4 to value 0
impl crate::Resettable for EXTI_EXTICR4rs {}
