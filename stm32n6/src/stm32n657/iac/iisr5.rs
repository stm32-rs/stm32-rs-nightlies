///Register `IISR5` reader
pub type R = crate::R<IISR5rs>;
///Field `ILACIN128` reader - illegal access input 128
pub type ILACIN128_R = crate::BitReader;
///Field `ILACIN129` reader - illegal access input 129
pub type ILACIN129_R = crate::BitReader;
///Field `ILACIN130` reader - illegal access input 130
pub type ILACIN130_R = crate::BitReader;
///Field `ILACIN131` reader - illegal access input 131
pub type ILACIN131_R = crate::BitReader;
///Field `ILACIN132` reader - illegal access input 132
pub type ILACIN132_R = crate::BitReader;
///Field `ILACIN133` reader - illegal access input 133
pub type ILACIN133_R = crate::BitReader;
///Field `ILACIN134` reader - illegal access input 134
pub type ILACIN134_R = crate::BitReader;
///Field `ILACIN135` reader - illegal access input 135
pub type ILACIN135_R = crate::BitReader;
///Field `ILACIN136` reader - illegal access input 136
pub type ILACIN136_R = crate::BitReader;
///Field `ILACIN137` reader - illegal access input 137
pub type ILACIN137_R = crate::BitReader;
///Field `ILACIN138` reader - illegal access input 138
pub type ILACIN138_R = crate::BitReader;
///Field `ILACIN139` reader - illegal access input 139
pub type ILACIN139_R = crate::BitReader;
///Field `ILACIN140` reader - illegal access input 140
pub type ILACIN140_R = crate::BitReader;
///Field `ILACIN141` reader - illegal access input 141
pub type ILACIN141_R = crate::BitReader;
///Field `ILACIN142` reader - illegal access input 142
pub type ILACIN142_R = crate::BitReader;
///Field `ILACIN143` reader - illegal access input 143
pub type ILACIN143_R = crate::BitReader;
///Field `ILACIN144` reader - illegal access input 144
pub type ILACIN144_R = crate::BitReader;
///Field `ILACIN145` reader - illegal access input 145
pub type ILACIN145_R = crate::BitReader;
///Field `ILACIN146` reader - illegal access input 146
pub type ILACIN146_R = crate::BitReader;
///Field `ILACIN147` reader - illegal access input 147
pub type ILACIN147_R = crate::BitReader;
///Field `ILACIN148` reader - illegal access input 148
pub type ILACIN148_R = crate::BitReader;
///Field `ILACIN149` reader - illegal access input 149
pub type ILACIN149_R = crate::BitReader;
///Field `ILACIN150` reader - illegal access input 150
pub type ILACIN150_R = crate::BitReader;
///Field `ILACIN151` reader - illegal access input 151
pub type ILACIN151_R = crate::BitReader;
///Field `ILACIN152` reader - illegal access input 152
pub type ILACIN152_R = crate::BitReader;
///Field `ILACIN153` reader - illegal access input 153
pub type ILACIN153_R = crate::BitReader;
///Field `ILACIN154` reader - illegal access input 154
pub type ILACIN154_R = crate::BitReader;
///Field `ILACIN155` reader - illegal access input 155
pub type ILACIN155_R = crate::BitReader;
///Field `ILACIN156` reader - illegal access input 156
pub type ILACIN156_R = crate::BitReader;
///Field `ILACIN157` reader - illegal access input 157
pub type ILACIN157_R = crate::BitReader;
///Field `ILACIN158` reader - illegal access input 158
pub type ILACIN158_R = crate::BitReader;
///Field `ILACIN159` reader - illegal access input 159
pub type ILACIN159_R = crate::BitReader;
impl R {
    ///Bit 0 - illegal access input 128
    #[inline(always)]
    pub fn ilacin128(&self) -> ILACIN128_R {
        ILACIN128_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access input 129
    #[inline(always)]
    pub fn ilacin129(&self) -> ILACIN129_R {
        ILACIN129_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access input 130
    #[inline(always)]
    pub fn ilacin130(&self) -> ILACIN130_R {
        ILACIN130_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access input 131
    #[inline(always)]
    pub fn ilacin131(&self) -> ILACIN131_R {
        ILACIN131_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access input 132
    #[inline(always)]
    pub fn ilacin132(&self) -> ILACIN132_R {
        ILACIN132_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access input 133
    #[inline(always)]
    pub fn ilacin133(&self) -> ILACIN133_R {
        ILACIN133_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access input 134
    #[inline(always)]
    pub fn ilacin134(&self) -> ILACIN134_R {
        ILACIN134_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access input 135
    #[inline(always)]
    pub fn ilacin135(&self) -> ILACIN135_R {
        ILACIN135_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access input 136
    #[inline(always)]
    pub fn ilacin136(&self) -> ILACIN136_R {
        ILACIN136_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access input 137
    #[inline(always)]
    pub fn ilacin137(&self) -> ILACIN137_R {
        ILACIN137_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access input 138
    #[inline(always)]
    pub fn ilacin138(&self) -> ILACIN138_R {
        ILACIN138_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access input 139
    #[inline(always)]
    pub fn ilacin139(&self) -> ILACIN139_R {
        ILACIN139_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access input 140
    #[inline(always)]
    pub fn ilacin140(&self) -> ILACIN140_R {
        ILACIN140_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access input 141
    #[inline(always)]
    pub fn ilacin141(&self) -> ILACIN141_R {
        ILACIN141_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access input 142
    #[inline(always)]
    pub fn ilacin142(&self) -> ILACIN142_R {
        ILACIN142_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access input 143
    #[inline(always)]
    pub fn ilacin143(&self) -> ILACIN143_R {
        ILACIN143_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access input 144
    #[inline(always)]
    pub fn ilacin144(&self) -> ILACIN144_R {
        ILACIN144_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access input 145
    #[inline(always)]
    pub fn ilacin145(&self) -> ILACIN145_R {
        ILACIN145_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access input 146
    #[inline(always)]
    pub fn ilacin146(&self) -> ILACIN146_R {
        ILACIN146_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access input 147
    #[inline(always)]
    pub fn ilacin147(&self) -> ILACIN147_R {
        ILACIN147_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - illegal access input 148
    #[inline(always)]
    pub fn ilacin148(&self) -> ILACIN148_R {
        ILACIN148_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - illegal access input 149
    #[inline(always)]
    pub fn ilacin149(&self) -> ILACIN149_R {
        ILACIN149_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - illegal access input 150
    #[inline(always)]
    pub fn ilacin150(&self) -> ILACIN150_R {
        ILACIN150_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - illegal access input 151
    #[inline(always)]
    pub fn ilacin151(&self) -> ILACIN151_R {
        ILACIN151_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - illegal access input 152
    #[inline(always)]
    pub fn ilacin152(&self) -> ILACIN152_R {
        ILACIN152_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access input 153
    #[inline(always)]
    pub fn ilacin153(&self) -> ILACIN153_R {
        ILACIN153_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - illegal access input 154
    #[inline(always)]
    pub fn ilacin154(&self) -> ILACIN154_R {
        ILACIN154_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - illegal access input 155
    #[inline(always)]
    pub fn ilacin155(&self) -> ILACIN155_R {
        ILACIN155_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - illegal access input 156
    #[inline(always)]
    pub fn ilacin156(&self) -> ILACIN156_R {
        ILACIN156_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - illegal access input 157
    #[inline(always)]
    pub fn ilacin157(&self) -> ILACIN157_R {
        ILACIN157_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - illegal access input 158
    #[inline(always)]
    pub fn ilacin158(&self) -> ILACIN158_R {
        ILACIN158_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - illegal access input 159
    #[inline(always)]
    pub fn ilacin159(&self) -> ILACIN159_R {
        ILACIN159_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IISR5")
            .field("ilacin128", &self.ilacin128())
            .field("ilacin129", &self.ilacin129())
            .field("ilacin130", &self.ilacin130())
            .field("ilacin131", &self.ilacin131())
            .field("ilacin132", &self.ilacin132())
            .field("ilacin133", &self.ilacin133())
            .field("ilacin134", &self.ilacin134())
            .field("ilacin135", &self.ilacin135())
            .field("ilacin136", &self.ilacin136())
            .field("ilacin137", &self.ilacin137())
            .field("ilacin138", &self.ilacin138())
            .field("ilacin139", &self.ilacin139())
            .field("ilacin140", &self.ilacin140())
            .field("ilacin141", &self.ilacin141())
            .field("ilacin142", &self.ilacin142())
            .field("ilacin143", &self.ilacin143())
            .field("ilacin144", &self.ilacin144())
            .field("ilacin145", &self.ilacin145())
            .field("ilacin146", &self.ilacin146())
            .field("ilacin147", &self.ilacin147())
            .field("ilacin148", &self.ilacin148())
            .field("ilacin149", &self.ilacin149())
            .field("ilacin150", &self.ilacin150())
            .field("ilacin151", &self.ilacin151())
            .field("ilacin152", &self.ilacin152())
            .field("ilacin153", &self.ilacin153())
            .field("ilacin154", &self.ilacin154())
            .field("ilacin155", &self.ilacin155())
            .field("ilacin156", &self.ilacin156())
            .field("ilacin157", &self.ilacin157())
            .field("ilacin158", &self.ilacin158())
            .field("ilacin159", &self.ilacin159())
            .finish()
    }
}
/**IAC ILAC input status register 5

You can [`read`](crate::Reg::read) this register and get [`iisr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#IAC:IISR5)*/
pub struct IISR5rs;
impl crate::RegisterSpec for IISR5rs {
    type Ux = u32;
}
///`read()` method returns [`iisr5::R`](R) reader structure
impl crate::Readable for IISR5rs {}
///`reset()` method sets IISR5 to value 0
impl crate::Resettable for IISR5rs {}
