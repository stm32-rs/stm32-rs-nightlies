///Register `ISR4` reader
pub type R = crate::R<ISR4rs>;
///Field `IAF128` reader - illegal access interrupt enable for peripheral 128
pub type IAF128_R = crate::BitReader;
///Field `IAF129` reader - illegal access interrupt enable for peripheral 129
pub type IAF129_R = crate::BitReader;
///Field `IAF130` reader - illegal access interrupt enable for peripheral 130
pub type IAF130_R = crate::BitReader;
///Field `IAF131` reader - illegal access interrupt enable for peripheral 131
pub type IAF131_R = crate::BitReader;
///Field `IAF132` reader - illegal access interrupt enable for peripheral 132
pub type IAF132_R = crate::BitReader;
///Field `IAF133` reader - illegal access interrupt enable for peripheral 133
pub type IAF133_R = crate::BitReader;
///Field `IAF134` reader - illegal access interrupt enable for peripheral 134
pub type IAF134_R = crate::BitReader;
///Field `IAF135` reader - illegal access interrupt enable for peripheral 135
pub type IAF135_R = crate::BitReader;
///Field `IAF136` reader - illegal access interrupt enable for peripheral 136
pub type IAF136_R = crate::BitReader;
///Field `IAF137` reader - illegal access interrupt enable for peripheral 137
pub type IAF137_R = crate::BitReader;
///Field `IAF138` reader - illegal access interrupt enable for peripheral 138
pub type IAF138_R = crate::BitReader;
///Field `IAF139` reader - illegal access interrupt enable for peripheral 139
pub type IAF139_R = crate::BitReader;
///Field `IAF140` reader - illegal access interrupt enable for peripheral 140
pub type IAF140_R = crate::BitReader;
///Field `IAF141` reader - illegal access interrupt enable for peripheral 141
pub type IAF141_R = crate::BitReader;
///Field `IAF142` reader - illegal access interrupt enable for peripheral 142
pub type IAF142_R = crate::BitReader;
///Field `IAF143` reader - illegal access interrupt enable for peripheral 143
pub type IAF143_R = crate::BitReader;
///Field `IAF144` reader - illegal access interrupt enable for peripheral 144
pub type IAF144_R = crate::BitReader;
///Field `IAF145` reader - illegal access interrupt enable for peripheral 145
pub type IAF145_R = crate::BitReader;
///Field `IAF146` reader - illegal access interrupt enable for peripheral 146
pub type IAF146_R = crate::BitReader;
///Field `IAF147` reader - illegal access interrupt enable for peripheral 147
pub type IAF147_R = crate::BitReader;
///Field `IAF148` reader - illegal access interrupt enable for peripheral 148
pub type IAF148_R = crate::BitReader;
///Field `IAF149` reader - illegal access interrupt enable for peripheral 149
pub type IAF149_R = crate::BitReader;
///Field `IAF150` reader - illegal access interrupt enable for peripheral 150
pub type IAF150_R = crate::BitReader;
///Field `IAF151` reader - illegal access interrupt enable for peripheral 151
pub type IAF151_R = crate::BitReader;
///Field `IAF152` reader - illegal access interrupt enable for peripheral 152
pub type IAF152_R = crate::BitReader;
///Field `IAF153` reader - illegal access interrupt enable for peripheral 153
pub type IAF153_R = crate::BitReader;
///Field `IAF154` reader - illegal access interrupt enable for peripheral 154
pub type IAF154_R = crate::BitReader;
///Field `IAF155` reader - illegal access interrupt enable for peripheral 155
pub type IAF155_R = crate::BitReader;
///Field `IAF156` reader - illegal access interrupt enable for peripheral 156
pub type IAF156_R = crate::BitReader;
///Field `IAF157` reader - illegal access interrupt enable for peripheral 157
pub type IAF157_R = crate::BitReader;
///Field `IAF158` reader - illegal access interrupt enable for peripheral 158
pub type IAF158_R = crate::BitReader;
///Field `IAF159` reader - illegal access interrupt enable for peripheral 159
pub type IAF159_R = crate::BitReader;
impl R {
    ///Bit 0 - illegal access interrupt enable for peripheral 128
    #[inline(always)]
    pub fn iaf128(&self) -> IAF128_R {
        IAF128_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access interrupt enable for peripheral 129
    #[inline(always)]
    pub fn iaf129(&self) -> IAF129_R {
        IAF129_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access interrupt enable for peripheral 130
    #[inline(always)]
    pub fn iaf130(&self) -> IAF130_R {
        IAF130_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access interrupt enable for peripheral 131
    #[inline(always)]
    pub fn iaf131(&self) -> IAF131_R {
        IAF131_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access interrupt enable for peripheral 132
    #[inline(always)]
    pub fn iaf132(&self) -> IAF132_R {
        IAF132_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access interrupt enable for peripheral 133
    #[inline(always)]
    pub fn iaf133(&self) -> IAF133_R {
        IAF133_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access interrupt enable for peripheral 134
    #[inline(always)]
    pub fn iaf134(&self) -> IAF134_R {
        IAF134_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access interrupt enable for peripheral 135
    #[inline(always)]
    pub fn iaf135(&self) -> IAF135_R {
        IAF135_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access interrupt enable for peripheral 136
    #[inline(always)]
    pub fn iaf136(&self) -> IAF136_R {
        IAF136_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access interrupt enable for peripheral 137
    #[inline(always)]
    pub fn iaf137(&self) -> IAF137_R {
        IAF137_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access interrupt enable for peripheral 138
    #[inline(always)]
    pub fn iaf138(&self) -> IAF138_R {
        IAF138_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access interrupt enable for peripheral 139
    #[inline(always)]
    pub fn iaf139(&self) -> IAF139_R {
        IAF139_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access interrupt enable for peripheral 140
    #[inline(always)]
    pub fn iaf140(&self) -> IAF140_R {
        IAF140_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access interrupt enable for peripheral 141
    #[inline(always)]
    pub fn iaf141(&self) -> IAF141_R {
        IAF141_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access interrupt enable for peripheral 142
    #[inline(always)]
    pub fn iaf142(&self) -> IAF142_R {
        IAF142_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access interrupt enable for peripheral 143
    #[inline(always)]
    pub fn iaf143(&self) -> IAF143_R {
        IAF143_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access interrupt enable for peripheral 144
    #[inline(always)]
    pub fn iaf144(&self) -> IAF144_R {
        IAF144_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access interrupt enable for peripheral 145
    #[inline(always)]
    pub fn iaf145(&self) -> IAF145_R {
        IAF145_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access interrupt enable for peripheral 146
    #[inline(always)]
    pub fn iaf146(&self) -> IAF146_R {
        IAF146_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access interrupt enable for peripheral 147
    #[inline(always)]
    pub fn iaf147(&self) -> IAF147_R {
        IAF147_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - illegal access interrupt enable for peripheral 148
    #[inline(always)]
    pub fn iaf148(&self) -> IAF148_R {
        IAF148_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - illegal access interrupt enable for peripheral 149
    #[inline(always)]
    pub fn iaf149(&self) -> IAF149_R {
        IAF149_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - illegal access interrupt enable for peripheral 150
    #[inline(always)]
    pub fn iaf150(&self) -> IAF150_R {
        IAF150_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - illegal access interrupt enable for peripheral 151
    #[inline(always)]
    pub fn iaf151(&self) -> IAF151_R {
        IAF151_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - illegal access interrupt enable for peripheral 152
    #[inline(always)]
    pub fn iaf152(&self) -> IAF152_R {
        IAF152_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access interrupt enable for peripheral 153
    #[inline(always)]
    pub fn iaf153(&self) -> IAF153_R {
        IAF153_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - illegal access interrupt enable for peripheral 154
    #[inline(always)]
    pub fn iaf154(&self) -> IAF154_R {
        IAF154_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - illegal access interrupt enable for peripheral 155
    #[inline(always)]
    pub fn iaf155(&self) -> IAF155_R {
        IAF155_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - illegal access interrupt enable for peripheral 156
    #[inline(always)]
    pub fn iaf156(&self) -> IAF156_R {
        IAF156_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - illegal access interrupt enable for peripheral 157
    #[inline(always)]
    pub fn iaf157(&self) -> IAF157_R {
        IAF157_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - illegal access interrupt enable for peripheral 158
    #[inline(always)]
    pub fn iaf158(&self) -> IAF158_R {
        IAF158_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - illegal access interrupt enable for peripheral 159
    #[inline(always)]
    pub fn iaf159(&self) -> IAF159_R {
        IAF159_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR4")
            .field("iaf128", &self.iaf128())
            .field("iaf129", &self.iaf129())
            .field("iaf130", &self.iaf130())
            .field("iaf131", &self.iaf131())
            .field("iaf132", &self.iaf132())
            .field("iaf133", &self.iaf133())
            .field("iaf134", &self.iaf134())
            .field("iaf135", &self.iaf135())
            .field("iaf136", &self.iaf136())
            .field("iaf137", &self.iaf137())
            .field("iaf138", &self.iaf138())
            .field("iaf139", &self.iaf139())
            .field("iaf140", &self.iaf140())
            .field("iaf141", &self.iaf141())
            .field("iaf142", &self.iaf142())
            .field("iaf143", &self.iaf143())
            .field("iaf144", &self.iaf144())
            .field("iaf145", &self.iaf145())
            .field("iaf146", &self.iaf146())
            .field("iaf147", &self.iaf147())
            .field("iaf148", &self.iaf148())
            .field("iaf149", &self.iaf149())
            .field("iaf150", &self.iaf150())
            .field("iaf151", &self.iaf151())
            .field("iaf152", &self.iaf152())
            .field("iaf153", &self.iaf153())
            .field("iaf154", &self.iaf154())
            .field("iaf155", &self.iaf155())
            .field("iaf156", &self.iaf156())
            .field("iaf157", &self.iaf157())
            .field("iaf158", &self.iaf158())
            .field("iaf159", &self.iaf159())
            .finish()
    }
}
/**IAC interrupt status register 4

You can [`read`](crate::Reg::read) this register and get [`isr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#IAC:ISR4)*/
pub struct ISR4rs;
impl crate::RegisterSpec for ISR4rs {
    type Ux = u32;
}
///`read()` method returns [`isr4::R`](R) reader structure
impl crate::Readable for ISR4rs {}
///`reset()` method sets ISR4 to value 0
impl crate::Resettable for ISR4rs {}
