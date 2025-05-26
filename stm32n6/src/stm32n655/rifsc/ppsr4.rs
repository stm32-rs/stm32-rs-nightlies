///Register `PPSR4` reader
pub type R = crate::R<PPSR4rs>;
///Field `PPEN128` reader - peripheral protection enable 128
pub type PPEN128_R = crate::BitReader;
///Field `PPEN129` reader - peripheral protection enable 129
pub type PPEN129_R = crate::BitReader;
///Field `PPEN130` reader - peripheral protection enable 130
pub type PPEN130_R = crate::BitReader;
///Field `PPEN131` reader - peripheral protection enable 131
pub type PPEN131_R = crate::BitReader;
///Field `PPEN132` reader - peripheral protection enable 132
pub type PPEN132_R = crate::BitReader;
///Field `PPEN133` reader - peripheral protection enable 133
pub type PPEN133_R = crate::BitReader;
///Field `PPEN134` reader - peripheral protection enable 134
pub type PPEN134_R = crate::BitReader;
///Field `PPEN135` reader - peripheral protection enable 135
pub type PPEN135_R = crate::BitReader;
///Field `PPEN136` reader - peripheral protection enable 136
pub type PPEN136_R = crate::BitReader;
///Field `PPEN137` reader - peripheral protection enable 137
pub type PPEN137_R = crate::BitReader;
///Field `PPEN138` reader - peripheral protection enable 138
pub type PPEN138_R = crate::BitReader;
///Field `PPEN139` reader - peripheral protection enable 139
pub type PPEN139_R = crate::BitReader;
///Field `PPEN140` reader - peripheral protection enable 140
pub type PPEN140_R = crate::BitReader;
///Field `PPEN141` reader - peripheral protection enable 141
pub type PPEN141_R = crate::BitReader;
///Field `PPEN142` reader - peripheral protection enable 142
pub type PPEN142_R = crate::BitReader;
///Field `PPEN143` reader - peripheral protection enable 143
pub type PPEN143_R = crate::BitReader;
///Field `PPEN144` reader - peripheral protection enable 144
pub type PPEN144_R = crate::BitReader;
///Field `PPEN145` reader - peripheral protection enable 145
pub type PPEN145_R = crate::BitReader;
///Field `PPEN146` reader - peripheral protection enable 146
pub type PPEN146_R = crate::BitReader;
///Field `PPEN147` reader - peripheral protection enable 147
pub type PPEN147_R = crate::BitReader;
///Field `PPEN148` reader - peripheral protection enable 148
pub type PPEN148_R = crate::BitReader;
///Field `PPEN149` reader - peripheral protection enable 149
pub type PPEN149_R = crate::BitReader;
///Field `PPEN150` reader - peripheral protection enable 150
pub type PPEN150_R = crate::BitReader;
///Field `PPEN151` reader - peripheral protection enable 151
pub type PPEN151_R = crate::BitReader;
///Field `PPEN152` reader - peripheral protection enable 152
pub type PPEN152_R = crate::BitReader;
///Field `PPEN153` reader - peripheral protection enable 153
pub type PPEN153_R = crate::BitReader;
///Field `PPEN154` reader - peripheral protection enable 154
pub type PPEN154_R = crate::BitReader;
///Field `PPEN155` reader - peripheral protection enable 155
pub type PPEN155_R = crate::BitReader;
///Field `PPEN156` reader - peripheral protection enable 156
pub type PPEN156_R = crate::BitReader;
///Field `PPEN157` reader - peripheral protection enable 157
pub type PPEN157_R = crate::BitReader;
///Field `PPEN158` reader - peripheral protection enable 158
pub type PPEN158_R = crate::BitReader;
///Field `PPEN159` reader - peripheral protection enable 159
pub type PPEN159_R = crate::BitReader;
impl R {
    ///Bit 0 - peripheral protection enable 128
    #[inline(always)]
    pub fn ppen128(&self) -> PPEN128_R {
        PPEN128_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - peripheral protection enable 129
    #[inline(always)]
    pub fn ppen129(&self) -> PPEN129_R {
        PPEN129_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - peripheral protection enable 130
    #[inline(always)]
    pub fn ppen130(&self) -> PPEN130_R {
        PPEN130_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - peripheral protection enable 131
    #[inline(always)]
    pub fn ppen131(&self) -> PPEN131_R {
        PPEN131_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - peripheral protection enable 132
    #[inline(always)]
    pub fn ppen132(&self) -> PPEN132_R {
        PPEN132_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - peripheral protection enable 133
    #[inline(always)]
    pub fn ppen133(&self) -> PPEN133_R {
        PPEN133_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - peripheral protection enable 134
    #[inline(always)]
    pub fn ppen134(&self) -> PPEN134_R {
        PPEN134_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - peripheral protection enable 135
    #[inline(always)]
    pub fn ppen135(&self) -> PPEN135_R {
        PPEN135_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - peripheral protection enable 136
    #[inline(always)]
    pub fn ppen136(&self) -> PPEN136_R {
        PPEN136_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - peripheral protection enable 137
    #[inline(always)]
    pub fn ppen137(&self) -> PPEN137_R {
        PPEN137_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - peripheral protection enable 138
    #[inline(always)]
    pub fn ppen138(&self) -> PPEN138_R {
        PPEN138_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - peripheral protection enable 139
    #[inline(always)]
    pub fn ppen139(&self) -> PPEN139_R {
        PPEN139_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - peripheral protection enable 140
    #[inline(always)]
    pub fn ppen140(&self) -> PPEN140_R {
        PPEN140_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - peripheral protection enable 141
    #[inline(always)]
    pub fn ppen141(&self) -> PPEN141_R {
        PPEN141_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - peripheral protection enable 142
    #[inline(always)]
    pub fn ppen142(&self) -> PPEN142_R {
        PPEN142_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - peripheral protection enable 143
    #[inline(always)]
    pub fn ppen143(&self) -> PPEN143_R {
        PPEN143_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - peripheral protection enable 144
    #[inline(always)]
    pub fn ppen144(&self) -> PPEN144_R {
        PPEN144_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - peripheral protection enable 145
    #[inline(always)]
    pub fn ppen145(&self) -> PPEN145_R {
        PPEN145_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - peripheral protection enable 146
    #[inline(always)]
    pub fn ppen146(&self) -> PPEN146_R {
        PPEN146_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - peripheral protection enable 147
    #[inline(always)]
    pub fn ppen147(&self) -> PPEN147_R {
        PPEN147_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - peripheral protection enable 148
    #[inline(always)]
    pub fn ppen148(&self) -> PPEN148_R {
        PPEN148_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - peripheral protection enable 149
    #[inline(always)]
    pub fn ppen149(&self) -> PPEN149_R {
        PPEN149_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - peripheral protection enable 150
    #[inline(always)]
    pub fn ppen150(&self) -> PPEN150_R {
        PPEN150_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - peripheral protection enable 151
    #[inline(always)]
    pub fn ppen151(&self) -> PPEN151_R {
        PPEN151_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - peripheral protection enable 152
    #[inline(always)]
    pub fn ppen152(&self) -> PPEN152_R {
        PPEN152_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - peripheral protection enable 153
    #[inline(always)]
    pub fn ppen153(&self) -> PPEN153_R {
        PPEN153_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - peripheral protection enable 154
    #[inline(always)]
    pub fn ppen154(&self) -> PPEN154_R {
        PPEN154_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - peripheral protection enable 155
    #[inline(always)]
    pub fn ppen155(&self) -> PPEN155_R {
        PPEN155_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - peripheral protection enable 156
    #[inline(always)]
    pub fn ppen156(&self) -> PPEN156_R {
        PPEN156_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - peripheral protection enable 157
    #[inline(always)]
    pub fn ppen157(&self) -> PPEN157_R {
        PPEN157_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - peripheral protection enable 158
    #[inline(always)]
    pub fn ppen158(&self) -> PPEN158_R {
        PPEN158_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - peripheral protection enable 159
    #[inline(always)]
    pub fn ppen159(&self) -> PPEN159_R {
        PPEN159_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPSR4")
            .field("ppen128", &self.ppen128())
            .field("ppen129", &self.ppen129())
            .field("ppen130", &self.ppen130())
            .field("ppen131", &self.ppen131())
            .field("ppen132", &self.ppen132())
            .field("ppen133", &self.ppen133())
            .field("ppen134", &self.ppen134())
            .field("ppen135", &self.ppen135())
            .field("ppen136", &self.ppen136())
            .field("ppen137", &self.ppen137())
            .field("ppen138", &self.ppen138())
            .field("ppen139", &self.ppen139())
            .field("ppen140", &self.ppen140())
            .field("ppen141", &self.ppen141())
            .field("ppen142", &self.ppen142())
            .field("ppen143", &self.ppen143())
            .field("ppen144", &self.ppen144())
            .field("ppen145", &self.ppen145())
            .field("ppen146", &self.ppen146())
            .field("ppen147", &self.ppen147())
            .field("ppen148", &self.ppen148())
            .field("ppen149", &self.ppen149())
            .field("ppen150", &self.ppen150())
            .field("ppen151", &self.ppen151())
            .field("ppen152", &self.ppen152())
            .field("ppen153", &self.ppen153())
            .field("ppen154", &self.ppen154())
            .field("ppen155", &self.ppen155())
            .field("ppen156", &self.ppen156())
            .field("ppen157", &self.ppen157())
            .field("ppen158", &self.ppen158())
            .field("ppen159", &self.ppen159())
            .finish()
    }
}
/**RIFSC peripheral protection status register 4

You can [`read`](crate::Reg::read) this register and get [`ppsr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:PPSR4)*/
pub struct PPSR4rs;
impl crate::RegisterSpec for PPSR4rs {
    type Ux = u32;
}
///`read()` method returns [`ppsr4::R`](R) reader structure
impl crate::Readable for PPSR4rs {}
///`reset()` method sets PPSR4 to value 0x3a0e_382e
impl crate::Resettable for PPSR4rs {
    const RESET_VALUE: u32 = 0x3a0e_382e;
}
