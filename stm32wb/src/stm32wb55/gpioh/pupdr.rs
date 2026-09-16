///Register `PUPDR` reader
pub type R = crate::R<PUPDRrs>;
///Register `PUPDR` writer
pub type W = crate::W<PUPDRrs>;
/**Port x configuration bits (y = 0..15)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PULL {
    ///0: No pull-up, pull-down
    Floating = 0,
    ///1: Pull-up
    PullUp = 1,
    ///2: Pull-down
    PullDown = 2,
}
impl From<PULL> for u8 {
    #[inline(always)]
    fn from(variant: PULL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PULL {
    type Ux = u8;
}
impl crate::IsEnum for PULL {}
///Field `PUPDR0` reader - Port x configuration bits (y = 0..15)
pub type PUPDR0_R = crate::FieldReader<PULL>;
impl PUPDR0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PULL> {
        match self.bits {
            0 => Some(PULL::Floating),
            1 => Some(PULL::PullUp),
            2 => Some(PULL::PullDown),
            _ => None,
        }
    }
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn is_floating(&self) -> bool {
        *self == PULL::Floating
    }
    ///Pull-up
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PULL::PullUp
    }
    ///Pull-down
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PULL::PullDown
    }
}
///Field `PUPDR0` writer - Port x configuration bits (y = 0..15)
pub type PUPDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PULL>;
impl<'a, REG> PUPDR0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn floating(self) -> &'a mut crate::W<REG> {
        self.variant(PULL::Floating)
    }
    ///Pull-up
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PULL::PullUp)
    }
    ///Pull-down
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PULL::PullDown)
    }
}
///Field `PUPDR1` reader - Port x configuration bits (y = 0..15)
pub use PUPDR0_R as PUPDR1_R;
///Field `PUPDR3` reader - Port x configuration bits (y = 0..15)
pub use PUPDR0_R as PUPDR3_R;
///Field `PUPDR1` writer - Port x configuration bits (y = 0..15)
pub use PUPDR0_W as PUPDR1_W;
///Field `PUPDR3` writer - Port x configuration bits (y = 0..15)
pub use PUPDR0_W as PUPDR3_W;
impl R {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr0(&self) -> PUPDR0_R {
        PUPDR0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr1(&self) -> PUPDR1_R {
        PUPDR1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr3(&self) -> PUPDR3_R {
        PUPDR3_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUPDR")
            .field("pupdr0", &self.pupdr0())
            .field("pupdr3", &self.pupdr3())
            .field("pupdr1", &self.pupdr1())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr0(&mut self) -> PUPDR0_W<'_, PUPDRrs> {
        PUPDR0_W::new(self, 0)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr1(&mut self) -> PUPDR1_W<'_, PUPDRrs> {
        PUPDR1_W::new(self, 2)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr3(&mut self) -> PUPDR3_W<'_, PUPDRrs> {
        PUPDR3_W::new(self, 6)
    }
}
/**GPIO port pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`pupdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOH:PUPDR)*/
pub struct PUPDRrs;
impl crate::RegisterSpec for PUPDRrs {
    type Ux = u32;
}
///`read()` method returns [`pupdr::R`](R) reader structure
impl crate::Readable for PUPDRrs {}
///`write(|w| ..)` method takes [`pupdr::W`](W) writer structure
impl crate::Writable for PUPDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUPDR to value 0
impl crate::Resettable for PUPDRrs {}
