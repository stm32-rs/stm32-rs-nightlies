///Register `SFSR3` reader
pub type R = crate::R<SFSR3rs>;
///Field `SFW96` reader - Shadowed fuse word 96
pub type SFW96_R = crate::BitReader;
///Field `SFW97` reader - Shadowed fuse word 97
pub type SFW97_R = crate::BitReader;
///Field `SFW98` reader - Shadowed fuse word 98
pub type SFW98_R = crate::BitReader;
///Field `SFW99` reader - Shadowed fuse word 99
pub type SFW99_R = crate::BitReader;
///Field `SFW100` reader - Shadowed fuse word 100
pub type SFW100_R = crate::BitReader;
///Field `SFW101` reader - Shadowed fuse word 101
pub type SFW101_R = crate::BitReader;
///Field `SFW102` reader - Shadowed fuse word 102
pub type SFW102_R = crate::BitReader;
///Field `SFW103` reader - Shadowed fuse word 103
pub type SFW103_R = crate::BitReader;
///Field `SFW104` reader - Shadowed fuse word 104
pub type SFW104_R = crate::BitReader;
///Field `SFW105` reader - Shadowed fuse word 105
pub type SFW105_R = crate::BitReader;
///Field `SFW106` reader - Shadowed fuse word 106
pub type SFW106_R = crate::BitReader;
///Field `SFW107` reader - Shadowed fuse word 107
pub type SFW107_R = crate::BitReader;
///Field `SFW108` reader - Shadowed fuse word 108
pub type SFW108_R = crate::BitReader;
///Field `SFW109` reader - Shadowed fuse word 109
pub type SFW109_R = crate::BitReader;
///Field `SFW110` reader - Shadowed fuse word 110
pub type SFW110_R = crate::BitReader;
///Field `SFW111` reader - Shadowed fuse word 111
pub type SFW111_R = crate::BitReader;
///Field `SFW112` reader - Shadowed fuse word 112
pub type SFW112_R = crate::BitReader;
///Field `SFW113` reader - Shadowed fuse word 113
pub type SFW113_R = crate::BitReader;
///Field `SFW114` reader - Shadowed fuse word 114
pub type SFW114_R = crate::BitReader;
///Field `SFW115` reader - Shadowed fuse word 115
pub type SFW115_R = crate::BitReader;
///Field `SFW116` reader - Shadowed fuse word 116
pub type SFW116_R = crate::BitReader;
///Field `SFW117` reader - Shadowed fuse word 117
pub type SFW117_R = crate::BitReader;
///Field `SFW118` reader - Shadowed fuse word 118
pub type SFW118_R = crate::BitReader;
///Field `SFW119` reader - Shadowed fuse word 119
pub type SFW119_R = crate::BitReader;
///Field `SFW120` reader - Shadowed fuse word 120
pub type SFW120_R = crate::BitReader;
///Field `SFW121` reader - Shadowed fuse word 121
pub type SFW121_R = crate::BitReader;
///Field `SFW122` reader - Shadowed fuse word 122
pub type SFW122_R = crate::BitReader;
///Field `SFW123` reader - Shadowed fuse word 123
pub type SFW123_R = crate::BitReader;
///Field `SFW124` reader - Shadowed fuse word 124
pub type SFW124_R = crate::BitReader;
///Field `SFW125` reader - Shadowed fuse word 125
pub type SFW125_R = crate::BitReader;
///Field `SFW126` reader - Shadowed fuse word 126
pub type SFW126_R = crate::BitReader;
///Field `SFW127` reader - Shadowed fuse word 127
pub type SFW127_R = crate::BitReader;
impl R {
    ///Bit 0 - Shadowed fuse word 96
    #[inline(always)]
    pub fn sfw96(&self) -> SFW96_R {
        SFW96_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Shadowed fuse word 97
    #[inline(always)]
    pub fn sfw97(&self) -> SFW97_R {
        SFW97_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Shadowed fuse word 98
    #[inline(always)]
    pub fn sfw98(&self) -> SFW98_R {
        SFW98_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Shadowed fuse word 99
    #[inline(always)]
    pub fn sfw99(&self) -> SFW99_R {
        SFW99_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Shadowed fuse word 100
    #[inline(always)]
    pub fn sfw100(&self) -> SFW100_R {
        SFW100_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Shadowed fuse word 101
    #[inline(always)]
    pub fn sfw101(&self) -> SFW101_R {
        SFW101_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Shadowed fuse word 102
    #[inline(always)]
    pub fn sfw102(&self) -> SFW102_R {
        SFW102_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Shadowed fuse word 103
    #[inline(always)]
    pub fn sfw103(&self) -> SFW103_R {
        SFW103_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Shadowed fuse word 104
    #[inline(always)]
    pub fn sfw104(&self) -> SFW104_R {
        SFW104_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Shadowed fuse word 105
    #[inline(always)]
    pub fn sfw105(&self) -> SFW105_R {
        SFW105_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Shadowed fuse word 106
    #[inline(always)]
    pub fn sfw106(&self) -> SFW106_R {
        SFW106_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Shadowed fuse word 107
    #[inline(always)]
    pub fn sfw107(&self) -> SFW107_R {
        SFW107_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Shadowed fuse word 108
    #[inline(always)]
    pub fn sfw108(&self) -> SFW108_R {
        SFW108_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Shadowed fuse word 109
    #[inline(always)]
    pub fn sfw109(&self) -> SFW109_R {
        SFW109_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Shadowed fuse word 110
    #[inline(always)]
    pub fn sfw110(&self) -> SFW110_R {
        SFW110_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Shadowed fuse word 111
    #[inline(always)]
    pub fn sfw111(&self) -> SFW111_R {
        SFW111_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Shadowed fuse word 112
    #[inline(always)]
    pub fn sfw112(&self) -> SFW112_R {
        SFW112_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Shadowed fuse word 113
    #[inline(always)]
    pub fn sfw113(&self) -> SFW113_R {
        SFW113_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Shadowed fuse word 114
    #[inline(always)]
    pub fn sfw114(&self) -> SFW114_R {
        SFW114_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Shadowed fuse word 115
    #[inline(always)]
    pub fn sfw115(&self) -> SFW115_R {
        SFW115_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Shadowed fuse word 116
    #[inline(always)]
    pub fn sfw116(&self) -> SFW116_R {
        SFW116_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Shadowed fuse word 117
    #[inline(always)]
    pub fn sfw117(&self) -> SFW117_R {
        SFW117_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Shadowed fuse word 118
    #[inline(always)]
    pub fn sfw118(&self) -> SFW118_R {
        SFW118_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Shadowed fuse word 119
    #[inline(always)]
    pub fn sfw119(&self) -> SFW119_R {
        SFW119_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Shadowed fuse word 120
    #[inline(always)]
    pub fn sfw120(&self) -> SFW120_R {
        SFW120_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Shadowed fuse word 121
    #[inline(always)]
    pub fn sfw121(&self) -> SFW121_R {
        SFW121_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Shadowed fuse word 122
    #[inline(always)]
    pub fn sfw122(&self) -> SFW122_R {
        SFW122_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Shadowed fuse word 123
    #[inline(always)]
    pub fn sfw123(&self) -> SFW123_R {
        SFW123_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Shadowed fuse word 124
    #[inline(always)]
    pub fn sfw124(&self) -> SFW124_R {
        SFW124_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Shadowed fuse word 125
    #[inline(always)]
    pub fn sfw125(&self) -> SFW125_R {
        SFW125_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Shadowed fuse word 126
    #[inline(always)]
    pub fn sfw126(&self) -> SFW126_R {
        SFW126_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Shadowed fuse word 127
    #[inline(always)]
    pub fn sfw127(&self) -> SFW127_R {
        SFW127_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFSR3")
            .field("sfw96", &self.sfw96())
            .field("sfw97", &self.sfw97())
            .field("sfw98", &self.sfw98())
            .field("sfw99", &self.sfw99())
            .field("sfw100", &self.sfw100())
            .field("sfw101", &self.sfw101())
            .field("sfw102", &self.sfw102())
            .field("sfw103", &self.sfw103())
            .field("sfw104", &self.sfw104())
            .field("sfw105", &self.sfw105())
            .field("sfw106", &self.sfw106())
            .field("sfw107", &self.sfw107())
            .field("sfw108", &self.sfw108())
            .field("sfw109", &self.sfw109())
            .field("sfw110", &self.sfw110())
            .field("sfw111", &self.sfw111())
            .field("sfw112", &self.sfw112())
            .field("sfw113", &self.sfw113())
            .field("sfw114", &self.sfw114())
            .field("sfw115", &self.sfw115())
            .field("sfw116", &self.sfw116())
            .field("sfw117", &self.sfw117())
            .field("sfw118", &self.sfw118())
            .field("sfw119", &self.sfw119())
            .field("sfw120", &self.sfw120())
            .field("sfw121", &self.sfw121())
            .field("sfw122", &self.sfw122())
            .field("sfw123", &self.sfw123())
            .field("sfw124", &self.sfw124())
            .field("sfw125", &self.sfw125())
            .field("sfw126", &self.sfw126())
            .field("sfw127", &self.sfw127())
            .finish()
    }
}
/**BSEC shadowed fuses status register 3

You can [`read`](crate::Reg::read) this register and get [`sfsr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SFSR3)*/
pub struct SFSR3rs;
impl crate::RegisterSpec for SFSR3rs {
    type Ux = u32;
}
///`read()` method returns [`sfsr3::R`](R) reader structure
impl crate::Readable for SFSR3rs {}
///`reset()` method sets SFSR3 to value 0
impl crate::Resettable for SFSR3rs {}
