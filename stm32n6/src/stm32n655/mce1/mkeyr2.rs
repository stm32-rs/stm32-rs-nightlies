///Register `MKEYR2` writer
pub type W = crate::W<MKEYR2rs>;
///Field `MKEY64` writer - Master key bit 64 (i = 31 to 0)
pub type MKEY64_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY65` writer - Master key bit 65 (i = 31 to 0)
pub type MKEY65_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY66` writer - Master key bit 66 (i = 31 to 0)
pub type MKEY66_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY67` writer - Master key bit 67 (i = 31 to 0)
pub type MKEY67_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY68` writer - Master key bit 68 (i = 31 to 0)
pub type MKEY68_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY69` writer - Master key bit 69 (i = 31 to 0)
pub type MKEY69_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY70` writer - Master key bit 70 (i = 31 to 0)
pub type MKEY70_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY71` writer - Master key bit 71 (i = 31 to 0)
pub type MKEY71_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY72` writer - Master key bit 72 (i = 31 to 0)
pub type MKEY72_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY73` writer - Master key bit 73 (i = 31 to 0)
pub type MKEY73_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY74` writer - Master key bit 74 (i = 31 to 0)
pub type MKEY74_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY75` writer - Master key bit 75 (i = 31 to 0)
pub type MKEY75_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY76` writer - Master key bit 76 (i = 31 to 0)
pub type MKEY76_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY77` writer - Master key bit 77 (i = 31 to 0)
pub type MKEY77_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY78` writer - Master key bit 78 (i = 31 to 0)
pub type MKEY78_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY79` writer - Master key bit 79 (i = 31 to 0)
pub type MKEY79_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY80` writer - Master key bit 80 (i = 31 to 0)
pub type MKEY80_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY81` writer - Master key bit 81 (i = 31 to 0)
pub type MKEY81_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY82` writer - Master key bit 82 (i = 31 to 0)
pub type MKEY82_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY83` writer - Master key bit 83 (i = 31 to 0)
pub type MKEY83_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY84` writer - Master key bit 84 (i = 31 to 0)
pub type MKEY84_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY85` writer - Master key bit 85 (i = 31 to 0)
pub type MKEY85_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY86` writer - Master key bit 86 (i = 31 to 0)
pub type MKEY86_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY87` writer - Master key bit 87 (i = 31 to 0)
pub type MKEY87_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY88` writer - Master key bit 88 (i = 31 to 0)
pub type MKEY88_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY89` writer - Master key bit 89 (i = 31 to 0)
pub type MKEY89_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY90` writer - Master key bit 90 (i = 31 to 0)
pub type MKEY90_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY91` writer - Master key bit 91 (i = 31 to 0)
pub type MKEY91_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY92` writer - Master key bit 92 (i = 31 to 0)
pub type MKEY92_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY93` writer - Master key bit 93 (i = 31 to 0)
pub type MKEY93_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY94` writer - Master key bit 94 (i = 31 to 0)
pub type MKEY94_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY95` writer - Master key bit 95 (i = 31 to 0)
pub type MKEY95_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<MKEYR2rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Master key bit 64 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey64(&mut self) -> MKEY64_W<'_, MKEYR2rs> {
        MKEY64_W::new(self, 0)
    }
    ///Bit 1 - Master key bit 65 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey65(&mut self) -> MKEY65_W<'_, MKEYR2rs> {
        MKEY65_W::new(self, 1)
    }
    ///Bit 2 - Master key bit 66 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey66(&mut self) -> MKEY66_W<'_, MKEYR2rs> {
        MKEY66_W::new(self, 2)
    }
    ///Bit 3 - Master key bit 67 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey67(&mut self) -> MKEY67_W<'_, MKEYR2rs> {
        MKEY67_W::new(self, 3)
    }
    ///Bit 4 - Master key bit 68 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey68(&mut self) -> MKEY68_W<'_, MKEYR2rs> {
        MKEY68_W::new(self, 4)
    }
    ///Bit 5 - Master key bit 69 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey69(&mut self) -> MKEY69_W<'_, MKEYR2rs> {
        MKEY69_W::new(self, 5)
    }
    ///Bit 6 - Master key bit 70 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey70(&mut self) -> MKEY70_W<'_, MKEYR2rs> {
        MKEY70_W::new(self, 6)
    }
    ///Bit 7 - Master key bit 71 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey71(&mut self) -> MKEY71_W<'_, MKEYR2rs> {
        MKEY71_W::new(self, 7)
    }
    ///Bit 8 - Master key bit 72 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey72(&mut self) -> MKEY72_W<'_, MKEYR2rs> {
        MKEY72_W::new(self, 8)
    }
    ///Bit 9 - Master key bit 73 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey73(&mut self) -> MKEY73_W<'_, MKEYR2rs> {
        MKEY73_W::new(self, 9)
    }
    ///Bit 10 - Master key bit 74 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey74(&mut self) -> MKEY74_W<'_, MKEYR2rs> {
        MKEY74_W::new(self, 10)
    }
    ///Bit 11 - Master key bit 75 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey75(&mut self) -> MKEY75_W<'_, MKEYR2rs> {
        MKEY75_W::new(self, 11)
    }
    ///Bit 12 - Master key bit 76 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey76(&mut self) -> MKEY76_W<'_, MKEYR2rs> {
        MKEY76_W::new(self, 12)
    }
    ///Bit 13 - Master key bit 77 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey77(&mut self) -> MKEY77_W<'_, MKEYR2rs> {
        MKEY77_W::new(self, 13)
    }
    ///Bit 14 - Master key bit 78 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey78(&mut self) -> MKEY78_W<'_, MKEYR2rs> {
        MKEY78_W::new(self, 14)
    }
    ///Bit 15 - Master key bit 79 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey79(&mut self) -> MKEY79_W<'_, MKEYR2rs> {
        MKEY79_W::new(self, 15)
    }
    ///Bit 16 - Master key bit 80 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey80(&mut self) -> MKEY80_W<'_, MKEYR2rs> {
        MKEY80_W::new(self, 16)
    }
    ///Bit 17 - Master key bit 81 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey81(&mut self) -> MKEY81_W<'_, MKEYR2rs> {
        MKEY81_W::new(self, 17)
    }
    ///Bit 18 - Master key bit 82 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey82(&mut self) -> MKEY82_W<'_, MKEYR2rs> {
        MKEY82_W::new(self, 18)
    }
    ///Bit 19 - Master key bit 83 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey83(&mut self) -> MKEY83_W<'_, MKEYR2rs> {
        MKEY83_W::new(self, 19)
    }
    ///Bit 20 - Master key bit 84 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey84(&mut self) -> MKEY84_W<'_, MKEYR2rs> {
        MKEY84_W::new(self, 20)
    }
    ///Bit 21 - Master key bit 85 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey85(&mut self) -> MKEY85_W<'_, MKEYR2rs> {
        MKEY85_W::new(self, 21)
    }
    ///Bit 22 - Master key bit 86 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey86(&mut self) -> MKEY86_W<'_, MKEYR2rs> {
        MKEY86_W::new(self, 22)
    }
    ///Bit 23 - Master key bit 87 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey87(&mut self) -> MKEY87_W<'_, MKEYR2rs> {
        MKEY87_W::new(self, 23)
    }
    ///Bit 24 - Master key bit 88 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey88(&mut self) -> MKEY88_W<'_, MKEYR2rs> {
        MKEY88_W::new(self, 24)
    }
    ///Bit 25 - Master key bit 89 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey89(&mut self) -> MKEY89_W<'_, MKEYR2rs> {
        MKEY89_W::new(self, 25)
    }
    ///Bit 26 - Master key bit 90 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey90(&mut self) -> MKEY90_W<'_, MKEYR2rs> {
        MKEY90_W::new(self, 26)
    }
    ///Bit 27 - Master key bit 91 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey91(&mut self) -> MKEY91_W<'_, MKEYR2rs> {
        MKEY91_W::new(self, 27)
    }
    ///Bit 28 - Master key bit 92 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey92(&mut self) -> MKEY92_W<'_, MKEYR2rs> {
        MKEY92_W::new(self, 28)
    }
    ///Bit 29 - Master key bit 93 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey93(&mut self) -> MKEY93_W<'_, MKEYR2rs> {
        MKEY93_W::new(self, 29)
    }
    ///Bit 30 - Master key bit 94 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey94(&mut self) -> MKEY94_W<'_, MKEYR2rs> {
        MKEY94_W::new(self, 30)
    }
    ///Bit 31 - Master key bit 95 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey95(&mut self) -> MKEY95_W<'_, MKEYR2rs> {
        MKEY95_W::new(self, 31)
    }
}
/**.MCE master key 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkeyr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:MKEYR2)*/
pub struct MKEYR2rs;
impl crate::RegisterSpec for MKEYR2rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`mkeyr2::W`](W) writer structure
impl crate::Writable for MKEYR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MKEYR2 to value 0
impl crate::Resettable for MKEYR2rs {}
