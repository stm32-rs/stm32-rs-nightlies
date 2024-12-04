///Register `EXTICR1` reader
pub type R = crate::R<EXTICR1rs>;
///Register `EXTICR1` writer
pub type W = crate::W<EXTICR1rs>;
/**EXTI x configuration (x = 0 to 3)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI0 {
    ///0: Select PA0 as the source input for the EXTI0 external interrupt
    Pa0 = 0,
    ///1: Select PB0 as the source input for the EXTI0 external interrupt
    Pb0 = 1,
    ///2: Select PC0 as the source input for the EXTI0 external interrupt
    Pc0 = 2,
    ///3: Select PD0 as the source input for the EXTI0 external interrupt
    Pd0 = 3,
    ///4: Select PE0 as the source input for the EXTI0 external interrupt
    Pe0 = 4,
    ///5: Select PH0 as the source input for the EXTI0 external interrupt
    Ph0 = 5,
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
///Field `EXTI0` reader - EXTI x configuration (x = 0 to 3)
pub type EXTI0_R = crate::FieldReader<EXTI0>;
impl EXTI0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI0> {
        match self.bits {
            0 => Some(EXTI0::Pa0),
            1 => Some(EXTI0::Pb0),
            2 => Some(EXTI0::Pc0),
            3 => Some(EXTI0::Pd0),
            4 => Some(EXTI0::Pe0),
            5 => Some(EXTI0::Ph0),
            _ => None,
        }
    }
    ///Select PA0 as the source input for the EXTI0 external interrupt
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == EXTI0::Pa0
    }
    ///Select PB0 as the source input for the EXTI0 external interrupt
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == EXTI0::Pb0
    }
    ///Select PC0 as the source input for the EXTI0 external interrupt
    #[inline(always)]
    pub fn is_pc0(&self) -> bool {
        *self == EXTI0::Pc0
    }
    ///Select PD0 as the source input for the EXTI0 external interrupt
    #[inline(always)]
    pub fn is_pd0(&self) -> bool {
        *self == EXTI0::Pd0
    }
    ///Select PE0 as the source input for the EXTI0 external interrupt
    #[inline(always)]
    pub fn is_pe0(&self) -> bool {
        *self == EXTI0::Pe0
    }
    ///Select PH0 as the source input for the EXTI0 external interrupt
    #[inline(always)]
    pub fn is_ph0(&self) -> bool {
        *self == EXTI0::Ph0
    }
}
///Field `EXTI0` writer - EXTI x configuration (x = 0 to 3)
pub type EXTI0_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI0>;
impl<'a, REG> EXTI0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select PA0 as the source input for the EXTI0 external interrupt
    #[inline(always)]
    pub fn pa0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pa0)
    }
    ///Select PB0 as the source input for the EXTI0 external interrupt
    #[inline(always)]
    pub fn pb0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pb0)
    }
    ///Select PC0 as the source input for the EXTI0 external interrupt
    #[inline(always)]
    pub fn pc0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pc0)
    }
    ///Select PD0 as the source input for the EXTI0 external interrupt
    #[inline(always)]
    pub fn pd0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pd0)
    }
    ///Select PE0 as the source input for the EXTI0 external interrupt
    #[inline(always)]
    pub fn pe0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pe0)
    }
    ///Select PH0 as the source input for the EXTI0 external interrupt
    #[inline(always)]
    pub fn ph0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Ph0)
    }
}
/**EXTI x configuration (x = 0 to 3)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI1 {
    ///0: Select PA1 as the source input for the EXTI1 external interrupt
    Pa1 = 0,
    ///1: Select PB1 as the source input for the EXTI1 external interrupt
    Pb1 = 1,
    ///2: Select PC1 as the source input for the EXTI1 external interrupt
    Pc1 = 2,
    ///3: Select PD1 as the source input for the EXTI1 external interrupt
    Pd1 = 3,
    ///4: Select PE1 as the source input for the EXTI1 external interrupt
    Pe1 = 4,
    ///5: Select PH1 as the source input for the EXTI1 external interrupt
    Ph1 = 5,
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
///Field `EXTI1` reader - EXTI x configuration (x = 0 to 3)
pub type EXTI1_R = crate::FieldReader<EXTI1>;
impl EXTI1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI1> {
        match self.bits {
            0 => Some(EXTI1::Pa1),
            1 => Some(EXTI1::Pb1),
            2 => Some(EXTI1::Pc1),
            3 => Some(EXTI1::Pd1),
            4 => Some(EXTI1::Pe1),
            5 => Some(EXTI1::Ph1),
            _ => None,
        }
    }
    ///Select PA1 as the source input for the EXTI1 external interrupt
    #[inline(always)]
    pub fn is_pa1(&self) -> bool {
        *self == EXTI1::Pa1
    }
    ///Select PB1 as the source input for the EXTI1 external interrupt
    #[inline(always)]
    pub fn is_pb1(&self) -> bool {
        *self == EXTI1::Pb1
    }
    ///Select PC1 as the source input for the EXTI1 external interrupt
    #[inline(always)]
    pub fn is_pc1(&self) -> bool {
        *self == EXTI1::Pc1
    }
    ///Select PD1 as the source input for the EXTI1 external interrupt
    #[inline(always)]
    pub fn is_pd1(&self) -> bool {
        *self == EXTI1::Pd1
    }
    ///Select PE1 as the source input for the EXTI1 external interrupt
    #[inline(always)]
    pub fn is_pe1(&self) -> bool {
        *self == EXTI1::Pe1
    }
    ///Select PH1 as the source input for the EXTI1 external interrupt
    #[inline(always)]
    pub fn is_ph1(&self) -> bool {
        *self == EXTI1::Ph1
    }
}
///Field `EXTI1` writer - EXTI x configuration (x = 0 to 3)
pub type EXTI1_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI1>;
impl<'a, REG> EXTI1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select PA1 as the source input for the EXTI1 external interrupt
    #[inline(always)]
    pub fn pa1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::Pa1)
    }
    ///Select PB1 as the source input for the EXTI1 external interrupt
    #[inline(always)]
    pub fn pb1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::Pb1)
    }
    ///Select PC1 as the source input for the EXTI1 external interrupt
    #[inline(always)]
    pub fn pc1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::Pc1)
    }
    ///Select PD1 as the source input for the EXTI1 external interrupt
    #[inline(always)]
    pub fn pd1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::Pd1)
    }
    ///Select PE1 as the source input for the EXTI1 external interrupt
    #[inline(always)]
    pub fn pe1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::Pe1)
    }
    ///Select PH1 as the source input for the EXTI1 external interrupt
    #[inline(always)]
    pub fn ph1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::Ph1)
    }
}
/**EXTI x configuration (x = 0 to 3)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI2 {
    ///0: Select PA2 as the source input for the EXTI2 external interrupt
    Pa2 = 0,
    ///1: Select PB2 as the source input for the EXTI2 external interrupt
    Pb2 = 1,
    ///2: Select PC2 as the source input for the EXTI2 external interrupt
    Pc2 = 2,
    ///3: Select PD2 as the source input for the EXTI2 external interrupt
    Pd2 = 3,
    ///4: Select PE2 as the source input for the EXTI2 external interrupt
    Pe2 = 4,
    ///5: Select PH2 as the source input for the EXTI2 external interrupt
    Ph2 = 5,
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
///Field `EXTI2` reader - EXTI x configuration (x = 0 to 3)
pub type EXTI2_R = crate::FieldReader<EXTI2>;
impl EXTI2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI2> {
        match self.bits {
            0 => Some(EXTI2::Pa2),
            1 => Some(EXTI2::Pb2),
            2 => Some(EXTI2::Pc2),
            3 => Some(EXTI2::Pd2),
            4 => Some(EXTI2::Pe2),
            5 => Some(EXTI2::Ph2),
            _ => None,
        }
    }
    ///Select PA2 as the source input for the EXTI2 external interrupt
    #[inline(always)]
    pub fn is_pa2(&self) -> bool {
        *self == EXTI2::Pa2
    }
    ///Select PB2 as the source input for the EXTI2 external interrupt
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        *self == EXTI2::Pb2
    }
    ///Select PC2 as the source input for the EXTI2 external interrupt
    #[inline(always)]
    pub fn is_pc2(&self) -> bool {
        *self == EXTI2::Pc2
    }
    ///Select PD2 as the source input for the EXTI2 external interrupt
    #[inline(always)]
    pub fn is_pd2(&self) -> bool {
        *self == EXTI2::Pd2
    }
    ///Select PE2 as the source input for the EXTI2 external interrupt
    #[inline(always)]
    pub fn is_pe2(&self) -> bool {
        *self == EXTI2::Pe2
    }
    ///Select PH2 as the source input for the EXTI2 external interrupt
    #[inline(always)]
    pub fn is_ph2(&self) -> bool {
        *self == EXTI2::Ph2
    }
}
///Field `EXTI2` writer - EXTI x configuration (x = 0 to 3)
pub type EXTI2_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI2>;
impl<'a, REG> EXTI2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select PA2 as the source input for the EXTI2 external interrupt
    #[inline(always)]
    pub fn pa2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::Pa2)
    }
    ///Select PB2 as the source input for the EXTI2 external interrupt
    #[inline(always)]
    pub fn pb2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::Pb2)
    }
    ///Select PC2 as the source input for the EXTI2 external interrupt
    #[inline(always)]
    pub fn pc2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::Pc2)
    }
    ///Select PD2 as the source input for the EXTI2 external interrupt
    #[inline(always)]
    pub fn pd2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::Pd2)
    }
    ///Select PE2 as the source input for the EXTI2 external interrupt
    #[inline(always)]
    pub fn pe2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::Pe2)
    }
    ///Select PH2 as the source input for the EXTI2 external interrupt
    #[inline(always)]
    pub fn ph2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::Ph2)
    }
}
/**EXTI x configuration (x = 0 to 3)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI3 {
    ///0: Select PA3 as the source input for the EXTI3 external interrupt
    Pa3 = 0,
    ///1: Select PB3 as the source input for the EXTI3 external interrupt
    Pb3 = 1,
    ///2: Select PC3 as the source input for the EXTI3 external interrupt
    Pc3 = 2,
    ///3: Select PD3 as the source input for the EXTI3 external interrupt
    Pd3 = 3,
    ///4: Select PE3 as the source input for the EXTI3 external interrupt
    Pe3 = 4,
    ///5: Select PH3 as the source input for the EXTI3 external interrupt
    Ph3 = 5,
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
///Field `EXTI3` reader - EXTI x configuration (x = 0 to 3)
pub type EXTI3_R = crate::FieldReader<EXTI3>;
impl EXTI3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI3> {
        match self.bits {
            0 => Some(EXTI3::Pa3),
            1 => Some(EXTI3::Pb3),
            2 => Some(EXTI3::Pc3),
            3 => Some(EXTI3::Pd3),
            4 => Some(EXTI3::Pe3),
            5 => Some(EXTI3::Ph3),
            _ => None,
        }
    }
    ///Select PA3 as the source input for the EXTI3 external interrupt
    #[inline(always)]
    pub fn is_pa3(&self) -> bool {
        *self == EXTI3::Pa3
    }
    ///Select PB3 as the source input for the EXTI3 external interrupt
    #[inline(always)]
    pub fn is_pb3(&self) -> bool {
        *self == EXTI3::Pb3
    }
    ///Select PC3 as the source input for the EXTI3 external interrupt
    #[inline(always)]
    pub fn is_pc3(&self) -> bool {
        *self == EXTI3::Pc3
    }
    ///Select PD3 as the source input for the EXTI3 external interrupt
    #[inline(always)]
    pub fn is_pd3(&self) -> bool {
        *self == EXTI3::Pd3
    }
    ///Select PE3 as the source input for the EXTI3 external interrupt
    #[inline(always)]
    pub fn is_pe3(&self) -> bool {
        *self == EXTI3::Pe3
    }
    ///Select PH3 as the source input for the EXTI3 external interrupt
    #[inline(always)]
    pub fn is_ph3(&self) -> bool {
        *self == EXTI3::Ph3
    }
}
///Field `EXTI3` writer - EXTI x configuration (x = 0 to 3)
pub type EXTI3_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI3>;
impl<'a, REG> EXTI3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select PA3 as the source input for the EXTI3 external interrupt
    #[inline(always)]
    pub fn pa3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::Pa3)
    }
    ///Select PB3 as the source input for the EXTI3 external interrupt
    #[inline(always)]
    pub fn pb3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::Pb3)
    }
    ///Select PC3 as the source input for the EXTI3 external interrupt
    #[inline(always)]
    pub fn pc3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::Pc3)
    }
    ///Select PD3 as the source input for the EXTI3 external interrupt
    #[inline(always)]
    pub fn pd3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::Pd3)
    }
    ///Select PE3 as the source input for the EXTI3 external interrupt
    #[inline(always)]
    pub fn pe3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::Pe3)
    }
    ///Select PH3 as the source input for the EXTI3 external interrupt
    #[inline(always)]
    pub fn ph3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::Ph3)
    }
}
impl R {
    ///Bits 0:3 - EXTI x configuration (x = 0 to 3)
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - EXTI x configuration (x = 0 to 3)
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - EXTI x configuration (x = 0 to 3)
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - EXTI x configuration (x = 0 to 3)
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTICR1")
            .field("exti3", &self.exti3())
            .field("exti2", &self.exti2())
            .field("exti1", &self.exti1())
            .field("exti0", &self.exti0())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - EXTI x configuration (x = 0 to 3)
    #[inline(always)]
    pub fn exti0(&mut self) -> EXTI0_W<EXTICR1rs> {
        EXTI0_W::new(self, 0)
    }
    ///Bits 4:7 - EXTI x configuration (x = 0 to 3)
    #[inline(always)]
    pub fn exti1(&mut self) -> EXTI1_W<EXTICR1rs> {
        EXTI1_W::new(self, 4)
    }
    ///Bits 8:11 - EXTI x configuration (x = 0 to 3)
    #[inline(always)]
    pub fn exti2(&mut self) -> EXTI2_W<EXTICR1rs> {
        EXTI2_W::new(self, 8)
    }
    ///Bits 12:15 - EXTI x configuration (x = 0 to 3)
    #[inline(always)]
    pub fn exti3(&mut self) -> EXTI3_W<EXTICR1rs> {
        EXTI3_W::new(self, 12)
    }
}
/**external interrupt configuration register 1

You can [`read`](crate::Reg::read) this register and get [`exticr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#SYSCFG:EXTICR1)*/
pub struct EXTICR1rs;
impl crate::RegisterSpec for EXTICR1rs {
    type Ux = u32;
}
///`read()` method returns [`exticr1::R`](R) reader structure
impl crate::Readable for EXTICR1rs {}
///`write(|w| ..)` method takes [`exticr1::W`](W) writer structure
impl crate::Writable for EXTICR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EXTICR1 to value 0
impl crate::Resettable for EXTICR1rs {
    const RESET_VALUE: u32 = 0;
}
