///Register `EXTICR3` reader
pub type R = crate::R<EXTICR3rs>;
///Register `EXTICR3` writer
pub type W = crate::W<EXTICR3rs>;
/**EXTIm GPIO port selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI8 {
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
///Field `EXTI8` reader - EXTIm GPIO port selection
pub type EXTI8_R = crate::FieldReader<EXTI8>;
impl EXTI8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI8> {
        match self.bits {
            0 => Some(EXTI8::Pa),
            1 => Some(EXTI8::Pb),
            2 => Some(EXTI8::Pc),
            3 => Some(EXTI8::Pd),
            4 => Some(EXTI8::Pe),
            5 => Some(EXTI8::Pf),
            6 => Some(EXTI8::Pg),
            7 => Some(EXTI8::Ph),
            8 => Some(EXTI8::Pi),
            9 => Some(EXTI8::Pj),
            _ => None,
        }
    }
    ///PAx pin
    #[inline(always)]
    pub fn is_pa(&self) -> bool {
        *self == EXTI8::Pa
    }
    ///PBx pin
    #[inline(always)]
    pub fn is_pb(&self) -> bool {
        *self == EXTI8::Pb
    }
    ///PCx pin
    #[inline(always)]
    pub fn is_pc(&self) -> bool {
        *self == EXTI8::Pc
    }
    ///PDx pin
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == EXTI8::Pd
    }
    ///PEx pin
    #[inline(always)]
    pub fn is_pe(&self) -> bool {
        *self == EXTI8::Pe
    }
    ///PFx pin
    #[inline(always)]
    pub fn is_pf(&self) -> bool {
        *self == EXTI8::Pf
    }
    ///PGx pin
    #[inline(always)]
    pub fn is_pg(&self) -> bool {
        *self == EXTI8::Pg
    }
    ///PHx pin
    #[inline(always)]
    pub fn is_ph(&self) -> bool {
        *self == EXTI8::Ph
    }
    ///PIx pin
    #[inline(always)]
    pub fn is_pi(&self) -> bool {
        *self == EXTI8::Pi
    }
    ///PJx pin
    #[inline(always)]
    pub fn is_pj(&self) -> bool {
        *self == EXTI8::Pj
    }
}
///Field `EXTI8` writer - EXTIm GPIO port selection
pub type EXTI8_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI8>;
impl<'a, REG> EXTI8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PAx pin
    #[inline(always)]
    pub fn pa(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8::Pa)
    }
    ///PBx pin
    #[inline(always)]
    pub fn pb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8::Pb)
    }
    ///PCx pin
    #[inline(always)]
    pub fn pc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8::Pc)
    }
    ///PDx pin
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8::Pd)
    }
    ///PEx pin
    #[inline(always)]
    pub fn pe(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8::Pe)
    }
    ///PFx pin
    #[inline(always)]
    pub fn pf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8::Pf)
    }
    ///PGx pin
    #[inline(always)]
    pub fn pg(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8::Pg)
    }
    ///PHx pin
    #[inline(always)]
    pub fn ph(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8::Ph)
    }
    ///PIx pin
    #[inline(always)]
    pub fn pi(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8::Pi)
    }
    ///PJx pin
    #[inline(always)]
    pub fn pj(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8::Pj)
    }
}
///Field `EXTI9` reader - EXTIm+1 GPIO port selection
pub use EXTI8_R as EXTI9_R;
///Field `EXTI10` reader - EXTIm+2 GPIO port selection
pub use EXTI8_R as EXTI10_R;
///Field `EXTI11` reader - EXTIm+3 GPIO port selection
pub use EXTI8_R as EXTI11_R;
///Field `EXTI9` writer - EXTIm+1 GPIO port selection
pub use EXTI8_W as EXTI9_W;
///Field `EXTI10` writer - EXTIm+2 GPIO port selection
pub use EXTI8_W as EXTI10_W;
///Field `EXTI11` writer - EXTIm+3 GPIO port selection
pub use EXTI8_W as EXTI11_W;
impl R {
    ///Bits 0:7 - EXTIm GPIO port selection
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - EXTIm+1 GPIO port selection
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - EXTIm+2 GPIO port selection
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - EXTIm+3 GPIO port selection
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTICR3")
            .field("exti8", &self.exti8())
            .field("exti9", &self.exti9())
            .field("exti10", &self.exti10())
            .field("exti11", &self.exti11())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - EXTIm GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti8(&mut self) -> EXTI8_W<EXTICR3rs> {
        EXTI8_W::new(self, 0)
    }
    ///Bits 8:15 - EXTIm+1 GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti9(&mut self) -> EXTI9_W<EXTICR3rs> {
        EXTI9_W::new(self, 8)
    }
    ///Bits 16:23 - EXTIm+2 GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti10(&mut self) -> EXTI10_W<EXTICR3rs> {
        EXTI10_W::new(self, 16)
    }
    ///Bits 24:31 - EXTIm+3 GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti11(&mut self) -> EXTI11_W<EXTICR3rs> {
        EXTI11_W::new(self, 24)
    }
}
/**EXTI external interrupt selection register

You can [`read`](crate::Reg::read) this register and get [`exticr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#EXTI:EXTICR3)*/
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
