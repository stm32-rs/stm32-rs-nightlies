///Register `EXTICR1` reader
pub type R = crate::R<EXTICR1rs>;
///Register `EXTICR1` writer
pub type W = crate::W<EXTICR1rs>;
/**EXTI 0 configuration bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI_ABCDEFGH {
    ///0: Select PAx as the source input for the EXTIx external interrupt
    Pa = 0,
    ///1: Select PBx as the source input for the EXTIx external interrupt
    Pb = 1,
    ///2: Select PCx as the source input for the EXTIx external interrupt
    Pc = 2,
    ///3: Select PDx as the source input for the EXTIx external interrupt
    Pd = 3,
    ///4: Select PEx as the source input for the EXTIx external interrupt
    Pe = 4,
    ///5: Select PFx as the source input for the EXTIx external interrupt
    Pf = 5,
    ///6: Select PGx as the source input for the EXTIx external interrupt
    Pg = 6,
    ///7: Select PHx as the source input for the EXTIx external interrupt
    Ph = 7,
    ///8: Select PIx as the source input for the EXTIx external interrupt
    Pi = 8,
}
impl From<EXTI_ABCDEFGH> for u8 {
    #[inline(always)]
    fn from(variant: EXTI_ABCDEFGH) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI_ABCDEFGH {
    type Ux = u8;
}
impl crate::IsEnum for EXTI_ABCDEFGH {}
///Field `EXTI0` reader - EXTI 0 configuration bits
pub type EXTI0_R = crate::FieldReader<EXTI_ABCDEFGH>;
impl EXTI0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI_ABCDEFGH> {
        match self.bits {
            0 => Some(EXTI_ABCDEFGH::Pa),
            1 => Some(EXTI_ABCDEFGH::Pb),
            2 => Some(EXTI_ABCDEFGH::Pc),
            3 => Some(EXTI_ABCDEFGH::Pd),
            4 => Some(EXTI_ABCDEFGH::Pe),
            5 => Some(EXTI_ABCDEFGH::Pf),
            6 => Some(EXTI_ABCDEFGH::Pg),
            7 => Some(EXTI_ABCDEFGH::Ph),
            8 => Some(EXTI_ABCDEFGH::Pi),
            _ => None,
        }
    }
    ///Select PAx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pa(&self) -> bool {
        *self == EXTI_ABCDEFGH::Pa
    }
    ///Select PBx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pb(&self) -> bool {
        *self == EXTI_ABCDEFGH::Pb
    }
    ///Select PCx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pc(&self) -> bool {
        *self == EXTI_ABCDEFGH::Pc
    }
    ///Select PDx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == EXTI_ABCDEFGH::Pd
    }
    ///Select PEx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pe(&self) -> bool {
        *self == EXTI_ABCDEFGH::Pe
    }
    ///Select PFx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pf(&self) -> bool {
        *self == EXTI_ABCDEFGH::Pf
    }
    ///Select PGx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pg(&self) -> bool {
        *self == EXTI_ABCDEFGH::Pg
    }
    ///Select PHx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_ph(&self) -> bool {
        *self == EXTI_ABCDEFGH::Ph
    }
    ///Select PIx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn is_pi(&self) -> bool {
        *self == EXTI_ABCDEFGH::Pi
    }
}
///Field `EXTI0` writer - EXTI 0 configuration bits
pub type EXTI0_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXTI_ABCDEFGH>;
impl<'a, REG> EXTI0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select PAx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pa(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_ABCDEFGH::Pa)
    }
    ///Select PBx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_ABCDEFGH::Pb)
    }
    ///Select PCx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_ABCDEFGH::Pc)
    }
    ///Select PDx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_ABCDEFGH::Pd)
    }
    ///Select PEx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pe(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_ABCDEFGH::Pe)
    }
    ///Select PFx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_ABCDEFGH::Pf)
    }
    ///Select PGx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pg(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_ABCDEFGH::Pg)
    }
    ///Select PHx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn ph(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_ABCDEFGH::Ph)
    }
    ///Select PIx as the source input for the EXTIx external interrupt
    #[inline(always)]
    pub fn pi(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI_ABCDEFGH::Pi)
    }
}
///Field `EXTI1` reader - EXTI 1 configuration bits
pub use EXTI0_R as EXTI1_R;
///Field `EXTI2` reader - EXTI 2 configuration bits
pub use EXTI0_R as EXTI2_R;
///Field `EXTI3` reader - EXTI 3 configuration bits
pub use EXTI0_R as EXTI3_R;
///Field `EXTI1` writer - EXTI 1 configuration bits
pub use EXTI0_W as EXTI1_W;
///Field `EXTI2` writer - EXTI 2 configuration bits
pub use EXTI0_W as EXTI2_W;
///Field `EXTI3` writer - EXTI 3 configuration bits
pub use EXTI0_W as EXTI3_W;
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#SYSCFG:EXTICR1)*/
pub struct EXTICR1rs;
impl crate::RegisterSpec for EXTICR1rs {
    type Ux = u32;
}
///`read()` method returns [`exticr1::R`](R) reader structure
impl crate::Readable for EXTICR1rs {}
///`write(|w| ..)` method takes [`exticr1::W`](W) writer structure
impl crate::Writable for EXTICR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTICR1 to value 0
impl crate::Resettable for EXTICR1rs {}
