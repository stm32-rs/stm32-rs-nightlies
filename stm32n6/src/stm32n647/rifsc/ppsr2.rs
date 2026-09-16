///Register `PPSR2` reader
pub type R = crate::R<PPSR2rs>;
///Field `PPEN64` reader - peripheral protection enable 64
pub type PPEN64_R = crate::BitReader;
///Field `PPEN65` reader - peripheral protection enable 65
pub type PPEN65_R = crate::BitReader;
///Field `PPEN66` reader - peripheral protection enable 66
pub type PPEN66_R = crate::BitReader;
///Field `PPEN67` reader - peripheral protection enable 67
pub type PPEN67_R = crate::BitReader;
///Field `PPEN68` reader - peripheral protection enable 68
pub type PPEN68_R = crate::BitReader;
///Field `PPEN69` reader - peripheral protection enable 69
pub type PPEN69_R = crate::BitReader;
///Field `PPEN70` reader - peripheral protection enable 70
pub type PPEN70_R = crate::BitReader;
///Field `PPEN71` reader - peripheral protection enable 71
pub type PPEN71_R = crate::BitReader;
///Field `PPEN72` reader - peripheral protection enable 72
pub type PPEN72_R = crate::BitReader;
///Field `PPEN73` reader - peripheral protection enable 73
pub type PPEN73_R = crate::BitReader;
///Field `PPEN74` reader - peripheral protection enable 74
pub type PPEN74_R = crate::BitReader;
///Field `PPEN75` reader - peripheral protection enable 75
pub type PPEN75_R = crate::BitReader;
///Field `PPEN76` reader - peripheral protection enable 76
pub type PPEN76_R = crate::BitReader;
///Field `PPEN77` reader - peripheral protection enable 77
pub type PPEN77_R = crate::BitReader;
///Field `PPEN78` reader - peripheral protection enable 78
pub type PPEN78_R = crate::BitReader;
///Field `PPEN79` reader - peripheral protection enable 79
pub type PPEN79_R = crate::BitReader;
///Field `PPEN80` reader - peripheral protection enable 80
pub type PPEN80_R = crate::BitReader;
///Field `PPEN81` reader - peripheral protection enable 81
pub type PPEN81_R = crate::BitReader;
///Field `PPEN82` reader - peripheral protection enable 82
pub type PPEN82_R = crate::BitReader;
///Field `PPEN83` reader - peripheral protection enable 83
pub type PPEN83_R = crate::BitReader;
///Field `PPEN84` reader - peripheral protection enable 84
pub type PPEN84_R = crate::BitReader;
///Field `PPEN85` reader - peripheral protection enable 85
pub type PPEN85_R = crate::BitReader;
///Field `PPEN86` reader - peripheral protection enable 86
pub type PPEN86_R = crate::BitReader;
///Field `PPEN87` reader - peripheral protection enable 87
pub type PPEN87_R = crate::BitReader;
///Field `PPEN88` reader - peripheral protection enable 88
pub type PPEN88_R = crate::BitReader;
///Field `PPEN89` reader - peripheral protection enable 89
pub type PPEN89_R = crate::BitReader;
///Field `PPEN90` reader - peripheral protection enable 90
pub type PPEN90_R = crate::BitReader;
///Field `PPEN91` reader - peripheral protection enable 91
pub type PPEN91_R = crate::BitReader;
///Field `PPEN92` reader - peripheral protection enable 92
pub type PPEN92_R = crate::BitReader;
///Field `PPEN93` reader - peripheral protection enable 93
pub type PPEN93_R = crate::BitReader;
///Field `PPEN94` reader - peripheral protection enable 94
pub type PPEN94_R = crate::BitReader;
///Field `PPEN95` reader - peripheral protection enable 95
pub type PPEN95_R = crate::BitReader;
impl R {
    ///Bit 0 - peripheral protection enable 64
    #[inline(always)]
    pub fn ppen64(&self) -> PPEN64_R {
        PPEN64_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - peripheral protection enable 65
    #[inline(always)]
    pub fn ppen65(&self) -> PPEN65_R {
        PPEN65_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - peripheral protection enable 66
    #[inline(always)]
    pub fn ppen66(&self) -> PPEN66_R {
        PPEN66_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - peripheral protection enable 67
    #[inline(always)]
    pub fn ppen67(&self) -> PPEN67_R {
        PPEN67_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - peripheral protection enable 68
    #[inline(always)]
    pub fn ppen68(&self) -> PPEN68_R {
        PPEN68_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - peripheral protection enable 69
    #[inline(always)]
    pub fn ppen69(&self) -> PPEN69_R {
        PPEN69_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - peripheral protection enable 70
    #[inline(always)]
    pub fn ppen70(&self) -> PPEN70_R {
        PPEN70_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - peripheral protection enable 71
    #[inline(always)]
    pub fn ppen71(&self) -> PPEN71_R {
        PPEN71_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - peripheral protection enable 72
    #[inline(always)]
    pub fn ppen72(&self) -> PPEN72_R {
        PPEN72_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - peripheral protection enable 73
    #[inline(always)]
    pub fn ppen73(&self) -> PPEN73_R {
        PPEN73_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - peripheral protection enable 74
    #[inline(always)]
    pub fn ppen74(&self) -> PPEN74_R {
        PPEN74_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - peripheral protection enable 75
    #[inline(always)]
    pub fn ppen75(&self) -> PPEN75_R {
        PPEN75_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - peripheral protection enable 76
    #[inline(always)]
    pub fn ppen76(&self) -> PPEN76_R {
        PPEN76_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - peripheral protection enable 77
    #[inline(always)]
    pub fn ppen77(&self) -> PPEN77_R {
        PPEN77_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - peripheral protection enable 78
    #[inline(always)]
    pub fn ppen78(&self) -> PPEN78_R {
        PPEN78_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - peripheral protection enable 79
    #[inline(always)]
    pub fn ppen79(&self) -> PPEN79_R {
        PPEN79_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - peripheral protection enable 80
    #[inline(always)]
    pub fn ppen80(&self) -> PPEN80_R {
        PPEN80_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - peripheral protection enable 81
    #[inline(always)]
    pub fn ppen81(&self) -> PPEN81_R {
        PPEN81_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - peripheral protection enable 82
    #[inline(always)]
    pub fn ppen82(&self) -> PPEN82_R {
        PPEN82_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - peripheral protection enable 83
    #[inline(always)]
    pub fn ppen83(&self) -> PPEN83_R {
        PPEN83_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - peripheral protection enable 84
    #[inline(always)]
    pub fn ppen84(&self) -> PPEN84_R {
        PPEN84_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - peripheral protection enable 85
    #[inline(always)]
    pub fn ppen85(&self) -> PPEN85_R {
        PPEN85_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - peripheral protection enable 86
    #[inline(always)]
    pub fn ppen86(&self) -> PPEN86_R {
        PPEN86_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - peripheral protection enable 87
    #[inline(always)]
    pub fn ppen87(&self) -> PPEN87_R {
        PPEN87_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - peripheral protection enable 88
    #[inline(always)]
    pub fn ppen88(&self) -> PPEN88_R {
        PPEN88_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - peripheral protection enable 89
    #[inline(always)]
    pub fn ppen89(&self) -> PPEN89_R {
        PPEN89_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - peripheral protection enable 90
    #[inline(always)]
    pub fn ppen90(&self) -> PPEN90_R {
        PPEN90_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - peripheral protection enable 91
    #[inline(always)]
    pub fn ppen91(&self) -> PPEN91_R {
        PPEN91_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - peripheral protection enable 92
    #[inline(always)]
    pub fn ppen92(&self) -> PPEN92_R {
        PPEN92_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - peripheral protection enable 93
    #[inline(always)]
    pub fn ppen93(&self) -> PPEN93_R {
        PPEN93_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - peripheral protection enable 94
    #[inline(always)]
    pub fn ppen94(&self) -> PPEN94_R {
        PPEN94_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - peripheral protection enable 95
    #[inline(always)]
    pub fn ppen95(&self) -> PPEN95_R {
        PPEN95_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPSR2")
            .field("ppen64", &self.ppen64())
            .field("ppen65", &self.ppen65())
            .field("ppen66", &self.ppen66())
            .field("ppen67", &self.ppen67())
            .field("ppen68", &self.ppen68())
            .field("ppen69", &self.ppen69())
            .field("ppen70", &self.ppen70())
            .field("ppen71", &self.ppen71())
            .field("ppen72", &self.ppen72())
            .field("ppen73", &self.ppen73())
            .field("ppen74", &self.ppen74())
            .field("ppen75", &self.ppen75())
            .field("ppen76", &self.ppen76())
            .field("ppen77", &self.ppen77())
            .field("ppen78", &self.ppen78())
            .field("ppen79", &self.ppen79())
            .field("ppen80", &self.ppen80())
            .field("ppen81", &self.ppen81())
            .field("ppen82", &self.ppen82())
            .field("ppen83", &self.ppen83())
            .field("ppen84", &self.ppen84())
            .field("ppen85", &self.ppen85())
            .field("ppen86", &self.ppen86())
            .field("ppen87", &self.ppen87())
            .field("ppen88", &self.ppen88())
            .field("ppen89", &self.ppen89())
            .field("ppen90", &self.ppen90())
            .field("ppen91", &self.ppen91())
            .field("ppen92", &self.ppen92())
            .field("ppen93", &self.ppen93())
            .field("ppen94", &self.ppen94())
            .field("ppen95", &self.ppen95())
            .finish()
    }
}
/**RIFSC peripheral protection status register 2

You can [`read`](crate::Reg::read) this register and get [`ppsr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RIFSC:PPSR2)*/
pub struct PPSR2rs;
impl crate::RegisterSpec for PPSR2rs {
    type Ux = u32;
}
///`read()` method returns [`ppsr2::R`](R) reader structure
impl crate::Readable for PPSR2rs {}
///`reset()` method sets PPSR2 to value 0xf7df_f03b
impl crate::Resettable for PPSR2rs {
    const RESET_VALUE: u32 = 0xf7df_f03b;
}
