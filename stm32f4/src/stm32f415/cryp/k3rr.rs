///Register `K3RR` writer
pub type W = crate::W<K3RRrs>;
///Field `b0` writer - b0
pub type B0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b1` writer - b1
pub type B1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b2` writer - b2
pub type B2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b3` writer - b3
pub type B3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b4` writer - b4
pub type B4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b5` writer - b5
pub type B5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b6` writer - b6
pub type B6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b7` writer - b7
pub type B7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b8` writer - b8
pub type B8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b9` writer - b9
pub type B9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b10` writer - b10
pub type B10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b11` writer - b11
pub type B11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b12` writer - b12
pub type B12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b13` writer - b13
pub type B13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b14` writer - b14
pub type B14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b15` writer - b15
pub type B15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b16` writer - b16
pub type B16_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b17` writer - b17
pub type B17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b18` writer - b18
pub type B18_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b19` writer - b19
pub type B19_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b20` writer - b20
pub type B20_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b21` writer - b21
pub type B21_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b22` writer - b22
pub type B22_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b23` writer - b23
pub type B23_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b24` writer - b24
pub type B24_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b25` writer - b25
pub type B25_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b26` writer - b26
pub type B26_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b27` writer - b27
pub type B27_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b28` writer - b28
pub type B28_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b29` writer - b29
pub type B29_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b30` writer - b30
pub type B30_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `b31` writer - b31
pub type B31_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<K3RRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - b0
    #[inline(always)]
    pub fn b0(&mut self) -> B0_W<'_, K3RRrs> {
        B0_W::new(self, 0)
    }
    ///Bit 1 - b1
    #[inline(always)]
    pub fn b1(&mut self) -> B1_W<'_, K3RRrs> {
        B1_W::new(self, 1)
    }
    ///Bit 2 - b2
    #[inline(always)]
    pub fn b2(&mut self) -> B2_W<'_, K3RRrs> {
        B2_W::new(self, 2)
    }
    ///Bit 3 - b3
    #[inline(always)]
    pub fn b3(&mut self) -> B3_W<'_, K3RRrs> {
        B3_W::new(self, 3)
    }
    ///Bit 4 - b4
    #[inline(always)]
    pub fn b4(&mut self) -> B4_W<'_, K3RRrs> {
        B4_W::new(self, 4)
    }
    ///Bit 5 - b5
    #[inline(always)]
    pub fn b5(&mut self) -> B5_W<'_, K3RRrs> {
        B5_W::new(self, 5)
    }
    ///Bit 6 - b6
    #[inline(always)]
    pub fn b6(&mut self) -> B6_W<'_, K3RRrs> {
        B6_W::new(self, 6)
    }
    ///Bit 7 - b7
    #[inline(always)]
    pub fn b7(&mut self) -> B7_W<'_, K3RRrs> {
        B7_W::new(self, 7)
    }
    ///Bit 8 - b8
    #[inline(always)]
    pub fn b8(&mut self) -> B8_W<'_, K3RRrs> {
        B8_W::new(self, 8)
    }
    ///Bit 9 - b9
    #[inline(always)]
    pub fn b9(&mut self) -> B9_W<'_, K3RRrs> {
        B9_W::new(self, 9)
    }
    ///Bit 10 - b10
    #[inline(always)]
    pub fn b10(&mut self) -> B10_W<'_, K3RRrs> {
        B10_W::new(self, 10)
    }
    ///Bit 11 - b11
    #[inline(always)]
    pub fn b11(&mut self) -> B11_W<'_, K3RRrs> {
        B11_W::new(self, 11)
    }
    ///Bit 12 - b12
    #[inline(always)]
    pub fn b12(&mut self) -> B12_W<'_, K3RRrs> {
        B12_W::new(self, 12)
    }
    ///Bit 13 - b13
    #[inline(always)]
    pub fn b13(&mut self) -> B13_W<'_, K3RRrs> {
        B13_W::new(self, 13)
    }
    ///Bit 14 - b14
    #[inline(always)]
    pub fn b14(&mut self) -> B14_W<'_, K3RRrs> {
        B14_W::new(self, 14)
    }
    ///Bit 15 - b15
    #[inline(always)]
    pub fn b15(&mut self) -> B15_W<'_, K3RRrs> {
        B15_W::new(self, 15)
    }
    ///Bit 16 - b16
    #[inline(always)]
    pub fn b16(&mut self) -> B16_W<'_, K3RRrs> {
        B16_W::new(self, 16)
    }
    ///Bit 17 - b17
    #[inline(always)]
    pub fn b17(&mut self) -> B17_W<'_, K3RRrs> {
        B17_W::new(self, 17)
    }
    ///Bit 18 - b18
    #[inline(always)]
    pub fn b18(&mut self) -> B18_W<'_, K3RRrs> {
        B18_W::new(self, 18)
    }
    ///Bit 19 - b19
    #[inline(always)]
    pub fn b19(&mut self) -> B19_W<'_, K3RRrs> {
        B19_W::new(self, 19)
    }
    ///Bit 20 - b20
    #[inline(always)]
    pub fn b20(&mut self) -> B20_W<'_, K3RRrs> {
        B20_W::new(self, 20)
    }
    ///Bit 21 - b21
    #[inline(always)]
    pub fn b21(&mut self) -> B21_W<'_, K3RRrs> {
        B21_W::new(self, 21)
    }
    ///Bit 22 - b22
    #[inline(always)]
    pub fn b22(&mut self) -> B22_W<'_, K3RRrs> {
        B22_W::new(self, 22)
    }
    ///Bit 23 - b23
    #[inline(always)]
    pub fn b23(&mut self) -> B23_W<'_, K3RRrs> {
        B23_W::new(self, 23)
    }
    ///Bit 24 - b24
    #[inline(always)]
    pub fn b24(&mut self) -> B24_W<'_, K3RRrs> {
        B24_W::new(self, 24)
    }
    ///Bit 25 - b25
    #[inline(always)]
    pub fn b25(&mut self) -> B25_W<'_, K3RRrs> {
        B25_W::new(self, 25)
    }
    ///Bit 26 - b26
    #[inline(always)]
    pub fn b26(&mut self) -> B26_W<'_, K3RRrs> {
        B26_W::new(self, 26)
    }
    ///Bit 27 - b27
    #[inline(always)]
    pub fn b27(&mut self) -> B27_W<'_, K3RRrs> {
        B27_W::new(self, 27)
    }
    ///Bit 28 - b28
    #[inline(always)]
    pub fn b28(&mut self) -> B28_W<'_, K3RRrs> {
        B28_W::new(self, 28)
    }
    ///Bit 29 - b29
    #[inline(always)]
    pub fn b29(&mut self) -> B29_W<'_, K3RRrs> {
        B29_W::new(self, 29)
    }
    ///Bit 30 - b30
    #[inline(always)]
    pub fn b30(&mut self) -> B30_W<'_, K3RRrs> {
        B30_W::new(self, 30)
    }
    ///Bit 31 - b31
    #[inline(always)]
    pub fn b31(&mut self) -> B31_W<'_, K3RRrs> {
        B31_W::new(self, 31)
    }
}
/**key registers

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`k3rr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#CRYP:K3RR)*/
pub struct K3RRrs;
impl crate::RegisterSpec for K3RRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`k3rr::W`](W) writer structure
impl crate::Writable for K3RRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets K3RR to value 0
impl crate::Resettable for K3RRrs {}
