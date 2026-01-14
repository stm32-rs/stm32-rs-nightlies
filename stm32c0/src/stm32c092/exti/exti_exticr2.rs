///Register `EXTI_EXTICR2` reader
pub type R = crate::R<EXTI_EXTICR2rs>;
///Register `EXTI_EXTICR2` writer
pub type W = crate::W<EXTI_EXTICR2rs>;
/**EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. Others reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI4 {
    ///0: PA\[4\] pin
    B0x00 = 0,
    ///1: PB\[4\] pin
    B0x01 = 1,
    ///2: PC\[4\] pin
    B0x02 = 2,
    ///3: PD\[4\] pin
    B0x03 = 3,
    ///5: PF\[4\] pin
    B0x05 = 5,
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
impl crate::IsEnum for EXTI4 {}
///Field `EXTI4` reader - EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. Others reserved
pub type EXTI4_R = crate::FieldReader<EXTI4>;
impl EXTI4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI4> {
        match self.bits {
            0 => Some(EXTI4::B0x00),
            1 => Some(EXTI4::B0x01),
            2 => Some(EXTI4::B0x02),
            3 => Some(EXTI4::B0x03),
            5 => Some(EXTI4::B0x05),
            _ => None,
        }
    }
    ///PA\[4\] pin
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == EXTI4::B0x00
    }
    ///PB\[4\] pin
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == EXTI4::B0x01
    }
    ///PC\[4\] pin
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == EXTI4::B0x02
    }
    ///PD\[4\] pin
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == EXTI4::B0x03
    }
    ///PF\[4\] pin
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == EXTI4::B0x05
    }
}
///Field `EXTI4` writer - EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. Others reserved
pub type EXTI4_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI4>;
impl<'a, REG> EXTI4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA\[4\] pin
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::B0x00)
    }
    ///PB\[4\] pin
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::B0x01)
    }
    ///PC\[4\] pin
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::B0x02)
    }
    ///PD\[4\] pin
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::B0x03)
    }
    ///PF\[4\] pin
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::B0x05)
    }
}
/**EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. Others reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI5 {
    ///0: PA\[5\] pin
    B0x00 = 0,
    ///1: PB\[5\] pin
    B0x01 = 1,
    ///2: PC\[5\] pin
    B0x02 = 2,
    ///3: PD\[5\] pin
    B0x03 = 3,
    ///5: PF\[5\] pin
    B0x05 = 5,
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
impl crate::IsEnum for EXTI5 {}
///Field `EXTI5` reader - EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. Others reserved
pub type EXTI5_R = crate::FieldReader<EXTI5>;
impl EXTI5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI5> {
        match self.bits {
            0 => Some(EXTI5::B0x00),
            1 => Some(EXTI5::B0x01),
            2 => Some(EXTI5::B0x02),
            3 => Some(EXTI5::B0x03),
            5 => Some(EXTI5::B0x05),
            _ => None,
        }
    }
    ///PA\[5\] pin
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == EXTI5::B0x00
    }
    ///PB\[5\] pin
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == EXTI5::B0x01
    }
    ///PC\[5\] pin
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == EXTI5::B0x02
    }
    ///PD\[5\] pin
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == EXTI5::B0x03
    }
    ///PF\[5\] pin
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == EXTI5::B0x05
    }
}
///Field `EXTI5` writer - EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. Others reserved
pub type EXTI5_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI5>;
impl<'a, REG> EXTI5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA\[5\] pin
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5::B0x00)
    }
    ///PB\[5\] pin
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5::B0x01)
    }
    ///PC\[5\] pin
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5::B0x02)
    }
    ///PD\[5\] pin
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5::B0x03)
    }
    ///PF\[5\] pin
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5::B0x05)
    }
}
/**EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. Others reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI6 {
    ///0: PA\[6\] pin
    B0x00 = 0,
    ///1: PB\[6\] pin
    B0x01 = 1,
    ///2: PC\[6\] pin
    B0x02 = 2,
    ///3: PD\[6\] pin
    B0x03 = 3,
    ///5: PF\[6\] pin
    B0x05 = 5,
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
impl crate::IsEnum for EXTI6 {}
///Field `EXTI6` reader - EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. Others reserved
pub type EXTI6_R = crate::FieldReader<EXTI6>;
impl EXTI6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI6> {
        match self.bits {
            0 => Some(EXTI6::B0x00),
            1 => Some(EXTI6::B0x01),
            2 => Some(EXTI6::B0x02),
            3 => Some(EXTI6::B0x03),
            5 => Some(EXTI6::B0x05),
            _ => None,
        }
    }
    ///PA\[6\] pin
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == EXTI6::B0x00
    }
    ///PB\[6\] pin
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == EXTI6::B0x01
    }
    ///PC\[6\] pin
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == EXTI6::B0x02
    }
    ///PD\[6\] pin
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == EXTI6::B0x03
    }
    ///PF\[6\] pin
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == EXTI6::B0x05
    }
}
///Field `EXTI6` writer - EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. Others reserved
pub type EXTI6_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI6>;
impl<'a, REG> EXTI6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA\[6\] pin
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6::B0x00)
    }
    ///PB\[6\] pin
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6::B0x01)
    }
    ///PC\[6\] pin
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6::B0x02)
    }
    ///PD\[6\] pin
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6::B0x03)
    }
    ///PF\[6\] pin
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6::B0x05)
    }
}
/**EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. Others reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI7 {
    ///0: PA\[7\] pin
    B0x00 = 0,
    ///1: PB\[7\] pin
    B0x01 = 1,
    ///2: PC\[7\] pin
    B0x02 = 2,
    ///3: PD\[7\] pin
    B0x03 = 3,
    ///5: PF\[7\] pin
    B0x05 = 5,
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
impl crate::IsEnum for EXTI7 {}
///Field `EXTI7` reader - EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. Others reserved
pub type EXTI7_R = crate::FieldReader<EXTI7>;
impl EXTI7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI7> {
        match self.bits {
            0 => Some(EXTI7::B0x00),
            1 => Some(EXTI7::B0x01),
            2 => Some(EXTI7::B0x02),
            3 => Some(EXTI7::B0x03),
            5 => Some(EXTI7::B0x05),
            _ => None,
        }
    }
    ///PA\[7\] pin
    #[inline(always)]
    pub fn is_b_0x00(&self) -> bool {
        *self == EXTI7::B0x00
    }
    ///PB\[7\] pin
    #[inline(always)]
    pub fn is_b_0x01(&self) -> bool {
        *self == EXTI7::B0x01
    }
    ///PC\[7\] pin
    #[inline(always)]
    pub fn is_b_0x02(&self) -> bool {
        *self == EXTI7::B0x02
    }
    ///PD\[7\] pin
    #[inline(always)]
    pub fn is_b_0x03(&self) -> bool {
        *self == EXTI7::B0x03
    }
    ///PF\[7\] pin
    #[inline(always)]
    pub fn is_b_0x05(&self) -> bool {
        *self == EXTI7::B0x05
    }
}
///Field `EXTI7` writer - EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. Others reserved
pub type EXTI7_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI7>;
impl<'a, REG> EXTI7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA\[7\] pin
    #[inline(always)]
    pub fn b_0x00(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7::B0x00)
    }
    ///PB\[7\] pin
    #[inline(always)]
    pub fn b_0x01(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7::B0x01)
    }
    ///PC\[7\] pin
    #[inline(always)]
    pub fn b_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7::B0x02)
    }
    ///PD\[7\] pin
    #[inline(always)]
    pub fn b_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7::B0x03)
    }
    ///PF\[7\] pin
    #[inline(always)]
    pub fn b_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7::B0x05)
    }
}
impl R {
    ///Bits 0:7 - EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI_EXTICR2")
            .field("exti4", &self.exti4())
            .field("exti5", &self.exti5())
            .field("exti6", &self.exti6())
            .field("exti7", &self.exti7())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti4(&mut self) -> EXTI4_W<'_, EXTI_EXTICR2rs> {
        EXTI4_W::new(self, 0)
    }
    ///Bits 8:15 - EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti5(&mut self) -> EXTI5_W<'_, EXTI_EXTICR2rs> {
        EXTI5_W::new(self, 8)
    }
    ///Bits 16:23 - EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti6(&mut self) -> EXTI6_W<'_, EXTI_EXTICR2rs> {
        EXTI6_W::new(self, 16)
    }
    ///Bits 24:31 - EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. Others reserved
    #[inline(always)]
    pub fn exti7(&mut self) -> EXTI7_W<'_, EXTI_EXTICR2rs> {
        EXTI7_W::new(self, 24)
    }
}
/**EXTI external interrupt selection register

You can [`read`](crate::Reg::read) this register and get [`exti_exticr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_exticr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#EXTI:EXTI_EXTICR2)*/
pub struct EXTI_EXTICR2rs;
impl crate::RegisterSpec for EXTI_EXTICR2rs {
    type Ux = u32;
}
///`read()` method returns [`exti_exticr2::R`](R) reader structure
impl crate::Readable for EXTI_EXTICR2rs {}
///`write(|w| ..)` method takes [`exti_exticr2::W`](W) writer structure
impl crate::Writable for EXTI_EXTICR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTI_EXTICR2 to value 0
impl crate::Resettable for EXTI_EXTICR2rs {}
