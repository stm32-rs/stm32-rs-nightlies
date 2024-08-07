///Register `EXTICR4` reader
pub type R = crate::R<EXTICR4rs>;
///Register `EXTICR4` writer
pub type W = crate::W<EXTICR4rs>;
/**EXTIm GPIO port selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI12 {
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
///Field `EXTI12` reader - EXTIm GPIO port selection
pub type EXTI12_R = crate::FieldReader<EXTI12>;
impl EXTI12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI12> {
        match self.bits {
            0 => Some(EXTI12::Pa),
            1 => Some(EXTI12::Pb),
            2 => Some(EXTI12::Pc),
            3 => Some(EXTI12::Pd),
            4 => Some(EXTI12::Pe),
            5 => Some(EXTI12::Pf),
            6 => Some(EXTI12::Pg),
            7 => Some(EXTI12::Ph),
            8 => Some(EXTI12::Pi),
            9 => Some(EXTI12::Pj),
            _ => None,
        }
    }
    ///PAx pin
    #[inline(always)]
    pub fn is_pa(&self) -> bool {
        *self == EXTI12::Pa
    }
    ///PBx pin
    #[inline(always)]
    pub fn is_pb(&self) -> bool {
        *self == EXTI12::Pb
    }
    ///PCx pin
    #[inline(always)]
    pub fn is_pc(&self) -> bool {
        *self == EXTI12::Pc
    }
    ///PDx pin
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == EXTI12::Pd
    }
    ///PEx pin
    #[inline(always)]
    pub fn is_pe(&self) -> bool {
        *self == EXTI12::Pe
    }
    ///PFx pin
    #[inline(always)]
    pub fn is_pf(&self) -> bool {
        *self == EXTI12::Pf
    }
    ///PGx pin
    #[inline(always)]
    pub fn is_pg(&self) -> bool {
        *self == EXTI12::Pg
    }
    ///PHx pin
    #[inline(always)]
    pub fn is_ph(&self) -> bool {
        *self == EXTI12::Ph
    }
    ///PIx pin
    #[inline(always)]
    pub fn is_pi(&self) -> bool {
        *self == EXTI12::Pi
    }
    ///PJx pin
    #[inline(always)]
    pub fn is_pj(&self) -> bool {
        *self == EXTI12::Pj
    }
}
///Field `EXTI12` writer - EXTIm GPIO port selection
pub type EXTI12_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI12>;
impl<'a, REG> EXTI12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PAx pin
    #[inline(always)]
    pub fn pa(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Pa)
    }
    ///PBx pin
    #[inline(always)]
    pub fn pb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Pb)
    }
    ///PCx pin
    #[inline(always)]
    pub fn pc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Pc)
    }
    ///PDx pin
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Pd)
    }
    ///PEx pin
    #[inline(always)]
    pub fn pe(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Pe)
    }
    ///PFx pin
    #[inline(always)]
    pub fn pf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Pf)
    }
    ///PGx pin
    #[inline(always)]
    pub fn pg(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Pg)
    }
    ///PHx pin
    #[inline(always)]
    pub fn ph(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Ph)
    }
    ///PIx pin
    #[inline(always)]
    pub fn pi(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Pi)
    }
    ///PJx pin
    #[inline(always)]
    pub fn pj(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12::Pj)
    }
}
///Field `EXTI13` reader - EXTIm+1 GPIO port selection
pub use EXTI12_R as EXTI13_R;
///Field `EXTI14` reader - EXTIm+2 GPIO port selection
pub use EXTI12_R as EXTI14_R;
///Field `EXTI15` reader - EXTIm+3 GPIO port selection
pub use EXTI12_R as EXTI15_R;
///Field `EXTI13` writer - EXTIm+1 GPIO port selection
pub use EXTI12_W as EXTI13_W;
///Field `EXTI14` writer - EXTIm+2 GPIO port selection
pub use EXTI12_W as EXTI14_W;
///Field `EXTI15` writer - EXTIm+3 GPIO port selection
pub use EXTI12_W as EXTI15_W;
impl R {
    ///Bits 0:7 - EXTIm GPIO port selection
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - EXTIm+1 GPIO port selection
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - EXTIm+2 GPIO port selection
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - EXTIm+3 GPIO port selection
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTICR4")
            .field("exti12", &self.exti12())
            .field("exti13", &self.exti13())
            .field("exti14", &self.exti14())
            .field("exti15", &self.exti15())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - EXTIm GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti12(&mut self) -> EXTI12_W<EXTICR4rs> {
        EXTI12_W::new(self, 0)
    }
    ///Bits 8:15 - EXTIm+1 GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti13(&mut self) -> EXTI13_W<EXTICR4rs> {
        EXTI13_W::new(self, 8)
    }
    ///Bits 16:23 - EXTIm+2 GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti14(&mut self) -> EXTI14_W<EXTICR4rs> {
        EXTI14_W::new(self, 16)
    }
    ///Bits 24:31 - EXTIm+3 GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti15(&mut self) -> EXTI15_W<EXTICR4rs> {
        EXTI15_W::new(self, 24)
    }
}
/**EXTI external interrupt selection register

You can [`read`](crate::Reg::read) this register and get [`exticr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#EXTI:EXTICR4)*/
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
