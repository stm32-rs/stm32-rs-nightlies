///Register `EXTICR1` reader
pub type R = crate::R<EXTICR1rs>;
///Register `EXTICR1` writer
pub type W = crate::W<EXTICR1rs>;
/**EXTI 0 configuration bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI0 {
    ///0: PA0 pin
    Pa0 = 0,
    ///1: PB0 pin
    Pb0 = 1,
    ///2: PC0 pin
    Pc0 = 2,
    ///3: PD0 pin
    Pd0 = 3,
    ///4: PE0 pin
    Pe0 = 4,
    ///5: PF0 pin
    Pf0 = 5,
    ///6: PG0 pin
    Pg0 = 6,
    ///7: PH0 pin
    Ph0 = 7,
    ///8: PI0 pin
    Pi0 = 8,
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
///Field `EXTI0` reader - EXTI 0 configuration bits
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
            5 => Some(EXTI0::Pf0),
            6 => Some(EXTI0::Pg0),
            7 => Some(EXTI0::Ph0),
            8 => Some(EXTI0::Pi0),
            _ => None,
        }
    }
    ///PA0 pin
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == EXTI0::Pa0
    }
    ///PB0 pin
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == EXTI0::Pb0
    }
    ///PC0 pin
    #[inline(always)]
    pub fn is_pc0(&self) -> bool {
        *self == EXTI0::Pc0
    }
    ///PD0 pin
    #[inline(always)]
    pub fn is_pd0(&self) -> bool {
        *self == EXTI0::Pd0
    }
    ///PE0 pin
    #[inline(always)]
    pub fn is_pe0(&self) -> bool {
        *self == EXTI0::Pe0
    }
    ///PF0 pin
    #[inline(always)]
    pub fn is_pf0(&self) -> bool {
        *self == EXTI0::Pf0
    }
    ///PG0 pin
    #[inline(always)]
    pub fn is_pg0(&self) -> bool {
        *self == EXTI0::Pg0
    }
    ///PH0 pin
    #[inline(always)]
    pub fn is_ph0(&self) -> bool {
        *self == EXTI0::Ph0
    }
    ///PI0 pin
    #[inline(always)]
    pub fn is_pi0(&self) -> bool {
        *self == EXTI0::Pi0
    }
}
///Field `EXTI0` writer - EXTI 0 configuration bits
pub type EXTI0_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI0>;
impl<'a, REG> EXTI0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA0 pin
    #[inline(always)]
    pub fn pa0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pa0)
    }
    ///PB0 pin
    #[inline(always)]
    pub fn pb0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pb0)
    }
    ///PC0 pin
    #[inline(always)]
    pub fn pc0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pc0)
    }
    ///PD0 pin
    #[inline(always)]
    pub fn pd0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pd0)
    }
    ///PE0 pin
    #[inline(always)]
    pub fn pe0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pe0)
    }
    ///PF0 pin
    #[inline(always)]
    pub fn pf0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pf0)
    }
    ///PG0 pin
    #[inline(always)]
    pub fn pg0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pg0)
    }
    ///PH0 pin
    #[inline(always)]
    pub fn ph0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Ph0)
    }
    ///PI0 pin
    #[inline(always)]
    pub fn pi0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0::Pi0)
    }
}
/**EXTI 1 configuration bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI1 {
    ///0: PA1 pin
    Pa1 = 0,
    ///1: PB1 pin
    Pb1 = 1,
    ///2: PC1 pin
    Pc1 = 2,
    ///3: PD1 pin
    Pd1 = 3,
    ///4: PE1 pin
    Pe1 = 4,
    ///5: PF1 pin
    Pf1 = 5,
    ///6: PG1 pin
    Pg1 = 6,
    ///7: PH1 pin
    Ph1 = 7,
    ///8: PI1 pin
    Pi1 = 8,
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
///Field `EXTI1` reader - EXTI 1 configuration bits
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
            5 => Some(EXTI1::Pf1),
            6 => Some(EXTI1::Pg1),
            7 => Some(EXTI1::Ph1),
            8 => Some(EXTI1::Pi1),
            _ => None,
        }
    }
    ///PA1 pin
    #[inline(always)]
    pub fn is_pa1(&self) -> bool {
        *self == EXTI1::Pa1
    }
    ///PB1 pin
    #[inline(always)]
    pub fn is_pb1(&self) -> bool {
        *self == EXTI1::Pb1
    }
    ///PC1 pin
    #[inline(always)]
    pub fn is_pc1(&self) -> bool {
        *self == EXTI1::Pc1
    }
    ///PD1 pin
    #[inline(always)]
    pub fn is_pd1(&self) -> bool {
        *self == EXTI1::Pd1
    }
    ///PE1 pin
    #[inline(always)]
    pub fn is_pe1(&self) -> bool {
        *self == EXTI1::Pe1
    }
    ///PF1 pin
    #[inline(always)]
    pub fn is_pf1(&self) -> bool {
        *self == EXTI1::Pf1
    }
    ///PG1 pin
    #[inline(always)]
    pub fn is_pg1(&self) -> bool {
        *self == EXTI1::Pg1
    }
    ///PH1 pin
    #[inline(always)]
    pub fn is_ph1(&self) -> bool {
        *self == EXTI1::Ph1
    }
    ///PI1 pin
    #[inline(always)]
    pub fn is_pi1(&self) -> bool {
        *self == EXTI1::Pi1
    }
}
///Field `EXTI1` writer - EXTI 1 configuration bits
pub type EXTI1_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI1>;
impl<'a, REG> EXTI1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA1 pin
    #[inline(always)]
    pub fn pa1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::Pa1)
    }
    ///PB1 pin
    #[inline(always)]
    pub fn pb1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::Pb1)
    }
    ///PC1 pin
    #[inline(always)]
    pub fn pc1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::Pc1)
    }
    ///PD1 pin
    #[inline(always)]
    pub fn pd1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::Pd1)
    }
    ///PE1 pin
    #[inline(always)]
    pub fn pe1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::Pe1)
    }
    ///PF1 pin
    #[inline(always)]
    pub fn pf1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::Pf1)
    }
    ///PG1 pin
    #[inline(always)]
    pub fn pg1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::Pg1)
    }
    ///PH1 pin
    #[inline(always)]
    pub fn ph1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::Ph1)
    }
    ///PI1 pin
    #[inline(always)]
    pub fn pi1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1::Pi1)
    }
}
/**EXTI 2 configuration bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI2 {
    ///0: PA2 pin
    Pa2 = 0,
    ///1: PB2 pin
    Pb2 = 1,
    ///2: PC2 pin
    Pc2 = 2,
    ///3: PD2 pin
    Pd2 = 3,
    ///4: PE2 pin
    Pe2 = 4,
    ///5: PF2 pin
    Pf2 = 5,
    ///6: PG2 pin
    Pg2 = 6,
    ///7: PH2 pin
    Ph2 = 7,
    ///8: PI2 pin
    Pi2 = 8,
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
///Field `EXTI2` reader - EXTI 2 configuration bits
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
            5 => Some(EXTI2::Pf2),
            6 => Some(EXTI2::Pg2),
            7 => Some(EXTI2::Ph2),
            8 => Some(EXTI2::Pi2),
            _ => None,
        }
    }
    ///PA2 pin
    #[inline(always)]
    pub fn is_pa2(&self) -> bool {
        *self == EXTI2::Pa2
    }
    ///PB2 pin
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        *self == EXTI2::Pb2
    }
    ///PC2 pin
    #[inline(always)]
    pub fn is_pc2(&self) -> bool {
        *self == EXTI2::Pc2
    }
    ///PD2 pin
    #[inline(always)]
    pub fn is_pd2(&self) -> bool {
        *self == EXTI2::Pd2
    }
    ///PE2 pin
    #[inline(always)]
    pub fn is_pe2(&self) -> bool {
        *self == EXTI2::Pe2
    }
    ///PF2 pin
    #[inline(always)]
    pub fn is_pf2(&self) -> bool {
        *self == EXTI2::Pf2
    }
    ///PG2 pin
    #[inline(always)]
    pub fn is_pg2(&self) -> bool {
        *self == EXTI2::Pg2
    }
    ///PH2 pin
    #[inline(always)]
    pub fn is_ph2(&self) -> bool {
        *self == EXTI2::Ph2
    }
    ///PI2 pin
    #[inline(always)]
    pub fn is_pi2(&self) -> bool {
        *self == EXTI2::Pi2
    }
}
///Field `EXTI2` writer - EXTI 2 configuration bits
pub type EXTI2_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI2>;
impl<'a, REG> EXTI2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA2 pin
    #[inline(always)]
    pub fn pa2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::Pa2)
    }
    ///PB2 pin
    #[inline(always)]
    pub fn pb2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::Pb2)
    }
    ///PC2 pin
    #[inline(always)]
    pub fn pc2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::Pc2)
    }
    ///PD2 pin
    #[inline(always)]
    pub fn pd2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::Pd2)
    }
    ///PE2 pin
    #[inline(always)]
    pub fn pe2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::Pe2)
    }
    ///PF2 pin
    #[inline(always)]
    pub fn pf2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::Pf2)
    }
    ///PG2 pin
    #[inline(always)]
    pub fn pg2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::Pg2)
    }
    ///PH2 pin
    #[inline(always)]
    pub fn ph2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::Ph2)
    }
    ///PI2 pin
    #[inline(always)]
    pub fn pi2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2::Pi2)
    }
}
/**EXTI 3 configuration bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI3 {
    ///0: PA3 pin
    Pa3 = 0,
    ///1: PB3 pin
    Pb3 = 1,
    ///2: PC3 pin
    Pc3 = 2,
    ///3: PD3 pin
    Pd3 = 3,
    ///4: PE3 pin
    Pe3 = 4,
    ///5: PF3 pin
    Pf3 = 5,
    ///6: PG3 pin
    Pg3 = 6,
    ///7: PH3 pin
    Ph3 = 7,
    ///8: PI3 pin
    Pi3 = 8,
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
///Field `EXTI3` reader - EXTI 3 configuration bits
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
            5 => Some(EXTI3::Pf3),
            6 => Some(EXTI3::Pg3),
            7 => Some(EXTI3::Ph3),
            8 => Some(EXTI3::Pi3),
            _ => None,
        }
    }
    ///PA3 pin
    #[inline(always)]
    pub fn is_pa3(&self) -> bool {
        *self == EXTI3::Pa3
    }
    ///PB3 pin
    #[inline(always)]
    pub fn is_pb3(&self) -> bool {
        *self == EXTI3::Pb3
    }
    ///PC3 pin
    #[inline(always)]
    pub fn is_pc3(&self) -> bool {
        *self == EXTI3::Pc3
    }
    ///PD3 pin
    #[inline(always)]
    pub fn is_pd3(&self) -> bool {
        *self == EXTI3::Pd3
    }
    ///PE3 pin
    #[inline(always)]
    pub fn is_pe3(&self) -> bool {
        *self == EXTI3::Pe3
    }
    ///PF3 pin
    #[inline(always)]
    pub fn is_pf3(&self) -> bool {
        *self == EXTI3::Pf3
    }
    ///PG3 pin
    #[inline(always)]
    pub fn is_pg3(&self) -> bool {
        *self == EXTI3::Pg3
    }
    ///PH3 pin
    #[inline(always)]
    pub fn is_ph3(&self) -> bool {
        *self == EXTI3::Ph3
    }
    ///PI3 pin
    #[inline(always)]
    pub fn is_pi3(&self) -> bool {
        *self == EXTI3::Pi3
    }
}
///Field `EXTI3` writer - EXTI 3 configuration bits
pub type EXTI3_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI3>;
impl<'a, REG> EXTI3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA3 pin
    #[inline(always)]
    pub fn pa3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::Pa3)
    }
    ///PB3 pin
    #[inline(always)]
    pub fn pb3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::Pb3)
    }
    ///PC3 pin
    #[inline(always)]
    pub fn pc3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::Pc3)
    }
    ///PD3 pin
    #[inline(always)]
    pub fn pd3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::Pd3)
    }
    ///PE3 pin
    #[inline(always)]
    pub fn pe3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::Pe3)
    }
    ///PF3 pin
    #[inline(always)]
    pub fn pf3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::Pf3)
    }
    ///PG3 pin
    #[inline(always)]
    pub fn pg3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::Pg3)
    }
    ///PH3 pin
    #[inline(always)]
    pub fn ph3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::Ph3)
    }
    ///PI3 pin
    #[inline(always)]
    pub fn pi3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3::Pi3)
    }
}
impl R {
    ///Bits 0:3 - EXTI 0 configuration bits
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - EXTI 1 configuration bits
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - EXTI 2 configuration bits
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - EXTI 3 configuration bits
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
    ///Bits 0:3 - EXTI 0 configuration bits
    #[inline(always)]
    pub fn exti0(&mut self) -> EXTI0_W<EXTICR1rs> {
        EXTI0_W::new(self, 0)
    }
    ///Bits 4:7 - EXTI 1 configuration bits
    #[inline(always)]
    pub fn exti1(&mut self) -> EXTI1_W<EXTICR1rs> {
        EXTI1_W::new(self, 4)
    }
    ///Bits 8:11 - EXTI 2 configuration bits
    #[inline(always)]
    pub fn exti2(&mut self) -> EXTI2_W<EXTICR1rs> {
        EXTI2_W::new(self, 8)
    }
    ///Bits 12:15 - EXTI 3 configuration bits
    #[inline(always)]
    pub fn exti3(&mut self) -> EXTI3_W<EXTICR1rs> {
        EXTI3_W::new(self, 12)
    }
}
/**external interrupt configuration register 1

You can [`read`](crate::Reg::read) this register and get [`exticr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#SYSCFG:EXTICR1)*/
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
