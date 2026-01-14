///Register `EXTI_EXTICR3` reader
pub type R = crate::R<EXTI_EXTICR3rs>;
///Register `EXTI_EXTICR3` writer
pub type W = crate::W<EXTI_EXTICR3rs>;
/**EXTI8 GPIO port selection These bits are written by software to select the source input for EXTI8 external interrupt. Others reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI8 {
    ///0: PA\[8\] pin
    B0x00 = 0,
    ///1: PB\[8\] pin
    B0x01 = 1,
    ///2: PC\[8\] pin
    B0x02 = 2,
    ///3: PD\[8\] pin
    B0x03 = 3,
    ///5: PF\[8\] pin
    B0x05 = 5,
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
///Field `EXTI8` reader - EXTI8 GPIO port selection These bits are written by software to select the source input for EXTI8 external interrupt. Others reserved
pub type EXTI8_R = crate::FieldReader<EXTI8>;
impl EXTI8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI8> {
        match self.bits {
            0 => Some(EXTI8::B0x00),
            1 => Some(EXTI8::B0x01),
            2 => Some(EXTI8::B0x02),
            3 => Some(EXTI8::B0x03),
            5 => Some(EXTI8::B0x05),
            _ => None,
        }
    }
    ///PA\[8\] pin
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == EXTI8::B0x00
    }
    ///PB\[8\] pin
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == EXTI8::B0x01
    }
    ///PC\[8\] pin
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == EXTI8::B0x02
    }
    ///PD\[8\] pin
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == EXTI8::B0x03
    }
    ///PF\[8\] pin
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == EXTI8::B0x05
    }
}
///Field `EXTI8` writer - EXTI8 GPIO port selection These bits are written by software to select the source input for EXTI8 external interrupt. Others reserved
pub type EXTI8_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI8>;
impl<'a, REG> EXTI8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA\[8\] pin
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8::B0x00)
    }
    ///PB\[8\] pin
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8::B0x01)
    }
    ///PC\[8\] pin
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8::B0x02)
    }
    ///PD\[8\] pin
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8::B0x03)
    }
    ///PF\[8\] pin
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8::B0x05)
    }
}
/**EXTI9 GPIO port selection These bits are written by software to select the source input for EXTI9 external interrupt. Others reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI9 {
    ///0: PA\[9\] pin
    B0x00 = 0,
    ///1: PB\[9\] pin
    B0x01 = 1,
    ///2: PC\[9\] pin
    B0x02 = 2,
    ///3: PD\[9\] pin
    B0x03 = 3,
    ///5: PF\[9\] pin
    B0x05 = 5,
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
///Field `EXTI9` reader - EXTI9 GPIO port selection These bits are written by software to select the source input for EXTI9 external interrupt. Others reserved
pub type EXTI9_R = crate::FieldReader<EXTI9>;
impl EXTI9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI9> {
        match self.bits {
            0 => Some(EXTI9::B0x00),
            1 => Some(EXTI9::B0x01),
            2 => Some(EXTI9::B0x02),
            3 => Some(EXTI9::B0x03),
            5 => Some(EXTI9::B0x05),
            _ => None,
        }
    }
    ///PA\[9\] pin
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == EXTI9::B0x00
    }
    ///PB\[9\] pin
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == EXTI9::B0x01
    }
    ///PC\[9\] pin
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == EXTI9::B0x02
    }
    ///PD\[9\] pin
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == EXTI9::B0x03
    }
    ///PF\[9\] pin
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == EXTI9::B0x05
    }
}
///Field `EXTI9` writer - EXTI9 GPIO port selection These bits are written by software to select the source input for EXTI9 external interrupt. Others reserved
pub type EXTI9_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI9>;
impl<'a, REG> EXTI9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA\[9\] pin
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI9::B0x00)
    }
    ///PB\[9\] pin
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI9::B0x01)
    }
    ///PC\[9\] pin
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI9::B0x02)
    }
    ///PD\[9\] pin
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI9::B0x03)
    }
    ///PF\[9\] pin
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI9::B0x05)
    }
}
/**EXTI10 GPIO port selection These bits are written by software to select the source input for EXTI10 external interrupt. Others reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI10 {
    ///0: PA\[10\] pin
    B0x00 = 0,
    ///1: PB\[10\] pin
    B0x01 = 1,
    ///2: PC\[10\] pin
    B0x02 = 2,
    ///3: PD\[10\] pin
    B0x03 = 3,
    ///5: PF\[10\] pin
    B0x05 = 5,
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
///Field `EXTI10` reader - EXTI10 GPIO port selection These bits are written by software to select the source input for EXTI10 external interrupt. Others reserved
pub type EXTI10_R = crate::FieldReader<EXTI10>;
impl EXTI10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI10> {
        match self.bits {
            0 => Some(EXTI10::B0x00),
            1 => Some(EXTI10::B0x01),
            2 => Some(EXTI10::B0x02),
            3 => Some(EXTI10::B0x03),
            5 => Some(EXTI10::B0x05),
            _ => None,
        }
    }
    ///PA\[10\] pin
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == EXTI10::B0x00
    }
    ///PB\[10\] pin
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == EXTI10::B0x01
    }
    ///PC\[10\] pin
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == EXTI10::B0x02
    }
    ///PD\[10\] pin
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == EXTI10::B0x03
    }
    ///PF\[10\] pin
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == EXTI10::B0x05
    }
}
///Field `EXTI10` writer - EXTI10 GPIO port selection These bits are written by software to select the source input for EXTI10 external interrupt. Others reserved
pub type EXTI10_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI10>;
impl<'a, REG> EXTI10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA\[10\] pin
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI10::B0x00)
    }
    ///PB\[10\] pin
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI10::B0x01)
    }
    ///PC\[10\] pin
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI10::B0x02)
    }
    ///PD\[10\] pin
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI10::B0x03)
    }
    ///PF\[10\] pin
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI10::B0x05)
    }
}
/**EXTI11 GPIO port selection These bits are written by software to select the source input for EXTI11 external interrupt. Others reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI11 {
    ///0: PA\[11\] pin
    B0x00 = 0,
    ///1: PB\[11\] pin
    B0x01 = 1,
    ///2: PC\[11\] pin
    B0x02 = 2,
    ///3: PD\[11\] pin
    B0x03 = 3,
    ///5: PF\[11\] pin
    B0x05 = 5,
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
///Field `EXTI11` reader - EXTI11 GPIO port selection These bits are written by software to select the source input for EXTI11 external interrupt. Others reserved
pub type EXTI11_R = crate::FieldReader<EXTI11>;
impl EXTI11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI11> {
        match self.bits {
            0 => Some(EXTI11::B0x00),
            1 => Some(EXTI11::B0x01),
            2 => Some(EXTI11::B0x02),
            3 => Some(EXTI11::B0x03),
            5 => Some(EXTI11::B0x05),
            _ => None,
        }
    }
    ///PA\[11\] pin
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == EXTI11::B0x00
    }
    ///PB\[11\] pin
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == EXTI11::B0x01
    }
    ///PC\[11\] pin
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == EXTI11::B0x02
    }
    ///PD\[11\] pin
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == EXTI11::B0x03
    }
    ///PF\[11\] pin
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == EXTI11::B0x05
    }
}
///Field `EXTI11` writer - EXTI11 GPIO port selection These bits are written by software to select the source input for EXTI11 external interrupt. Others reserved
pub type EXTI11_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI11>;
impl<'a, REG> EXTI11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA\[11\] pin
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI11::B0x00)
    }
    ///PB\[11\] pin
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI11::B0x01)
    }
    ///PC\[11\] pin
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI11::B0x02)
    }
    ///PD\[11\] pin
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI11::B0x03)
    }
    ///PF\[11\] pin
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI11::B0x05)
    }
}
impl R {
    ///Bits 0:7 - EXTI8 GPIO port selection These bits are written by software to select the source input for EXTI8 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - EXTI9 GPIO port selection These bits are written by software to select the source input for EXTI9 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - EXTI10 GPIO port selection These bits are written by software to select the source input for EXTI10 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - EXTI11 GPIO port selection These bits are written by software to select the source input for EXTI11 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI_EXTICR3")
            .field("exti8", &self.exti8())
            .field("exti9", &self.exti9())
            .field("exti10", &self.exti10())
            .field("exti11", &self.exti11())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - EXTI8 GPIO port selection These bits are written by software to select the source input for EXTI8 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti8(&mut self) -> EXTI8_W<'_, EXTI_EXTICR3rs> {
        EXTI8_W::new(self, 0)
    }
    ///Bits 8:15 - EXTI9 GPIO port selection These bits are written by software to select the source input for EXTI9 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti9(&mut self) -> EXTI9_W<'_, EXTI_EXTICR3rs> {
        EXTI9_W::new(self, 8)
    }
    ///Bits 16:23 - EXTI10 GPIO port selection These bits are written by software to select the source input for EXTI10 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti10(&mut self) -> EXTI10_W<'_, EXTI_EXTICR3rs> {
        EXTI10_W::new(self, 16)
    }
    ///Bits 24:31 - EXTI11 GPIO port selection These bits are written by software to select the source input for EXTI11 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti11(&mut self) -> EXTI11_W<'_, EXTI_EXTICR3rs> {
        EXTI11_W::new(self, 24)
    }
}
/**EXTI external interrupt selection register

You can [`read`](crate::Reg::read) this register and get [`exti_exticr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_exticr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#EXTI:EXTI_EXTICR3)*/
pub struct EXTI_EXTICR3rs;
impl crate::RegisterSpec for EXTI_EXTICR3rs {
    type Ux = u32;
}
///`read()` method returns [`exti_exticr3::R`](R) reader structure
impl crate::Readable for EXTI_EXTICR3rs {}
///`write(|w| ..)` method takes [`exti_exticr3::W`](W) writer structure
impl crate::Writable for EXTI_EXTICR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTI_EXTICR3 to value 0
impl crate::Resettable for EXTI_EXTICR3rs {}
