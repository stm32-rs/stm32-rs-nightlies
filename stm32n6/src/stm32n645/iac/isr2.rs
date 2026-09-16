///Register `ISR2` reader
pub type R = crate::R<ISR2rs>;
///Field `IAF64` reader - illegal access interrupt enable for peripheral 64
pub type IAF64_R = crate::BitReader;
///Field `IAF65` reader - illegal access interrupt enable for peripheral 65
pub type IAF65_R = crate::BitReader;
///Field `IAF66` reader - illegal access interrupt enable for peripheral 66
pub type IAF66_R = crate::BitReader;
///Field `IAF67` reader - illegal access interrupt enable for peripheral 67
pub type IAF67_R = crate::BitReader;
///Field `IAF68` reader - illegal access interrupt enable for peripheral 68
pub type IAF68_R = crate::BitReader;
///Field `IAF69` reader - illegal access interrupt enable for peripheral 69
pub type IAF69_R = crate::BitReader;
///Field `IAF70` reader - illegal access interrupt enable for peripheral 70
pub type IAF70_R = crate::BitReader;
///Field `IAF71` reader - illegal access interrupt enable for peripheral 71
pub type IAF71_R = crate::BitReader;
///Field `IAF72` reader - illegal access interrupt enable for peripheral 72
pub type IAF72_R = crate::BitReader;
///Field `IAF73` reader - illegal access interrupt enable for peripheral 73
pub type IAF73_R = crate::BitReader;
///Field `IAF74` reader - illegal access interrupt enable for peripheral 74
pub type IAF74_R = crate::BitReader;
///Field `IAF75` reader - illegal access interrupt enable for peripheral 75
pub type IAF75_R = crate::BitReader;
///Field `IAF76` reader - illegal access interrupt enable for peripheral 76
pub type IAF76_R = crate::BitReader;
///Field `IAF77` reader - illegal access interrupt enable for peripheral 77
pub type IAF77_R = crate::BitReader;
///Field `IAF78` reader - illegal access interrupt enable for peripheral 78
pub type IAF78_R = crate::BitReader;
///Field `IAF79` reader - illegal access interrupt enable for peripheral 79
pub type IAF79_R = crate::BitReader;
///Field `IAF80` reader - illegal access interrupt enable for peripheral 80
pub type IAF80_R = crate::BitReader;
///Field `IAF81` reader - illegal access interrupt enable for peripheral 81
pub type IAF81_R = crate::BitReader;
///Field `IAF82` reader - illegal access interrupt enable for peripheral 82
pub type IAF82_R = crate::BitReader;
///Field `IAF83` reader - illegal access interrupt enable for peripheral 83
pub type IAF83_R = crate::BitReader;
///Field `IAF84` reader - illegal access interrupt enable for peripheral 84
pub type IAF84_R = crate::BitReader;
///Field `IAF85` reader - illegal access interrupt enable for peripheral 85
pub type IAF85_R = crate::BitReader;
///Field `IAF86` reader - illegal access interrupt enable for peripheral 86
pub type IAF86_R = crate::BitReader;
///Field `IAF87` reader - illegal access interrupt enable for peripheral 87
pub type IAF87_R = crate::BitReader;
///Field `IAF88` reader - illegal access interrupt enable for peripheral 88
pub type IAF88_R = crate::BitReader;
///Field `IAF89` reader - illegal access interrupt enable for peripheral 89
pub type IAF89_R = crate::BitReader;
///Field `IAF90` reader - illegal access interrupt enable for peripheral 90
pub type IAF90_R = crate::BitReader;
///Field `IAF91` reader - illegal access interrupt enable for peripheral 91
pub type IAF91_R = crate::BitReader;
///Field `IAF92` reader - illegal access interrupt enable for peripheral 92
pub type IAF92_R = crate::BitReader;
///Field `IAF93` reader - illegal access interrupt enable for peripheral 93
pub type IAF93_R = crate::BitReader;
///Field `IAF94` reader - illegal access interrupt enable for peripheral 94
pub type IAF94_R = crate::BitReader;
///Field `IAF95` reader - illegal access interrupt enable for peripheral 95
pub type IAF95_R = crate::BitReader;
impl R {
    ///Bit 0 - illegal access interrupt enable for peripheral 64
    #[inline(always)]
    pub fn iaf64(&self) -> IAF64_R {
        IAF64_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access interrupt enable for peripheral 65
    #[inline(always)]
    pub fn iaf65(&self) -> IAF65_R {
        IAF65_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access interrupt enable for peripheral 66
    #[inline(always)]
    pub fn iaf66(&self) -> IAF66_R {
        IAF66_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access interrupt enable for peripheral 67
    #[inline(always)]
    pub fn iaf67(&self) -> IAF67_R {
        IAF67_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access interrupt enable for peripheral 68
    #[inline(always)]
    pub fn iaf68(&self) -> IAF68_R {
        IAF68_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access interrupt enable for peripheral 69
    #[inline(always)]
    pub fn iaf69(&self) -> IAF69_R {
        IAF69_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access interrupt enable for peripheral 70
    #[inline(always)]
    pub fn iaf70(&self) -> IAF70_R {
        IAF70_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access interrupt enable for peripheral 71
    #[inline(always)]
    pub fn iaf71(&self) -> IAF71_R {
        IAF71_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access interrupt enable for peripheral 72
    #[inline(always)]
    pub fn iaf72(&self) -> IAF72_R {
        IAF72_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access interrupt enable for peripheral 73
    #[inline(always)]
    pub fn iaf73(&self) -> IAF73_R {
        IAF73_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access interrupt enable for peripheral 74
    #[inline(always)]
    pub fn iaf74(&self) -> IAF74_R {
        IAF74_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access interrupt enable for peripheral 75
    #[inline(always)]
    pub fn iaf75(&self) -> IAF75_R {
        IAF75_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access interrupt enable for peripheral 76
    #[inline(always)]
    pub fn iaf76(&self) -> IAF76_R {
        IAF76_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access interrupt enable for peripheral 77
    #[inline(always)]
    pub fn iaf77(&self) -> IAF77_R {
        IAF77_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access interrupt enable for peripheral 78
    #[inline(always)]
    pub fn iaf78(&self) -> IAF78_R {
        IAF78_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access interrupt enable for peripheral 79
    #[inline(always)]
    pub fn iaf79(&self) -> IAF79_R {
        IAF79_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access interrupt enable for peripheral 80
    #[inline(always)]
    pub fn iaf80(&self) -> IAF80_R {
        IAF80_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access interrupt enable for peripheral 81
    #[inline(always)]
    pub fn iaf81(&self) -> IAF81_R {
        IAF81_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access interrupt enable for peripheral 82
    #[inline(always)]
    pub fn iaf82(&self) -> IAF82_R {
        IAF82_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access interrupt enable for peripheral 83
    #[inline(always)]
    pub fn iaf83(&self) -> IAF83_R {
        IAF83_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - illegal access interrupt enable for peripheral 84
    #[inline(always)]
    pub fn iaf84(&self) -> IAF84_R {
        IAF84_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - illegal access interrupt enable for peripheral 85
    #[inline(always)]
    pub fn iaf85(&self) -> IAF85_R {
        IAF85_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - illegal access interrupt enable for peripheral 86
    #[inline(always)]
    pub fn iaf86(&self) -> IAF86_R {
        IAF86_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - illegal access interrupt enable for peripheral 87
    #[inline(always)]
    pub fn iaf87(&self) -> IAF87_R {
        IAF87_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - illegal access interrupt enable for peripheral 88
    #[inline(always)]
    pub fn iaf88(&self) -> IAF88_R {
        IAF88_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access interrupt enable for peripheral 89
    #[inline(always)]
    pub fn iaf89(&self) -> IAF89_R {
        IAF89_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - illegal access interrupt enable for peripheral 90
    #[inline(always)]
    pub fn iaf90(&self) -> IAF90_R {
        IAF90_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - illegal access interrupt enable for peripheral 91
    #[inline(always)]
    pub fn iaf91(&self) -> IAF91_R {
        IAF91_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - illegal access interrupt enable for peripheral 92
    #[inline(always)]
    pub fn iaf92(&self) -> IAF92_R {
        IAF92_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - illegal access interrupt enable for peripheral 93
    #[inline(always)]
    pub fn iaf93(&self) -> IAF93_R {
        IAF93_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - illegal access interrupt enable for peripheral 94
    #[inline(always)]
    pub fn iaf94(&self) -> IAF94_R {
        IAF94_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - illegal access interrupt enable for peripheral 95
    #[inline(always)]
    pub fn iaf95(&self) -> IAF95_R {
        IAF95_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR2")
            .field("iaf64", &self.iaf64())
            .field("iaf65", &self.iaf65())
            .field("iaf66", &self.iaf66())
            .field("iaf67", &self.iaf67())
            .field("iaf68", &self.iaf68())
            .field("iaf69", &self.iaf69())
            .field("iaf70", &self.iaf70())
            .field("iaf71", &self.iaf71())
            .field("iaf72", &self.iaf72())
            .field("iaf73", &self.iaf73())
            .field("iaf74", &self.iaf74())
            .field("iaf75", &self.iaf75())
            .field("iaf76", &self.iaf76())
            .field("iaf77", &self.iaf77())
            .field("iaf78", &self.iaf78())
            .field("iaf79", &self.iaf79())
            .field("iaf80", &self.iaf80())
            .field("iaf81", &self.iaf81())
            .field("iaf82", &self.iaf82())
            .field("iaf83", &self.iaf83())
            .field("iaf84", &self.iaf84())
            .field("iaf85", &self.iaf85())
            .field("iaf86", &self.iaf86())
            .field("iaf87", &self.iaf87())
            .field("iaf88", &self.iaf88())
            .field("iaf89", &self.iaf89())
            .field("iaf90", &self.iaf90())
            .field("iaf91", &self.iaf91())
            .field("iaf92", &self.iaf92())
            .field("iaf93", &self.iaf93())
            .field("iaf94", &self.iaf94())
            .field("iaf95", &self.iaf95())
            .finish()
    }
}
/**IAC interrupt status register 2

You can [`read`](crate::Reg::read) this register and get [`isr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#IAC:ISR2)*/
pub struct ISR2rs;
impl crate::RegisterSpec for ISR2rs {
    type Ux = u32;
}
///`read()` method returns [`isr2::R`](R) reader structure
impl crate::Readable for ISR2rs {}
///`reset()` method sets ISR2 to value 0
impl crate::Resettable for ISR2rs {}
