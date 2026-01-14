///Register `PUPDR` reader
pub type R = crate::R<PUPDRrs>;
///Register `PUPDR` writer
pub type W = crate::W<PUPDRrs>;
/**Port C configuration I/O pin 13

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
///Field `PUPD13` reader - Port C configuration I/O pin 13
pub type PUPD13_R = crate::FieldReader<PULL>;
impl PUPD13_R {
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
///Field `PUPD13` writer - Port C configuration I/O pin 13
pub type PUPD13_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PULL>;
impl<'a, REG> PUPD13_W<'a, REG>
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
///Field `PUPD14` reader - Port C configuration I/O pin 14
pub use PUPD13_R as PUPD14_R;
///Field `PUPD15` reader - Port C configuration I/O pin 15 These bits are written by software to configure the I/O pull-up or pull-down Access can be protected by GPIOC SEC15.
pub use PUPD13_R as PUPD15_R;
///Field `PUPD14` writer - Port C configuration I/O pin 14
pub use PUPD13_W as PUPD14_W;
///Field `PUPD15` writer - Port C configuration I/O pin 15 These bits are written by software to configure the I/O pull-up or pull-down Access can be protected by GPIOC SEC15.
pub use PUPD13_W as PUPD15_W;
impl R {
    ///Bits 26:27 - Port C configuration I/O pin 13
    #[inline(always)]
    pub fn pupd13(&self) -> PUPD13_R {
        PUPD13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port C configuration I/O pin 14
    #[inline(always)]
    pub fn pupd14(&self) -> PUPD14_R {
        PUPD14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port C configuration I/O pin 15 These bits are written by software to configure the I/O pull-up or pull-down Access can be protected by GPIOC SEC15.
    #[inline(always)]
    pub fn pupd15(&self) -> PUPD15_R {
        PUPD15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUPDR")
            .field("pupd13", &self.pupd13())
            .field("pupd14", &self.pupd14())
            .field("pupd15", &self.pupd15())
            .finish()
    }
}
impl W {
    ///Bits 26:27 - Port C configuration I/O pin 13
    #[inline(always)]
    pub fn pupd13(&mut self) -> PUPD13_W<'_, PUPDRrs> {
        PUPD13_W::new(self, 26)
    }
    ///Bits 28:29 - Port C configuration I/O pin 14
    #[inline(always)]
    pub fn pupd14(&mut self) -> PUPD14_W<'_, PUPDRrs> {
        PUPD14_W::new(self, 28)
    }
    ///Bits 30:31 - Port C configuration I/O pin 15 These bits are written by software to configure the I/O pull-up or pull-down Access can be protected by GPIOC SEC15.
    #[inline(always)]
    pub fn pupd15(&mut self) -> PUPD15_W<'_, PUPDRrs> {
        PUPD15_W::new(self, 30)
    }
}
/**GPIO port C pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`pupdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#GPIOC:PUPDR)*/
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
