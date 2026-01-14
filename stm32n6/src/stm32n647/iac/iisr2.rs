///Register `IISR2` reader
pub type R = crate::R<IISR2rs>;
///Field `ILACIN64` reader - illegal access input 64
pub type ILACIN64_R = crate::BitReader;
///Field `ILACIN65` reader - illegal access input 65
pub type ILACIN65_R = crate::BitReader;
///Field `ILACIN66` reader - illegal access input 66
pub type ILACIN66_R = crate::BitReader;
///Field `ILACIN67` reader - illegal access input 67
pub type ILACIN67_R = crate::BitReader;
///Field `ILACIN68` reader - illegal access input 68
pub type ILACIN68_R = crate::BitReader;
///Field `ILACIN69` reader - illegal access input 69
pub type ILACIN69_R = crate::BitReader;
///Field `ILACIN70` reader - illegal access input 70
pub type ILACIN70_R = crate::BitReader;
///Field `ILACIN71` reader - illegal access input 71
pub type ILACIN71_R = crate::BitReader;
///Field `ILACIN72` reader - illegal access input 72
pub type ILACIN72_R = crate::BitReader;
///Field `ILACIN73` reader - illegal access input 73
pub type ILACIN73_R = crate::BitReader;
///Field `ILACIN74` reader - illegal access input 74
pub type ILACIN74_R = crate::BitReader;
///Field `ILACIN75` reader - illegal access input 75
pub type ILACIN75_R = crate::BitReader;
///Field `ILACIN76` reader - illegal access input 76
pub type ILACIN76_R = crate::BitReader;
///Field `ILACIN77` reader - illegal access input 77
pub type ILACIN77_R = crate::BitReader;
///Field `ILACIN78` reader - illegal access input 78
pub type ILACIN78_R = crate::BitReader;
///Field `ILACIN79` reader - illegal access input 79
pub type ILACIN79_R = crate::BitReader;
///Field `ILACIN80` reader - illegal access input 80
pub type ILACIN80_R = crate::BitReader;
///Field `ILACIN81` reader - illegal access input 81
pub type ILACIN81_R = crate::BitReader;
///Field `ILACIN82` reader - illegal access input 82
pub type ILACIN82_R = crate::BitReader;
///Field `ILACIN83` reader - illegal access input 83
pub type ILACIN83_R = crate::BitReader;
///Field `ILACIN84` reader - illegal access input 84
pub type ILACIN84_R = crate::BitReader;
///Field `ILACIN85` reader - illegal access input 85
pub type ILACIN85_R = crate::BitReader;
///Field `ILACIN86` reader - illegal access input 86
pub type ILACIN86_R = crate::BitReader;
///Field `ILACIN87` reader - illegal access input 87
pub type ILACIN87_R = crate::BitReader;
///Field `ILACIN88` reader - illegal access input 88
pub type ILACIN88_R = crate::BitReader;
///Field `ILACIN89` reader - illegal access input 89
pub type ILACIN89_R = crate::BitReader;
///Field `ILACIN90` reader - illegal access input 90
pub type ILACIN90_R = crate::BitReader;
///Field `ILACIN91` reader - illegal access input 91
pub type ILACIN91_R = crate::BitReader;
///Field `ILACIN92` reader - illegal access input 92
pub type ILACIN92_R = crate::BitReader;
///Field `ILACIN93` reader - illegal access input 93
pub type ILACIN93_R = crate::BitReader;
///Field `ILACIN94` reader - illegal access input 94
pub type ILACIN94_R = crate::BitReader;
///Field `ILACIN95` reader - illegal access input 95
pub type ILACIN95_R = crate::BitReader;
impl R {
    ///Bit 0 - illegal access input 64
    #[inline(always)]
    pub fn ilacin64(&self) -> ILACIN64_R {
        ILACIN64_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access input 65
    #[inline(always)]
    pub fn ilacin65(&self) -> ILACIN65_R {
        ILACIN65_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access input 66
    #[inline(always)]
    pub fn ilacin66(&self) -> ILACIN66_R {
        ILACIN66_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access input 67
    #[inline(always)]
    pub fn ilacin67(&self) -> ILACIN67_R {
        ILACIN67_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access input 68
    #[inline(always)]
    pub fn ilacin68(&self) -> ILACIN68_R {
        ILACIN68_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access input 69
    #[inline(always)]
    pub fn ilacin69(&self) -> ILACIN69_R {
        ILACIN69_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access input 70
    #[inline(always)]
    pub fn ilacin70(&self) -> ILACIN70_R {
        ILACIN70_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access input 71
    #[inline(always)]
    pub fn ilacin71(&self) -> ILACIN71_R {
        ILACIN71_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access input 72
    #[inline(always)]
    pub fn ilacin72(&self) -> ILACIN72_R {
        ILACIN72_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access input 73
    #[inline(always)]
    pub fn ilacin73(&self) -> ILACIN73_R {
        ILACIN73_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access input 74
    #[inline(always)]
    pub fn ilacin74(&self) -> ILACIN74_R {
        ILACIN74_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access input 75
    #[inline(always)]
    pub fn ilacin75(&self) -> ILACIN75_R {
        ILACIN75_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access input 76
    #[inline(always)]
    pub fn ilacin76(&self) -> ILACIN76_R {
        ILACIN76_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access input 77
    #[inline(always)]
    pub fn ilacin77(&self) -> ILACIN77_R {
        ILACIN77_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access input 78
    #[inline(always)]
    pub fn ilacin78(&self) -> ILACIN78_R {
        ILACIN78_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access input 79
    #[inline(always)]
    pub fn ilacin79(&self) -> ILACIN79_R {
        ILACIN79_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access input 80
    #[inline(always)]
    pub fn ilacin80(&self) -> ILACIN80_R {
        ILACIN80_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access input 81
    #[inline(always)]
    pub fn ilacin81(&self) -> ILACIN81_R {
        ILACIN81_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access input 82
    #[inline(always)]
    pub fn ilacin82(&self) -> ILACIN82_R {
        ILACIN82_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access input 83
    #[inline(always)]
    pub fn ilacin83(&self) -> ILACIN83_R {
        ILACIN83_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - illegal access input 84
    #[inline(always)]
    pub fn ilacin84(&self) -> ILACIN84_R {
        ILACIN84_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - illegal access input 85
    #[inline(always)]
    pub fn ilacin85(&self) -> ILACIN85_R {
        ILACIN85_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - illegal access input 86
    #[inline(always)]
    pub fn ilacin86(&self) -> ILACIN86_R {
        ILACIN86_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - illegal access input 87
    #[inline(always)]
    pub fn ilacin87(&self) -> ILACIN87_R {
        ILACIN87_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - illegal access input 88
    #[inline(always)]
    pub fn ilacin88(&self) -> ILACIN88_R {
        ILACIN88_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access input 89
    #[inline(always)]
    pub fn ilacin89(&self) -> ILACIN89_R {
        ILACIN89_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - illegal access input 90
    #[inline(always)]
    pub fn ilacin90(&self) -> ILACIN90_R {
        ILACIN90_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - illegal access input 91
    #[inline(always)]
    pub fn ilacin91(&self) -> ILACIN91_R {
        ILACIN91_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - illegal access input 92
    #[inline(always)]
    pub fn ilacin92(&self) -> ILACIN92_R {
        ILACIN92_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - illegal access input 93
    #[inline(always)]
    pub fn ilacin93(&self) -> ILACIN93_R {
        ILACIN93_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - illegal access input 94
    #[inline(always)]
    pub fn ilacin94(&self) -> ILACIN94_R {
        ILACIN94_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - illegal access input 95
    #[inline(always)]
    pub fn ilacin95(&self) -> ILACIN95_R {
        ILACIN95_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IISR2")
            .field("ilacin64", &self.ilacin64())
            .field("ilacin65", &self.ilacin65())
            .field("ilacin66", &self.ilacin66())
            .field("ilacin67", &self.ilacin67())
            .field("ilacin68", &self.ilacin68())
            .field("ilacin69", &self.ilacin69())
            .field("ilacin70", &self.ilacin70())
            .field("ilacin71", &self.ilacin71())
            .field("ilacin72", &self.ilacin72())
            .field("ilacin73", &self.ilacin73())
            .field("ilacin74", &self.ilacin74())
            .field("ilacin75", &self.ilacin75())
            .field("ilacin76", &self.ilacin76())
            .field("ilacin77", &self.ilacin77())
            .field("ilacin78", &self.ilacin78())
            .field("ilacin79", &self.ilacin79())
            .field("ilacin80", &self.ilacin80())
            .field("ilacin81", &self.ilacin81())
            .field("ilacin82", &self.ilacin82())
            .field("ilacin83", &self.ilacin83())
            .field("ilacin84", &self.ilacin84())
            .field("ilacin85", &self.ilacin85())
            .field("ilacin86", &self.ilacin86())
            .field("ilacin87", &self.ilacin87())
            .field("ilacin88", &self.ilacin88())
            .field("ilacin89", &self.ilacin89())
            .field("ilacin90", &self.ilacin90())
            .field("ilacin91", &self.ilacin91())
            .field("ilacin92", &self.ilacin92())
            .field("ilacin93", &self.ilacin93())
            .field("ilacin94", &self.ilacin94())
            .field("ilacin95", &self.ilacin95())
            .finish()
    }
}
/**IAC ILAC input status register 2

You can [`read`](crate::Reg::read) this register and get [`iisr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#IAC:IISR2)*/
pub struct IISR2rs;
impl crate::RegisterSpec for IISR2rs {
    type Ux = u32;
}
///`read()` method returns [`iisr2::R`](R) reader structure
impl crate::Readable for IISR2rs {}
///`reset()` method sets IISR2 to value 0x77df_f03b
impl crate::Resettable for IISR2rs {
    const RESET_VALUE: u32 = 0x77df_f03b;
}
