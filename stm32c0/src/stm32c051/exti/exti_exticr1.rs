///Register `EXTI_EXTICR1` reader
pub type R = crate::R<EXTI_EXTICR1rs>;
///Register `EXTI_EXTICR1` writer
pub type W = crate::W<EXTI_EXTICR1rs>;
/**EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. Others reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI0 {
    ///0: PA\[0\] pin
    B0x00 = 0,
    ///1: PB\[0\] pin
    B0x01 = 1,
    ///2: PC\[0\] pin
    B0x02 = 2,
    ///3: PD\[0\] pin
    B0x03 = 3,
    ///5: PF\[0\] pin
    B0x05 = 5,
}
impl From<EXTI0> for u8 {
    #[inline(always)]
    fn from(variant: EXTI0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI0 {
    type Ux = u8;
}
impl crate::IsEnum for EXTI0 {}
///Field `EXTI0` reader - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. Others reserved
pub type EXTI0_R = crate::FieldReader<EXTI0>;
impl EXTI0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI0> {
        match self.bits {
            0 => Some(EXTI0::B0x00),
            1 => Some(EXTI0::B0x01),
            2 => Some(EXTI0::B0x02),
            3 => Some(EXTI0::B0x03),
            5 => Some(EXTI0::B0x05),
            _ => None,
        }
    }
    ///PA\[0\] pin
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == EXTI0::B0x00
    }
    ///PB\[0\] pin
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == EXTI0::B0x01
    }
    ///PC\[0\] pin
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == EXTI0::B0x02
    }
    ///PD\[0\] pin
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == EXTI0::B0x03
    }
    ///PF\[0\] pin
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == EXTI0::B0x05
    }
}
///Field `EXTI0` writer - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. Others reserved
pub type EXTI0_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI0>;
impl<'a, REG> EXTI0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA\[0\] pin
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::B0x00)
    }
    ///PB\[0\] pin
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::B0x01)
    }
    ///PC\[0\] pin
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::B0x02)
    }
    ///PD\[0\] pin
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::B0x03)
    }
    ///PF\[0\] pin
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::B0x05)
    }
}
/**EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. Others reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI1 {
    ///0: PA\[1\] pin
    B0x00 = 0,
    ///1: PB\[1\] pin
    B0x01 = 1,
    ///2: PC\[1\] pin
    B0x02 = 2,
    ///3: PD\[1\] pin
    B0x03 = 3,
    ///5: PF\[1\] pin
    B0x05 = 5,
}
impl From<EXTI1> for u8 {
    #[inline(always)]
    fn from(variant: EXTI1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI1 {
    type Ux = u8;
}
impl crate::IsEnum for EXTI1 {}
///Field `EXTI1` reader - EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. Others reserved
pub type EXTI1_R = crate::FieldReader<EXTI1>;
impl EXTI1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI1> {
        match self.bits {
            0 => Some(EXTI1::B0x00),
            1 => Some(EXTI1::B0x01),
            2 => Some(EXTI1::B0x02),
            3 => Some(EXTI1::B0x03),
            5 => Some(EXTI1::B0x05),
            _ => None,
        }
    }
    ///PA\[1\] pin
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == EXTI1::B0x00
    }
    ///PB\[1\] pin
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == EXTI1::B0x01
    }
    ///PC\[1\] pin
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == EXTI1::B0x02
    }
    ///PD\[1\] pin
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == EXTI1::B0x03
    }
    ///PF\[1\] pin
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == EXTI1::B0x05
    }
}
///Field `EXTI1` writer - EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. Others reserved
pub type EXTI1_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI1>;
impl<'a, REG> EXTI1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA\[1\] pin
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::B0x00)
    }
    ///PB\[1\] pin
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::B0x01)
    }
    ///PC\[1\] pin
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::B0x02)
    }
    ///PD\[1\] pin
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::B0x03)
    }
    ///PF\[1\] pin
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::B0x05)
    }
}
/**EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. Others reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI2 {
    ///0: PA\[2\] pin
    B0x00 = 0,
    ///1: PB\[2\] pin
    B0x01 = 1,
    ///2: PC\[2\] pin
    B0x02 = 2,
    ///3: PD\[2\] pin
    B0x03 = 3,
    ///5: PF\[2\] pin
    B0x05 = 5,
}
impl From<EXTI2> for u8 {
    #[inline(always)]
    fn from(variant: EXTI2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI2 {
    type Ux = u8;
}
impl crate::IsEnum for EXTI2 {}
///Field `EXTI2` reader - EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. Others reserved
pub type EXTI2_R = crate::FieldReader<EXTI2>;
impl EXTI2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI2> {
        match self.bits {
            0 => Some(EXTI2::B0x00),
            1 => Some(EXTI2::B0x01),
            2 => Some(EXTI2::B0x02),
            3 => Some(EXTI2::B0x03),
            5 => Some(EXTI2::B0x05),
            _ => None,
        }
    }
    ///PA\[2\] pin
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == EXTI2::B0x00
    }
    ///PB\[2\] pin
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == EXTI2::B0x01
    }
    ///PC\[2\] pin
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == EXTI2::B0x02
    }
    ///PD\[2\] pin
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == EXTI2::B0x03
    }
    ///PF\[2\] pin
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == EXTI2::B0x05
    }
}
///Field `EXTI2` writer - EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. Others reserved
pub type EXTI2_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI2>;
impl<'a, REG> EXTI2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA\[2\] pin
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::B0x00)
    }
    ///PB\[2\] pin
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::B0x01)
    }
    ///PC\[2\] pin
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::B0x02)
    }
    ///PD\[2\] pin
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::B0x03)
    }
    ///PF\[2\] pin
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::B0x05)
    }
}
/**EXTI3 GPIO port selection These bits are written by software to select the source input for EXTI3 external interrupt. Others reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI3 {
    ///0: PA\[3\] pin
    B0x00 = 0,
    ///1: PB\[3\] pin
    B0x01 = 1,
    ///2: PC\[3\] pin
    B0x02 = 2,
    ///3: PD\[3\] pin
    B0x03 = 3,
    ///5: PF\[3\] pin
    B0x05 = 5,
}
impl From<EXTI3> for u8 {
    #[inline(always)]
    fn from(variant: EXTI3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI3 {
    type Ux = u8;
}
impl crate::IsEnum for EXTI3 {}
///Field `EXTI3` reader - EXTI3 GPIO port selection These bits are written by software to select the source input for EXTI3 external interrupt. Others reserved
pub type EXTI3_R = crate::FieldReader<EXTI3>;
impl EXTI3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI3> {
        match self.bits {
            0 => Some(EXTI3::B0x00),
            1 => Some(EXTI3::B0x01),
            2 => Some(EXTI3::B0x02),
            3 => Some(EXTI3::B0x03),
            5 => Some(EXTI3::B0x05),
            _ => None,
        }
    }
    ///PA\[3\] pin
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == EXTI3::B0x00
    }
    ///PB\[3\] pin
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == EXTI3::B0x01
    }
    ///PC\[3\] pin
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == EXTI3::B0x02
    }
    ///PD\[3\] pin
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == EXTI3::B0x03
    }
    ///PF\[3\] pin
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == EXTI3::B0x05
    }
}
///Field `EXTI3` writer - EXTI3 GPIO port selection These bits are written by software to select the source input for EXTI3 external interrupt. Others reserved
pub type EXTI3_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI3>;
impl<'a, REG> EXTI3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA\[3\] pin
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::B0x00)
    }
    ///PB\[3\] pin
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::B0x01)
    }
    ///PC\[3\] pin
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::B0x02)
    }
    ///PD\[3\] pin
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::B0x03)
    }
    ///PF\[3\] pin
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::B0x05)
    }
}
impl R {
    ///Bits 0:7 - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - EXTI3 GPIO port selection These bits are written by software to select the source input for EXTI3 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI_EXTICR1")
            .field("exti0", &self.exti0())
            .field("exti1", &self.exti1())
            .field("exti2", &self.exti2())
            .field("exti3", &self.exti3())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti0(&mut self) -> EXTI0_W<'_, EXTI_EXTICR1rs> {
        EXTI0_W::new(self, 0)
    }
    ///Bits 8:15 - EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti1(&mut self) -> EXTI1_W<'_, EXTI_EXTICR1rs> {
        EXTI1_W::new(self, 8)
    }
    ///Bits 16:23 - EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti2(&mut self) -> EXTI2_W<'_, EXTI_EXTICR1rs> {
        EXTI2_W::new(self, 16)
    }
    ///Bits 24:31 - EXTI3 GPIO port selection These bits are written by software to select the source input for EXTI3 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti3(&mut self) -> EXTI3_W<'_, EXTI_EXTICR1rs> {
        EXTI3_W::new(self, 24)
    }
}
/**EXTI external interrupt selection register

You can [`read`](crate::Reg::read) this register and get [`exti_exticr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_exticr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#EXTI:EXTI_EXTICR1)*/
pub struct EXTI_EXTICR1rs;
impl crate::RegisterSpec for EXTI_EXTICR1rs {
    type Ux = u32;
}
///`read()` method returns [`exti_exticr1::R`](R) reader structure
impl crate::Readable for EXTI_EXTICR1rs {}
///`write(|w| ..)` method takes [`exti_exticr1::W`](W) writer structure
impl crate::Writable for EXTI_EXTICR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTI_EXTICR1 to value 0
impl crate::Resettable for EXTI_EXTICR1rs {}
