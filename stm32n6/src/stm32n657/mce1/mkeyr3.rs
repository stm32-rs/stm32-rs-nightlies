///Register `MKEYR3` writer
pub type W = crate::W<MKEYR3rs>;
///Field `MKEY96` writer - Master key bit 96 (i = 31 to 0)
pub type MKEY96_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY97` writer - Master key bit 97 (i = 31 to 0)
pub type MKEY97_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY98` writer - Master key bit 98 (i = 31 to 0)
pub type MKEY98_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY99` writer - Master key bit 99 (i = 31 to 0)
pub type MKEY99_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY100` writer - Master key bit 100 (i = 31 to 0)
pub type MKEY100_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY101` writer - Master key bit 101 (i = 31 to 0)
pub type MKEY101_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY102` writer - Master key bit 102 (i = 31 to 0)
pub type MKEY102_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY103` writer - Master key bit 103 (i = 31 to 0)
pub type MKEY103_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY104` writer - Master key bit 104 (i = 31 to 0)
pub type MKEY104_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY105` writer - Master key bit 105 (i = 31 to 0)
pub type MKEY105_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY106` writer - Master key bit 106 (i = 31 to 0)
pub type MKEY106_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY107` writer - Master key bit 107 (i = 31 to 0)
pub type MKEY107_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY108` writer - Master key bit 108 (i = 31 to 0)
pub type MKEY108_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY109` writer - Master key bit 109 (i = 31 to 0)
pub type MKEY109_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY110` writer - Master key bit 110 (i = 31 to 0)
pub type MKEY110_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY111` writer - Master key bit 111 (i = 31 to 0)
pub type MKEY111_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY112` writer - Master key bit 112 (i = 31 to 0)
pub type MKEY112_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY113` writer - Master key bit 113 (i = 31 to 0)
pub type MKEY113_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY114` writer - Master key bit 114 (i = 31 to 0)
pub type MKEY114_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY115` writer - Master key bit 115 (i = 31 to 0)
pub type MKEY115_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY116` writer - Master key bit 116 (i = 31 to 0)
pub type MKEY116_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY117` writer - Master key bit 117 (i = 31 to 0)
pub type MKEY117_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY118` writer - Master key bit 118 (i = 31 to 0)
pub type MKEY118_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY119` writer - Master key bit 119 (i = 31 to 0)
pub type MKEY119_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY120` writer - Master key bit 120 (i = 31 to 0)
pub type MKEY120_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY121` writer - Master key bit 121 (i = 31 to 0)
pub type MKEY121_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY122` writer - Master key bit 122 (i = 31 to 0)
pub type MKEY122_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY123` writer - Master key bit 123 (i = 31 to 0)
pub type MKEY123_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY124` writer - Master key bit 124 (i = 31 to 0)
pub type MKEY124_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY125` writer - Master key bit 125 (i = 31 to 0)
pub type MKEY125_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY126` writer - Master key bit 126 (i = 31 to 0)
pub type MKEY126_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY127` writer - Master key bit 127 (i = 31 to 0)
pub type MKEY127_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<MKEYR3rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Master key bit 96 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey96(&mut self) -> MKEY96_W<'_, MKEYR3rs> {
        MKEY96_W::new(self, 0)
    }
    ///Bit 1 - Master key bit 97 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey97(&mut self) -> MKEY97_W<'_, MKEYR3rs> {
        MKEY97_W::new(self, 1)
    }
    ///Bit 2 - Master key bit 98 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey98(&mut self) -> MKEY98_W<'_, MKEYR3rs> {
        MKEY98_W::new(self, 2)
    }
    ///Bit 3 - Master key bit 99 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey99(&mut self) -> MKEY99_W<'_, MKEYR3rs> {
        MKEY99_W::new(self, 3)
    }
    ///Bit 4 - Master key bit 100 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey100(&mut self) -> MKEY100_W<'_, MKEYR3rs> {
        MKEY100_W::new(self, 4)
    }
    ///Bit 5 - Master key bit 101 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey101(&mut self) -> MKEY101_W<'_, MKEYR3rs> {
        MKEY101_W::new(self, 5)
    }
    ///Bit 6 - Master key bit 102 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey102(&mut self) -> MKEY102_W<'_, MKEYR3rs> {
        MKEY102_W::new(self, 6)
    }
    ///Bit 7 - Master key bit 103 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey103(&mut self) -> MKEY103_W<'_, MKEYR3rs> {
        MKEY103_W::new(self, 7)
    }
    ///Bit 8 - Master key bit 104 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey104(&mut self) -> MKEY104_W<'_, MKEYR3rs> {
        MKEY104_W::new(self, 8)
    }
    ///Bit 9 - Master key bit 105 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey105(&mut self) -> MKEY105_W<'_, MKEYR3rs> {
        MKEY105_W::new(self, 9)
    }
    ///Bit 10 - Master key bit 106 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey106(&mut self) -> MKEY106_W<'_, MKEYR3rs> {
        MKEY106_W::new(self, 10)
    }
    ///Bit 11 - Master key bit 107 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey107(&mut self) -> MKEY107_W<'_, MKEYR3rs> {
        MKEY107_W::new(self, 11)
    }
    ///Bit 12 - Master key bit 108 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey108(&mut self) -> MKEY108_W<'_, MKEYR3rs> {
        MKEY108_W::new(self, 12)
    }
    ///Bit 13 - Master key bit 109 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey109(&mut self) -> MKEY109_W<'_, MKEYR3rs> {
        MKEY109_W::new(self, 13)
    }
    ///Bit 14 - Master key bit 110 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey110(&mut self) -> MKEY110_W<'_, MKEYR3rs> {
        MKEY110_W::new(self, 14)
    }
    ///Bit 15 - Master key bit 111 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey111(&mut self) -> MKEY111_W<'_, MKEYR3rs> {
        MKEY111_W::new(self, 15)
    }
    ///Bit 16 - Master key bit 112 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey112(&mut self) -> MKEY112_W<'_, MKEYR3rs> {
        MKEY112_W::new(self, 16)
    }
    ///Bit 17 - Master key bit 113 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey113(&mut self) -> MKEY113_W<'_, MKEYR3rs> {
        MKEY113_W::new(self, 17)
    }
    ///Bit 18 - Master key bit 114 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey114(&mut self) -> MKEY114_W<'_, MKEYR3rs> {
        MKEY114_W::new(self, 18)
    }
    ///Bit 19 - Master key bit 115 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey115(&mut self) -> MKEY115_W<'_, MKEYR3rs> {
        MKEY115_W::new(self, 19)
    }
    ///Bit 20 - Master key bit 116 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey116(&mut self) -> MKEY116_W<'_, MKEYR3rs> {
        MKEY116_W::new(self, 20)
    }
    ///Bit 21 - Master key bit 117 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey117(&mut self) -> MKEY117_W<'_, MKEYR3rs> {
        MKEY117_W::new(self, 21)
    }
    ///Bit 22 - Master key bit 118 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey118(&mut self) -> MKEY118_W<'_, MKEYR3rs> {
        MKEY118_W::new(self, 22)
    }
    ///Bit 23 - Master key bit 119 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey119(&mut self) -> MKEY119_W<'_, MKEYR3rs> {
        MKEY119_W::new(self, 23)
    }
    ///Bit 24 - Master key bit 120 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey120(&mut self) -> MKEY120_W<'_, MKEYR3rs> {
        MKEY120_W::new(self, 24)
    }
    ///Bit 25 - Master key bit 121 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey121(&mut self) -> MKEY121_W<'_, MKEYR3rs> {
        MKEY121_W::new(self, 25)
    }
    ///Bit 26 - Master key bit 122 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey122(&mut self) -> MKEY122_W<'_, MKEYR3rs> {
        MKEY122_W::new(self, 26)
    }
    ///Bit 27 - Master key bit 123 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey123(&mut self) -> MKEY123_W<'_, MKEYR3rs> {
        MKEY123_W::new(self, 27)
    }
    ///Bit 28 - Master key bit 124 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey124(&mut self) -> MKEY124_W<'_, MKEYR3rs> {
        MKEY124_W::new(self, 28)
    }
    ///Bit 29 - Master key bit 125 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey125(&mut self) -> MKEY125_W<'_, MKEYR3rs> {
        MKEY125_W::new(self, 29)
    }
    ///Bit 30 - Master key bit 126 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey126(&mut self) -> MKEY126_W<'_, MKEYR3rs> {
        MKEY126_W::new(self, 30)
    }
    ///Bit 31 - Master key bit 127 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey127(&mut self) -> MKEY127_W<'_, MKEYR3rs> {
        MKEY127_W::new(self, 31)
    }
}
/**.MCE master key 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkeyr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MCE1:MKEYR3)*/
pub struct MKEYR3rs;
impl crate::RegisterSpec for MKEYR3rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`mkeyr3::W`](W) writer structure
impl crate::Writable for MKEYR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MKEYR3 to value 0
impl crate::Resettable for MKEYR3rs {}
