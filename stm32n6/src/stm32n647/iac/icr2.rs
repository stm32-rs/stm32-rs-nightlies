///Register `ICR2` writer
pub type W = crate::W<ICR2rs>;
///Field `IAF64` writer - illegal access flag clear for peripheral 64
pub type IAF64_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF65` writer - illegal access flag clear for peripheral 65
pub type IAF65_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF66` writer - illegal access flag clear for peripheral 66
pub type IAF66_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF67` writer - illegal access flag clear for peripheral 67
pub type IAF67_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF68` writer - illegal access flag clear for peripheral 68
pub type IAF68_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF69` writer - illegal access flag clear for peripheral 69
pub type IAF69_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF70` writer - illegal access flag clear for peripheral 70
pub type IAF70_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF71` writer - illegal access flag clear for peripheral 71
pub type IAF71_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF72` writer - illegal access flag clear for peripheral 72
pub type IAF72_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF73` writer - illegal access flag clear for peripheral 73
pub type IAF73_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF74` writer - illegal access flag clear for peripheral 74
pub type IAF74_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF75` writer - illegal access flag clear for peripheral 75
pub type IAF75_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF76` writer - illegal access flag clear for peripheral 76
pub type IAF76_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF77` writer - illegal access flag clear for peripheral 77
pub type IAF77_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF78` writer - illegal access flag clear for peripheral 78
pub type IAF78_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF79` writer - illegal access flag clear for peripheral 79
pub type IAF79_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF80` writer - illegal access flag clear for peripheral 80
pub type IAF80_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF81` writer - illegal access flag clear for peripheral 81
pub type IAF81_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF82` writer - illegal access flag clear for peripheral 82
pub type IAF82_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF83` writer - illegal access flag clear for peripheral 83
pub type IAF83_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF84` writer - illegal access flag clear for peripheral 84
pub type IAF84_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF85` writer - illegal access flag clear for peripheral 85
pub type IAF85_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF86` writer - illegal access flag clear for peripheral 86
pub type IAF86_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF87` writer - illegal access flag clear for peripheral 87
pub type IAF87_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF88` writer - illegal access flag clear for peripheral 88
pub type IAF88_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF89` writer - illegal access flag clear for peripheral 89
pub type IAF89_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF90` writer - illegal access flag clear for peripheral 90
pub type IAF90_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF91` writer - illegal access flag clear for peripheral 91
pub type IAF91_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF92` writer - illegal access flag clear for peripheral 92
pub type IAF92_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF93` writer - illegal access flag clear for peripheral 93
pub type IAF93_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF94` writer - illegal access flag clear for peripheral 94
pub type IAF94_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IAF95` writer - illegal access flag clear for peripheral 95
pub type IAF95_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICR2rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - illegal access flag clear for peripheral 64
    #[inline(always)]
    pub fn iaf64(&mut self) -> IAF64_W<'_, ICR2rs> {
        IAF64_W::new(self, 0)
    }
    ///Bit 1 - illegal access flag clear for peripheral 65
    #[inline(always)]
    pub fn iaf65(&mut self) -> IAF65_W<'_, ICR2rs> {
        IAF65_W::new(self, 1)
    }
    ///Bit 2 - illegal access flag clear for peripheral 66
    #[inline(always)]
    pub fn iaf66(&mut self) -> IAF66_W<'_, ICR2rs> {
        IAF66_W::new(self, 2)
    }
    ///Bit 3 - illegal access flag clear for peripheral 67
    #[inline(always)]
    pub fn iaf67(&mut self) -> IAF67_W<'_, ICR2rs> {
        IAF67_W::new(self, 3)
    }
    ///Bit 4 - illegal access flag clear for peripheral 68
    #[inline(always)]
    pub fn iaf68(&mut self) -> IAF68_W<'_, ICR2rs> {
        IAF68_W::new(self, 4)
    }
    ///Bit 5 - illegal access flag clear for peripheral 69
    #[inline(always)]
    pub fn iaf69(&mut self) -> IAF69_W<'_, ICR2rs> {
        IAF69_W::new(self, 5)
    }
    ///Bit 6 - illegal access flag clear for peripheral 70
    #[inline(always)]
    pub fn iaf70(&mut self) -> IAF70_W<'_, ICR2rs> {
        IAF70_W::new(self, 6)
    }
    ///Bit 7 - illegal access flag clear for peripheral 71
    #[inline(always)]
    pub fn iaf71(&mut self) -> IAF71_W<'_, ICR2rs> {
        IAF71_W::new(self, 7)
    }
    ///Bit 8 - illegal access flag clear for peripheral 72
    #[inline(always)]
    pub fn iaf72(&mut self) -> IAF72_W<'_, ICR2rs> {
        IAF72_W::new(self, 8)
    }
    ///Bit 9 - illegal access flag clear for peripheral 73
    #[inline(always)]
    pub fn iaf73(&mut self) -> IAF73_W<'_, ICR2rs> {
        IAF73_W::new(self, 9)
    }
    ///Bit 10 - illegal access flag clear for peripheral 74
    #[inline(always)]
    pub fn iaf74(&mut self) -> IAF74_W<'_, ICR2rs> {
        IAF74_W::new(self, 10)
    }
    ///Bit 11 - illegal access flag clear for peripheral 75
    #[inline(always)]
    pub fn iaf75(&mut self) -> IAF75_W<'_, ICR2rs> {
        IAF75_W::new(self, 11)
    }
    ///Bit 12 - illegal access flag clear for peripheral 76
    #[inline(always)]
    pub fn iaf76(&mut self) -> IAF76_W<'_, ICR2rs> {
        IAF76_W::new(self, 12)
    }
    ///Bit 13 - illegal access flag clear for peripheral 77
    #[inline(always)]
    pub fn iaf77(&mut self) -> IAF77_W<'_, ICR2rs> {
        IAF77_W::new(self, 13)
    }
    ///Bit 14 - illegal access flag clear for peripheral 78
    #[inline(always)]
    pub fn iaf78(&mut self) -> IAF78_W<'_, ICR2rs> {
        IAF78_W::new(self, 14)
    }
    ///Bit 15 - illegal access flag clear for peripheral 79
    #[inline(always)]
    pub fn iaf79(&mut self) -> IAF79_W<'_, ICR2rs> {
        IAF79_W::new(self, 15)
    }
    ///Bit 16 - illegal access flag clear for peripheral 80
    #[inline(always)]
    pub fn iaf80(&mut self) -> IAF80_W<'_, ICR2rs> {
        IAF80_W::new(self, 16)
    }
    ///Bit 17 - illegal access flag clear for peripheral 81
    #[inline(always)]
    pub fn iaf81(&mut self) -> IAF81_W<'_, ICR2rs> {
        IAF81_W::new(self, 17)
    }
    ///Bit 18 - illegal access flag clear for peripheral 82
    #[inline(always)]
    pub fn iaf82(&mut self) -> IAF82_W<'_, ICR2rs> {
        IAF82_W::new(self, 18)
    }
    ///Bit 19 - illegal access flag clear for peripheral 83
    #[inline(always)]
    pub fn iaf83(&mut self) -> IAF83_W<'_, ICR2rs> {
        IAF83_W::new(self, 19)
    }
    ///Bit 20 - illegal access flag clear for peripheral 84
    #[inline(always)]
    pub fn iaf84(&mut self) -> IAF84_W<'_, ICR2rs> {
        IAF84_W::new(self, 20)
    }
    ///Bit 21 - illegal access flag clear for peripheral 85
    #[inline(always)]
    pub fn iaf85(&mut self) -> IAF85_W<'_, ICR2rs> {
        IAF85_W::new(self, 21)
    }
    ///Bit 22 - illegal access flag clear for peripheral 86
    #[inline(always)]
    pub fn iaf86(&mut self) -> IAF86_W<'_, ICR2rs> {
        IAF86_W::new(self, 22)
    }
    ///Bit 23 - illegal access flag clear for peripheral 87
    #[inline(always)]
    pub fn iaf87(&mut self) -> IAF87_W<'_, ICR2rs> {
        IAF87_W::new(self, 23)
    }
    ///Bit 24 - illegal access flag clear for peripheral 88
    #[inline(always)]
    pub fn iaf88(&mut self) -> IAF88_W<'_, ICR2rs> {
        IAF88_W::new(self, 24)
    }
    ///Bit 25 - illegal access flag clear for peripheral 89
    #[inline(always)]
    pub fn iaf89(&mut self) -> IAF89_W<'_, ICR2rs> {
        IAF89_W::new(self, 25)
    }
    ///Bit 26 - illegal access flag clear for peripheral 90
    #[inline(always)]
    pub fn iaf90(&mut self) -> IAF90_W<'_, ICR2rs> {
        IAF90_W::new(self, 26)
    }
    ///Bit 27 - illegal access flag clear for peripheral 91
    #[inline(always)]
    pub fn iaf91(&mut self) -> IAF91_W<'_, ICR2rs> {
        IAF91_W::new(self, 27)
    }
    ///Bit 28 - illegal access flag clear for peripheral 92
    #[inline(always)]
    pub fn iaf92(&mut self) -> IAF92_W<'_, ICR2rs> {
        IAF92_W::new(self, 28)
    }
    ///Bit 29 - illegal access flag clear for peripheral 93
    #[inline(always)]
    pub fn iaf93(&mut self) -> IAF93_W<'_, ICR2rs> {
        IAF93_W::new(self, 29)
    }
    ///Bit 30 - illegal access flag clear for peripheral 94
    #[inline(always)]
    pub fn iaf94(&mut self) -> IAF94_W<'_, ICR2rs> {
        IAF94_W::new(self, 30)
    }
    ///Bit 31 - illegal access flag clear for peripheral 95
    #[inline(always)]
    pub fn iaf95(&mut self) -> IAF95_W<'_, ICR2rs> {
        IAF95_W::new(self, 31)
    }
}
/**IAC interrupt clear register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#IAC:ICR2)*/
pub struct ICR2rs;
impl crate::RegisterSpec for ICR2rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr2::W`](W) writer structure
impl crate::Writable for ICR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR2 to value 0
impl crate::Resettable for ICR2rs {}
