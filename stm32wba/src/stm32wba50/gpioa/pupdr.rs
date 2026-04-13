///Register `PUPDR` reader
pub type R = crate::R<PUPDRrs>;
///Register `PUPDR` writer
pub type W = crate::W<PUPDRrs>;
/**Port configuration I/O pin 0

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
///Field `PUPD0` reader - Port configuration I/O pin 0
pub type PUPD0_R = crate::FieldReader<PULL>;
impl PUPD0_R {
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
///Field `PUPD0` writer - Port configuration I/O pin 0
pub type PUPD0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PULL>;
impl<'a, REG> PUPD0_W<'a, REG>
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
///Field `PUPD1` reader - Port configuration I/O pin 1
pub use PUPD0_R as PUPD1_R;
///Field `PUPD2` reader - Port configuration I/O pin 2
pub use PUPD0_R as PUPD2_R;
///Field `PUPD3` reader - Port configuration I/O pin 3
pub use PUPD0_R as PUPD3_R;
///Field `PUPD5` reader - Port configuration I/O pin 5
pub use PUPD0_R as PUPD5_R;
///Field `PUPD6` reader - Port configuration I/O pin 6
pub use PUPD0_R as PUPD6_R;
///Field `PUPD7` reader - Port configuration I/O pin 7
pub use PUPD0_R as PUPD7_R;
///Field `PUPD8` reader - Port configuration I/O pin 8
pub use PUPD0_R as PUPD8_R;
///Field `PUPD9` reader - Port configuration I/O pin 9
pub use PUPD0_R as PUPD9_R;
///Field `PUPD10` reader - Port configuration I/O pin 10
pub use PUPD0_R as PUPD10_R;
///Field `PUPD11` reader - Port configuration I/O pin 11
pub use PUPD0_R as PUPD11_R;
///Field `PUPD12` reader - Port configuration I/O pin 12
pub use PUPD0_R as PUPD12_R;
///Field `PUPD13` reader - Port configuration I/O pin 13
pub use PUPD0_R as PUPD13_R;
///Field `PUPD14` reader - Port configuration I/O pin 14
pub use PUPD0_R as PUPD14_R;
///Field `PUPD15` reader - Port configuration I/O pin 15 These bits are written by software to configure the I/O pull-up or pull-down Access can be protected by GPIOA SEC15.
pub use PUPD0_R as PUPD15_R;
///Field `PUPD1` writer - Port configuration I/O pin 1
pub use PUPD0_W as PUPD1_W;
///Field `PUPD2` writer - Port configuration I/O pin 2
pub use PUPD0_W as PUPD2_W;
///Field `PUPD3` writer - Port configuration I/O pin 3
pub use PUPD0_W as PUPD3_W;
///Field `PUPD5` writer - Port configuration I/O pin 5
pub use PUPD0_W as PUPD5_W;
///Field `PUPD6` writer - Port configuration I/O pin 6
pub use PUPD0_W as PUPD6_W;
///Field `PUPD7` writer - Port configuration I/O pin 7
pub use PUPD0_W as PUPD7_W;
///Field `PUPD8` writer - Port configuration I/O pin 8
pub use PUPD0_W as PUPD8_W;
///Field `PUPD9` writer - Port configuration I/O pin 9
pub use PUPD0_W as PUPD9_W;
///Field `PUPD10` writer - Port configuration I/O pin 10
pub use PUPD0_W as PUPD10_W;
///Field `PUPD11` writer - Port configuration I/O pin 11
pub use PUPD0_W as PUPD11_W;
///Field `PUPD12` writer - Port configuration I/O pin 12
pub use PUPD0_W as PUPD12_W;
///Field `PUPD13` writer - Port configuration I/O pin 13
pub use PUPD0_W as PUPD13_W;
///Field `PUPD14` writer - Port configuration I/O pin 14
pub use PUPD0_W as PUPD14_W;
///Field `PUPD15` writer - Port configuration I/O pin 15 These bits are written by software to configure the I/O pull-up or pull-down Access can be protected by GPIOA SEC15.
pub use PUPD0_W as PUPD15_W;
impl R {
    ///Bits 0:1 - Port configuration I/O pin 0
    #[inline(always)]
    pub fn pupd0(&self) -> PUPD0_R {
        PUPD0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port configuration I/O pin 1
    #[inline(always)]
    pub fn pupd1(&self) -> PUPD1_R {
        PUPD1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port configuration I/O pin 2
    #[inline(always)]
    pub fn pupd2(&self) -> PUPD2_R {
        PUPD2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port configuration I/O pin 3
    #[inline(always)]
    pub fn pupd3(&self) -> PUPD3_R {
        PUPD3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 10:11 - Port configuration I/O pin 5
    #[inline(always)]
    pub fn pupd5(&self) -> PUPD5_R {
        PUPD5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Port configuration I/O pin 6
    #[inline(always)]
    pub fn pupd6(&self) -> PUPD6_R {
        PUPD6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Port configuration I/O pin 7
    #[inline(always)]
    pub fn pupd7(&self) -> PUPD7_R {
        PUPD7_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Port configuration I/O pin 8
    #[inline(always)]
    pub fn pupd8(&self) -> PUPD8_R {
        PUPD8_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Port configuration I/O pin 9
    #[inline(always)]
    pub fn pupd9(&self) -> PUPD9_R {
        PUPD9_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Port configuration I/O pin 10
    #[inline(always)]
    pub fn pupd10(&self) -> PUPD10_R {
        PUPD10_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Port configuration I/O pin 11
    #[inline(always)]
    pub fn pupd11(&self) -> PUPD11_R {
        PUPD11_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - Port configuration I/O pin 12
    #[inline(always)]
    pub fn pupd12(&self) -> PUPD12_R {
        PUPD12_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Port configuration I/O pin 13
    #[inline(always)]
    pub fn pupd13(&self) -> PUPD13_R {
        PUPD13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port configuration I/O pin 14
    #[inline(always)]
    pub fn pupd14(&self) -> PUPD14_R {
        PUPD14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port configuration I/O pin 15 These bits are written by software to configure the I/O pull-up or pull-down Access can be protected by GPIOA SEC15.
    #[inline(always)]
    pub fn pupd15(&self) -> PUPD15_R {
        PUPD15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUPDR")
            .field("pupd0", &self.pupd0())
            .field("pupd1", &self.pupd1())
            .field("pupd2", &self.pupd2())
            .field("pupd3", &self.pupd3())
            .field("pupd5", &self.pupd5())
            .field("pupd6", &self.pupd6())
            .field("pupd7", &self.pupd7())
            .field("pupd8", &self.pupd8())
            .field("pupd9", &self.pupd9())
            .field("pupd10", &self.pupd10())
            .field("pupd11", &self.pupd11())
            .field("pupd12", &self.pupd12())
            .field("pupd13", &self.pupd13())
            .field("pupd14", &self.pupd14())
            .field("pupd15", &self.pupd15())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Port configuration I/O pin 0
    #[inline(always)]
    pub fn pupd0(&mut self) -> PUPD0_W<'_, PUPDRrs> {
        PUPD0_W::new(self, 0)
    }
    ///Bits 2:3 - Port configuration I/O pin 1
    #[inline(always)]
    pub fn pupd1(&mut self) -> PUPD1_W<'_, PUPDRrs> {
        PUPD1_W::new(self, 2)
    }
    ///Bits 4:5 - Port configuration I/O pin 2
    #[inline(always)]
    pub fn pupd2(&mut self) -> PUPD2_W<'_, PUPDRrs> {
        PUPD2_W::new(self, 4)
    }
    ///Bits 6:7 - Port configuration I/O pin 3
    #[inline(always)]
    pub fn pupd3(&mut self) -> PUPD3_W<'_, PUPDRrs> {
        PUPD3_W::new(self, 6)
    }
    ///Bits 10:11 - Port configuration I/O pin 5
    #[inline(always)]
    pub fn pupd5(&mut self) -> PUPD5_W<'_, PUPDRrs> {
        PUPD5_W::new(self, 10)
    }
    ///Bits 12:13 - Port configuration I/O pin 6
    #[inline(always)]
    pub fn pupd6(&mut self) -> PUPD6_W<'_, PUPDRrs> {
        PUPD6_W::new(self, 12)
    }
    ///Bits 14:15 - Port configuration I/O pin 7
    #[inline(always)]
    pub fn pupd7(&mut self) -> PUPD7_W<'_, PUPDRrs> {
        PUPD7_W::new(self, 14)
    }
    ///Bits 16:17 - Port configuration I/O pin 8
    #[inline(always)]
    pub fn pupd8(&mut self) -> PUPD8_W<'_, PUPDRrs> {
        PUPD8_W::new(self, 16)
    }
    ///Bits 18:19 - Port configuration I/O pin 9
    #[inline(always)]
    pub fn pupd9(&mut self) -> PUPD9_W<'_, PUPDRrs> {
        PUPD9_W::new(self, 18)
    }
    ///Bits 20:21 - Port configuration I/O pin 10
    #[inline(always)]
    pub fn pupd10(&mut self) -> PUPD10_W<'_, PUPDRrs> {
        PUPD10_W::new(self, 20)
    }
    ///Bits 22:23 - Port configuration I/O pin 11
    #[inline(always)]
    pub fn pupd11(&mut self) -> PUPD11_W<'_, PUPDRrs> {
        PUPD11_W::new(self, 22)
    }
    ///Bits 24:25 - Port configuration I/O pin 12
    #[inline(always)]
    pub fn pupd12(&mut self) -> PUPD12_W<'_, PUPDRrs> {
        PUPD12_W::new(self, 24)
    }
    ///Bits 26:27 - Port configuration I/O pin 13
    #[inline(always)]
    pub fn pupd13(&mut self) -> PUPD13_W<'_, PUPDRrs> {
        PUPD13_W::new(self, 26)
    }
    ///Bits 28:29 - Port configuration I/O pin 14
    #[inline(always)]
    pub fn pupd14(&mut self) -> PUPD14_W<'_, PUPDRrs> {
        PUPD14_W::new(self, 28)
    }
    ///Bits 30:31 - Port configuration I/O pin 15 These bits are written by software to configure the I/O pull-up or pull-down Access can be protected by GPIOA SEC15.
    #[inline(always)]
    pub fn pupd15(&mut self) -> PUPD15_W<'_, PUPDRrs> {
        PUPD15_W::new(self, 30)
    }
}
/**GPIO port A pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`pupdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#GPIOA:PUPDR)*/
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
///`reset()` method sets PUPDR to value 0x6400_0000
impl crate::Resettable for PUPDRrs {
    const RESET_VALUE: u32 = 0x6400_0000;
}
