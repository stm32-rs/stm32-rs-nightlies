///Register `FMKEYR2` writer
pub type W = crate::W<FMKEYR2rs>;
///Field `FMKEY64` writer - Fast master key bit 64 (i = 31 to 0)
pub type FMKEY64_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY65` writer - Fast master key bit 65 (i = 31 to 0)
pub type FMKEY65_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY66` writer - Fast master key bit 66 (i = 31 to 0)
pub type FMKEY66_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY67` writer - Fast master key bit 67 (i = 31 to 0)
pub type FMKEY67_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY68` writer - Fast master key bit 68 (i = 31 to 0)
pub type FMKEY68_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY69` writer - Fast master key bit 69 (i = 31 to 0)
pub type FMKEY69_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY70` writer - Fast master key bit 70 (i = 31 to 0)
pub type FMKEY70_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY71` writer - Fast master key bit 71 (i = 31 to 0)
pub type FMKEY71_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY72` writer - Fast master key bit 72 (i = 31 to 0)
pub type FMKEY72_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY73` writer - Fast master key bit 73 (i = 31 to 0)
pub type FMKEY73_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY74` writer - Fast master key bit 74 (i = 31 to 0)
pub type FMKEY74_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY75` writer - Fast master key bit 75 (i = 31 to 0)
pub type FMKEY75_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY76` writer - Fast master key bit 76 (i = 31 to 0)
pub type FMKEY76_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY77` writer - Fast master key bit 77 (i = 31 to 0)
pub type FMKEY77_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY78` writer - Fast master key bit 78 (i = 31 to 0)
pub type FMKEY78_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY79` writer - Fast master key bit 79 (i = 31 to 0)
pub type FMKEY79_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY80` writer - Fast master key bit 80 (i = 31 to 0)
pub type FMKEY80_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY81` writer - Fast master key bit 81 (i = 31 to 0)
pub type FMKEY81_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY82` writer - Fast master key bit 82 (i = 31 to 0)
pub type FMKEY82_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY83` writer - Fast master key bit 83 (i = 31 to 0)
pub type FMKEY83_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY84` writer - Fast master key bit 84 (i = 31 to 0)
pub type FMKEY84_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY85` writer - Fast master key bit 85 (i = 31 to 0)
pub type FMKEY85_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY86` writer - Fast master key bit 86 (i = 31 to 0)
pub type FMKEY86_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY87` writer - Fast master key bit 87 (i = 31 to 0)
pub type FMKEY87_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY88` writer - Fast master key bit 88 (i = 31 to 0)
pub type FMKEY88_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY89` writer - Fast master key bit 89 (i = 31 to 0)
pub type FMKEY89_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY90` writer - Fast master key bit 90 (i = 31 to 0)
pub type FMKEY90_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY91` writer - Fast master key bit 91 (i = 31 to 0)
pub type FMKEY91_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY92` writer - Fast master key bit 92 (i = 31 to 0)
pub type FMKEY92_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY93` writer - Fast master key bit 93 (i = 31 to 0)
pub type FMKEY93_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY94` writer - Fast master key bit 94 (i = 31 to 0)
pub type FMKEY94_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY95` writer - Fast master key bit 95 (i = 31 to 0)
pub type FMKEY95_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FMKEYR2rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Fast master key bit 64 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey64(&mut self) -> FMKEY64_W<'_, FMKEYR2rs> {
        FMKEY64_W::new(self, 0)
    }
    ///Bit 1 - Fast master key bit 65 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey65(&mut self) -> FMKEY65_W<'_, FMKEYR2rs> {
        FMKEY65_W::new(self, 1)
    }
    ///Bit 2 - Fast master key bit 66 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey66(&mut self) -> FMKEY66_W<'_, FMKEYR2rs> {
        FMKEY66_W::new(self, 2)
    }
    ///Bit 3 - Fast master key bit 67 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey67(&mut self) -> FMKEY67_W<'_, FMKEYR2rs> {
        FMKEY67_W::new(self, 3)
    }
    ///Bit 4 - Fast master key bit 68 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey68(&mut self) -> FMKEY68_W<'_, FMKEYR2rs> {
        FMKEY68_W::new(self, 4)
    }
    ///Bit 5 - Fast master key bit 69 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey69(&mut self) -> FMKEY69_W<'_, FMKEYR2rs> {
        FMKEY69_W::new(self, 5)
    }
    ///Bit 6 - Fast master key bit 70 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey70(&mut self) -> FMKEY70_W<'_, FMKEYR2rs> {
        FMKEY70_W::new(self, 6)
    }
    ///Bit 7 - Fast master key bit 71 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey71(&mut self) -> FMKEY71_W<'_, FMKEYR2rs> {
        FMKEY71_W::new(self, 7)
    }
    ///Bit 8 - Fast master key bit 72 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey72(&mut self) -> FMKEY72_W<'_, FMKEYR2rs> {
        FMKEY72_W::new(self, 8)
    }
    ///Bit 9 - Fast master key bit 73 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey73(&mut self) -> FMKEY73_W<'_, FMKEYR2rs> {
        FMKEY73_W::new(self, 9)
    }
    ///Bit 10 - Fast master key bit 74 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey74(&mut self) -> FMKEY74_W<'_, FMKEYR2rs> {
        FMKEY74_W::new(self, 10)
    }
    ///Bit 11 - Fast master key bit 75 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey75(&mut self) -> FMKEY75_W<'_, FMKEYR2rs> {
        FMKEY75_W::new(self, 11)
    }
    ///Bit 12 - Fast master key bit 76 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey76(&mut self) -> FMKEY76_W<'_, FMKEYR2rs> {
        FMKEY76_W::new(self, 12)
    }
    ///Bit 13 - Fast master key bit 77 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey77(&mut self) -> FMKEY77_W<'_, FMKEYR2rs> {
        FMKEY77_W::new(self, 13)
    }
    ///Bit 14 - Fast master key bit 78 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey78(&mut self) -> FMKEY78_W<'_, FMKEYR2rs> {
        FMKEY78_W::new(self, 14)
    }
    ///Bit 15 - Fast master key bit 79 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey79(&mut self) -> FMKEY79_W<'_, FMKEYR2rs> {
        FMKEY79_W::new(self, 15)
    }
    ///Bit 16 - Fast master key bit 80 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey80(&mut self) -> FMKEY80_W<'_, FMKEYR2rs> {
        FMKEY80_W::new(self, 16)
    }
    ///Bit 17 - Fast master key bit 81 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey81(&mut self) -> FMKEY81_W<'_, FMKEYR2rs> {
        FMKEY81_W::new(self, 17)
    }
    ///Bit 18 - Fast master key bit 82 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey82(&mut self) -> FMKEY82_W<'_, FMKEYR2rs> {
        FMKEY82_W::new(self, 18)
    }
    ///Bit 19 - Fast master key bit 83 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey83(&mut self) -> FMKEY83_W<'_, FMKEYR2rs> {
        FMKEY83_W::new(self, 19)
    }
    ///Bit 20 - Fast master key bit 84 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey84(&mut self) -> FMKEY84_W<'_, FMKEYR2rs> {
        FMKEY84_W::new(self, 20)
    }
    ///Bit 21 - Fast master key bit 85 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey85(&mut self) -> FMKEY85_W<'_, FMKEYR2rs> {
        FMKEY85_W::new(self, 21)
    }
    ///Bit 22 - Fast master key bit 86 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey86(&mut self) -> FMKEY86_W<'_, FMKEYR2rs> {
        FMKEY86_W::new(self, 22)
    }
    ///Bit 23 - Fast master key bit 87 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey87(&mut self) -> FMKEY87_W<'_, FMKEYR2rs> {
        FMKEY87_W::new(self, 23)
    }
    ///Bit 24 - Fast master key bit 88 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey88(&mut self) -> FMKEY88_W<'_, FMKEYR2rs> {
        FMKEY88_W::new(self, 24)
    }
    ///Bit 25 - Fast master key bit 89 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey89(&mut self) -> FMKEY89_W<'_, FMKEYR2rs> {
        FMKEY89_W::new(self, 25)
    }
    ///Bit 26 - Fast master key bit 90 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey90(&mut self) -> FMKEY90_W<'_, FMKEYR2rs> {
        FMKEY90_W::new(self, 26)
    }
    ///Bit 27 - Fast master key bit 91 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey91(&mut self) -> FMKEY91_W<'_, FMKEYR2rs> {
        FMKEY91_W::new(self, 27)
    }
    ///Bit 28 - Fast master key bit 92 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey92(&mut self) -> FMKEY92_W<'_, FMKEYR2rs> {
        FMKEY92_W::new(self, 28)
    }
    ///Bit 29 - Fast master key bit 93 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey93(&mut self) -> FMKEY93_W<'_, FMKEYR2rs> {
        FMKEY93_W::new(self, 29)
    }
    ///Bit 30 - Fast master key bit 94 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey94(&mut self) -> FMKEY94_W<'_, FMKEYR2rs> {
        FMKEY94_W::new(self, 30)
    }
    ///Bit 31 - Fast master key bit 95 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey95(&mut self) -> FMKEY95_W<'_, FMKEYR2rs> {
        FMKEY95_W::new(self, 31)
    }
}
/**MCE fast master key 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmkeyr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:FMKEYR2)*/
pub struct FMKEYR2rs;
impl crate::RegisterSpec for FMKEYR2rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fmkeyr2::W`](W) writer structure
impl crate::Writable for FMKEYR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FMKEYR2 to value 0
impl crate::Resettable for FMKEYR2rs {}
