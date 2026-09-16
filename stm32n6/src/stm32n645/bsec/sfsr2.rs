///Register `SFSR2` reader
pub type R = crate::R<SFSR2rs>;
///Field `SFW64` reader - Shadowed fuse word 64
pub type SFW64_R = crate::BitReader;
///Field `SFW65` reader - Shadowed fuse word 65
pub type SFW65_R = crate::BitReader;
///Field `SFW66` reader - Shadowed fuse word 66
pub type SFW66_R = crate::BitReader;
///Field `SFW67` reader - Shadowed fuse word 67
pub type SFW67_R = crate::BitReader;
///Field `SFW68` reader - Shadowed fuse word 68
pub type SFW68_R = crate::BitReader;
///Field `SFW69` reader - Shadowed fuse word 69
pub type SFW69_R = crate::BitReader;
///Field `SFW70` reader - Shadowed fuse word 70
pub type SFW70_R = crate::BitReader;
///Field `SFW71` reader - Shadowed fuse word 71
pub type SFW71_R = crate::BitReader;
///Field `SFW72` reader - Shadowed fuse word 72
pub type SFW72_R = crate::BitReader;
///Field `SFW73` reader - Shadowed fuse word 73
pub type SFW73_R = crate::BitReader;
///Field `SFW74` reader - Shadowed fuse word 74
pub type SFW74_R = crate::BitReader;
///Field `SFW75` reader - Shadowed fuse word 75
pub type SFW75_R = crate::BitReader;
///Field `SFW76` reader - Shadowed fuse word 76
pub type SFW76_R = crate::BitReader;
///Field `SFW77` reader - Shadowed fuse word 77
pub type SFW77_R = crate::BitReader;
///Field `SFW78` reader - Shadowed fuse word 78
pub type SFW78_R = crate::BitReader;
///Field `SFW79` reader - Shadowed fuse word 79
pub type SFW79_R = crate::BitReader;
///Field `SFW80` reader - Shadowed fuse word 80
pub type SFW80_R = crate::BitReader;
///Field `SFW81` reader - Shadowed fuse word 81
pub type SFW81_R = crate::BitReader;
///Field `SFW82` reader - Shadowed fuse word 82
pub type SFW82_R = crate::BitReader;
///Field `SFW83` reader - Shadowed fuse word 83
pub type SFW83_R = crate::BitReader;
///Field `SFW84` reader - Shadowed fuse word 84
pub type SFW84_R = crate::BitReader;
///Field `SFW85` reader - Shadowed fuse word 85
pub type SFW85_R = crate::BitReader;
///Field `SFW86` reader - Shadowed fuse word 86
pub type SFW86_R = crate::BitReader;
///Field `SFW87` reader - Shadowed fuse word 87
pub type SFW87_R = crate::BitReader;
///Field `SFW88` reader - Shadowed fuse word 88
pub type SFW88_R = crate::BitReader;
///Field `SFW89` reader - Shadowed fuse word 89
pub type SFW89_R = crate::BitReader;
///Field `SFW90` reader - Shadowed fuse word 90
pub type SFW90_R = crate::BitReader;
///Field `SFW91` reader - Shadowed fuse word 91
pub type SFW91_R = crate::BitReader;
///Field `SFW92` reader - Shadowed fuse word 92
pub type SFW92_R = crate::BitReader;
///Field `SFW93` reader - Shadowed fuse word 93
pub type SFW93_R = crate::BitReader;
///Field `SFW94` reader - Shadowed fuse word 94
pub type SFW94_R = crate::BitReader;
///Field `SFW95` reader - Shadowed fuse word 95
pub type SFW95_R = crate::BitReader;
impl R {
    ///Bit 0 - Shadowed fuse word 64
    #[inline(always)]
    pub fn sfw64(&self) -> SFW64_R {
        SFW64_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Shadowed fuse word 65
    #[inline(always)]
    pub fn sfw65(&self) -> SFW65_R {
        SFW65_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Shadowed fuse word 66
    #[inline(always)]
    pub fn sfw66(&self) -> SFW66_R {
        SFW66_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Shadowed fuse word 67
    #[inline(always)]
    pub fn sfw67(&self) -> SFW67_R {
        SFW67_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Shadowed fuse word 68
    #[inline(always)]
    pub fn sfw68(&self) -> SFW68_R {
        SFW68_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Shadowed fuse word 69
    #[inline(always)]
    pub fn sfw69(&self) -> SFW69_R {
        SFW69_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Shadowed fuse word 70
    #[inline(always)]
    pub fn sfw70(&self) -> SFW70_R {
        SFW70_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Shadowed fuse word 71
    #[inline(always)]
    pub fn sfw71(&self) -> SFW71_R {
        SFW71_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Shadowed fuse word 72
    #[inline(always)]
    pub fn sfw72(&self) -> SFW72_R {
        SFW72_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Shadowed fuse word 73
    #[inline(always)]
    pub fn sfw73(&self) -> SFW73_R {
        SFW73_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Shadowed fuse word 74
    #[inline(always)]
    pub fn sfw74(&self) -> SFW74_R {
        SFW74_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Shadowed fuse word 75
    #[inline(always)]
    pub fn sfw75(&self) -> SFW75_R {
        SFW75_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Shadowed fuse word 76
    #[inline(always)]
    pub fn sfw76(&self) -> SFW76_R {
        SFW76_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Shadowed fuse word 77
    #[inline(always)]
    pub fn sfw77(&self) -> SFW77_R {
        SFW77_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Shadowed fuse word 78
    #[inline(always)]
    pub fn sfw78(&self) -> SFW78_R {
        SFW78_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Shadowed fuse word 79
    #[inline(always)]
    pub fn sfw79(&self) -> SFW79_R {
        SFW79_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Shadowed fuse word 80
    #[inline(always)]
    pub fn sfw80(&self) -> SFW80_R {
        SFW80_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Shadowed fuse word 81
    #[inline(always)]
    pub fn sfw81(&self) -> SFW81_R {
        SFW81_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Shadowed fuse word 82
    #[inline(always)]
    pub fn sfw82(&self) -> SFW82_R {
        SFW82_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Shadowed fuse word 83
    #[inline(always)]
    pub fn sfw83(&self) -> SFW83_R {
        SFW83_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Shadowed fuse word 84
    #[inline(always)]
    pub fn sfw84(&self) -> SFW84_R {
        SFW84_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Shadowed fuse word 85
    #[inline(always)]
    pub fn sfw85(&self) -> SFW85_R {
        SFW85_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Shadowed fuse word 86
    #[inline(always)]
    pub fn sfw86(&self) -> SFW86_R {
        SFW86_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Shadowed fuse word 87
    #[inline(always)]
    pub fn sfw87(&self) -> SFW87_R {
        SFW87_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Shadowed fuse word 88
    #[inline(always)]
    pub fn sfw88(&self) -> SFW88_R {
        SFW88_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Shadowed fuse word 89
    #[inline(always)]
    pub fn sfw89(&self) -> SFW89_R {
        SFW89_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Shadowed fuse word 90
    #[inline(always)]
    pub fn sfw90(&self) -> SFW90_R {
        SFW90_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Shadowed fuse word 91
    #[inline(always)]
    pub fn sfw91(&self) -> SFW91_R {
        SFW91_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Shadowed fuse word 92
    #[inline(always)]
    pub fn sfw92(&self) -> SFW92_R {
        SFW92_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Shadowed fuse word 93
    #[inline(always)]
    pub fn sfw93(&self) -> SFW93_R {
        SFW93_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Shadowed fuse word 94
    #[inline(always)]
    pub fn sfw94(&self) -> SFW94_R {
        SFW94_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Shadowed fuse word 95
    #[inline(always)]
    pub fn sfw95(&self) -> SFW95_R {
        SFW95_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFSR2")
            .field("sfw64", &self.sfw64())
            .field("sfw65", &self.sfw65())
            .field("sfw66", &self.sfw66())
            .field("sfw67", &self.sfw67())
            .field("sfw68", &self.sfw68())
            .field("sfw69", &self.sfw69())
            .field("sfw70", &self.sfw70())
            .field("sfw71", &self.sfw71())
            .field("sfw72", &self.sfw72())
            .field("sfw73", &self.sfw73())
            .field("sfw74", &self.sfw74())
            .field("sfw75", &self.sfw75())
            .field("sfw76", &self.sfw76())
            .field("sfw77", &self.sfw77())
            .field("sfw78", &self.sfw78())
            .field("sfw79", &self.sfw79())
            .field("sfw80", &self.sfw80())
            .field("sfw81", &self.sfw81())
            .field("sfw82", &self.sfw82())
            .field("sfw83", &self.sfw83())
            .field("sfw84", &self.sfw84())
            .field("sfw85", &self.sfw85())
            .field("sfw86", &self.sfw86())
            .field("sfw87", &self.sfw87())
            .field("sfw88", &self.sfw88())
            .field("sfw89", &self.sfw89())
            .field("sfw90", &self.sfw90())
            .field("sfw91", &self.sfw91())
            .field("sfw92", &self.sfw92())
            .field("sfw93", &self.sfw93())
            .field("sfw94", &self.sfw94())
            .field("sfw95", &self.sfw95())
            .finish()
    }
}
/**BSEC shadowed fuses status register 2

You can [`read`](crate::Reg::read) this register and get [`sfsr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SFSR2)*/
pub struct SFSR2rs;
impl crate::RegisterSpec for SFSR2rs {
    type Ux = u32;
}
///`read()` method returns [`sfsr2::R`](R) reader structure
impl crate::Readable for SFSR2rs {}
///`reset()` method sets SFSR2 to value 0
impl crate::Resettable for SFSR2rs {}
