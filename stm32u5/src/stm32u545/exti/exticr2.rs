///Register `EXTICR2` reader
pub type R = crate::R<EXTICR2rs>;
///Register `EXTICR2` writer
pub type W = crate::W<EXTICR2rs>;
/**EXTIm GPIO port selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI4 {
    ///0: PAx pin
    Pa = 0,
    ///1: PBx pin
    Pb = 1,
    ///2: PCx pin
    Pc = 2,
    ///3: PDx pin
    Pd = 3,
    ///4: PEx pin
    Pe = 4,
    ///5: PFx pin
    Pf = 5,
    ///6: PGx pin
    Pg = 6,
    ///7: PHx pin
    Ph = 7,
    ///8: PIx pin
    Pi = 8,
    ///9: PJx pin
    Pj = 9,
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
///Field `EXTI4` reader - EXTIm GPIO port selection
pub type EXTI4_R = crate::FieldReader<EXTI4>;
impl EXTI4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI4> {
        match self.bits {
            0 => Some(EXTI4::Pa),
            1 => Some(EXTI4::Pb),
            2 => Some(EXTI4::Pc),
            3 => Some(EXTI4::Pd),
            4 => Some(EXTI4::Pe),
            5 => Some(EXTI4::Pf),
            6 => Some(EXTI4::Pg),
            7 => Some(EXTI4::Ph),
            8 => Some(EXTI4::Pi),
            9 => Some(EXTI4::Pj),
            _ => None,
        }
    }
    ///PAx pin
    #[inline(always)]
    pub fn is_pa(&self) -> bool {
        *self == EXTI4::Pa
    }
    ///PBx pin
    #[inline(always)]
    pub fn is_pb(&self) -> bool {
        *self == EXTI4::Pb
    }
    ///PCx pin
    #[inline(always)]
    pub fn is_pc(&self) -> bool {
        *self == EXTI4::Pc
    }
    ///PDx pin
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == EXTI4::Pd
    }
    ///PEx pin
    #[inline(always)]
    pub fn is_pe(&self) -> bool {
        *self == EXTI4::Pe
    }
    ///PFx pin
    #[inline(always)]
    pub fn is_pf(&self) -> bool {
        *self == EXTI4::Pf
    }
    ///PGx pin
    #[inline(always)]
    pub fn is_pg(&self) -> bool {
        *self == EXTI4::Pg
    }
    ///PHx pin
    #[inline(always)]
    pub fn is_ph(&self) -> bool {
        *self == EXTI4::Ph
    }
    ///PIx pin
    #[inline(always)]
    pub fn is_pi(&self) -> bool {
        *self == EXTI4::Pi
    }
    ///PJx pin
    #[inline(always)]
    pub fn is_pj(&self) -> bool {
        *self == EXTI4::Pj
    }
}
///Field `EXTI4` writer - EXTIm GPIO port selection
pub type EXTI4_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI4>;
impl<'a, REG> EXTI4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PAx pin
    #[inline(always)]
    pub fn pa(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Pa)
    }
    ///PBx pin
    #[inline(always)]
    pub fn pb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Pb)
    }
    ///PCx pin
    #[inline(always)]
    pub fn pc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Pc)
    }
    ///PDx pin
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Pd)
    }
    ///PEx pin
    #[inline(always)]
    pub fn pe(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Pe)
    }
    ///PFx pin
    #[inline(always)]
    pub fn pf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Pf)
    }
    ///PGx pin
    #[inline(always)]
    pub fn pg(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Pg)
    }
    ///PHx pin
    #[inline(always)]
    pub fn ph(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Ph)
    }
    ///PIx pin
    #[inline(always)]
    pub fn pi(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Pi)
    }
    ///PJx pin
    #[inline(always)]
    pub fn pj(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4::Pj)
    }
}
///Field `EXTI5` reader - EXTIm+1 GPIO port selection
pub use EXTI4_R as EXTI5_R;
///Field `EXTI6` reader - EXTIm+2 GPIO port selection
pub use EXTI4_R as EXTI6_R;
///Field `EXTI7` reader - EXTIm+3 GPIO port selection
pub use EXTI4_R as EXTI7_R;
///Field `EXTI5` writer - EXTIm+1 GPIO port selection
pub use EXTI4_W as EXTI5_W;
///Field `EXTI6` writer - EXTIm+2 GPIO port selection
pub use EXTI4_W as EXTI6_W;
///Field `EXTI7` writer - EXTIm+3 GPIO port selection
pub use EXTI4_W as EXTI7_W;
impl R {
    ///Bits 0:7 - EXTIm GPIO port selection
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - EXTIm+1 GPIO port selection
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - EXTIm+2 GPIO port selection
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - EXTIm+3 GPIO port selection
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTICR2")
            .field("exti4", &self.exti4())
            .field("exti5", &self.exti5())
            .field("exti6", &self.exti6())
            .field("exti7", &self.exti7())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - EXTIm GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti4(&mut self) -> EXTI4_W<EXTICR2rs> {
        EXTI4_W::new(self, 0)
    }
    ///Bits 8:15 - EXTIm+1 GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti5(&mut self) -> EXTI5_W<EXTICR2rs> {
        EXTI5_W::new(self, 8)
    }
    ///Bits 16:23 - EXTIm+2 GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti6(&mut self) -> EXTI6_W<EXTICR2rs> {
        EXTI6_W::new(self, 16)
    }
    ///Bits 24:31 - EXTIm+3 GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti7(&mut self) -> EXTI7_W<EXTICR2rs> {
        EXTI7_W::new(self, 24)
    }
}
/**EXTI external interrupt selection register

You can [`read`](crate::Reg::read) this register and get [`exticr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#EXTI:EXTICR2)*/
pub struct EXTICR2rs;
impl crate::RegisterSpec for EXTICR2rs {
    type Ux = u32;
}
///`read()` method returns [`exticr2::R`](R) reader structure
impl crate::Readable for EXTICR2rs {}
///`write(|w| ..)` method takes [`exticr2::W`](W) writer structure
impl crate::Writable for EXTICR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EXTICR2 to value 0
impl crate::Resettable for EXTICR2rs {
    const RESET_VALUE: u32 = 0;
}
