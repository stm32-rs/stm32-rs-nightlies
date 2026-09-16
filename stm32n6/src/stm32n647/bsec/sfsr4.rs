///Register `SFSR4` reader
pub type R = crate::R<SFSR4rs>;
///Field `SFW128` reader - Shadowed fuse word 128
pub type SFW128_R = crate::BitReader;
///Field `SFW129` reader - Shadowed fuse word 129
pub type SFW129_R = crate::BitReader;
///Field `SFW130` reader - Shadowed fuse word 130
pub type SFW130_R = crate::BitReader;
///Field `SFW131` reader - Shadowed fuse word 131
pub type SFW131_R = crate::BitReader;
///Field `SFW132` reader - Shadowed fuse word 132
pub type SFW132_R = crate::BitReader;
///Field `SFW133` reader - Shadowed fuse word 133
pub type SFW133_R = crate::BitReader;
///Field `SFW134` reader - Shadowed fuse word 134
pub type SFW134_R = crate::BitReader;
///Field `SFW135` reader - Shadowed fuse word 135
pub type SFW135_R = crate::BitReader;
///Field `SFW136` reader - Shadowed fuse word 136
pub type SFW136_R = crate::BitReader;
///Field `SFW137` reader - Shadowed fuse word 137
pub type SFW137_R = crate::BitReader;
///Field `SFW138` reader - Shadowed fuse word 138
pub type SFW138_R = crate::BitReader;
///Field `SFW139` reader - Shadowed fuse word 139
pub type SFW139_R = crate::BitReader;
///Field `SFW140` reader - Shadowed fuse word 140
pub type SFW140_R = crate::BitReader;
///Field `SFW141` reader - Shadowed fuse word 141
pub type SFW141_R = crate::BitReader;
///Field `SFW142` reader - Shadowed fuse word 142
pub type SFW142_R = crate::BitReader;
///Field `SFW143` reader - Shadowed fuse word 143
pub type SFW143_R = crate::BitReader;
///Field `SFW144` reader - Shadowed fuse word 144
pub type SFW144_R = crate::BitReader;
///Field `SFW145` reader - Shadowed fuse word 145
pub type SFW145_R = crate::BitReader;
///Field `SFW146` reader - Shadowed fuse word 146
pub type SFW146_R = crate::BitReader;
///Field `SFW147` reader - Shadowed fuse word 147
pub type SFW147_R = crate::BitReader;
///Field `SFW148` reader - Shadowed fuse word 148
pub type SFW148_R = crate::BitReader;
///Field `SFW149` reader - Shadowed fuse word 149
pub type SFW149_R = crate::BitReader;
///Field `SFW150` reader - Shadowed fuse word 150
pub type SFW150_R = crate::BitReader;
///Field `SFW151` reader - Shadowed fuse word 151
pub type SFW151_R = crate::BitReader;
///Field `SFW152` reader - Shadowed fuse word 152
pub type SFW152_R = crate::BitReader;
///Field `SFW153` reader - Shadowed fuse word 153
pub type SFW153_R = crate::BitReader;
///Field `SFW154` reader - Shadowed fuse word 154
pub type SFW154_R = crate::BitReader;
///Field `SFW155` reader - Shadowed fuse word 155
pub type SFW155_R = crate::BitReader;
///Field `SFW156` reader - Shadowed fuse word 156
pub type SFW156_R = crate::BitReader;
///Field `SFW157` reader - Shadowed fuse word 157
pub type SFW157_R = crate::BitReader;
///Field `SFW158` reader - Shadowed fuse word 158
pub type SFW158_R = crate::BitReader;
///Field `SFW159` reader - Shadowed fuse word 159
pub type SFW159_R = crate::BitReader;
impl R {
    ///Bit 0 - Shadowed fuse word 128
    #[inline(always)]
    pub fn sfw128(&self) -> SFW128_R {
        SFW128_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Shadowed fuse word 129
    #[inline(always)]
    pub fn sfw129(&self) -> SFW129_R {
        SFW129_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Shadowed fuse word 130
    #[inline(always)]
    pub fn sfw130(&self) -> SFW130_R {
        SFW130_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Shadowed fuse word 131
    #[inline(always)]
    pub fn sfw131(&self) -> SFW131_R {
        SFW131_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Shadowed fuse word 132
    #[inline(always)]
    pub fn sfw132(&self) -> SFW132_R {
        SFW132_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Shadowed fuse word 133
    #[inline(always)]
    pub fn sfw133(&self) -> SFW133_R {
        SFW133_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Shadowed fuse word 134
    #[inline(always)]
    pub fn sfw134(&self) -> SFW134_R {
        SFW134_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Shadowed fuse word 135
    #[inline(always)]
    pub fn sfw135(&self) -> SFW135_R {
        SFW135_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Shadowed fuse word 136
    #[inline(always)]
    pub fn sfw136(&self) -> SFW136_R {
        SFW136_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Shadowed fuse word 137
    #[inline(always)]
    pub fn sfw137(&self) -> SFW137_R {
        SFW137_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Shadowed fuse word 138
    #[inline(always)]
    pub fn sfw138(&self) -> SFW138_R {
        SFW138_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Shadowed fuse word 139
    #[inline(always)]
    pub fn sfw139(&self) -> SFW139_R {
        SFW139_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Shadowed fuse word 140
    #[inline(always)]
    pub fn sfw140(&self) -> SFW140_R {
        SFW140_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Shadowed fuse word 141
    #[inline(always)]
    pub fn sfw141(&self) -> SFW141_R {
        SFW141_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Shadowed fuse word 142
    #[inline(always)]
    pub fn sfw142(&self) -> SFW142_R {
        SFW142_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Shadowed fuse word 143
    #[inline(always)]
    pub fn sfw143(&self) -> SFW143_R {
        SFW143_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Shadowed fuse word 144
    #[inline(always)]
    pub fn sfw144(&self) -> SFW144_R {
        SFW144_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Shadowed fuse word 145
    #[inline(always)]
    pub fn sfw145(&self) -> SFW145_R {
        SFW145_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Shadowed fuse word 146
    #[inline(always)]
    pub fn sfw146(&self) -> SFW146_R {
        SFW146_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Shadowed fuse word 147
    #[inline(always)]
    pub fn sfw147(&self) -> SFW147_R {
        SFW147_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Shadowed fuse word 148
    #[inline(always)]
    pub fn sfw148(&self) -> SFW148_R {
        SFW148_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Shadowed fuse word 149
    #[inline(always)]
    pub fn sfw149(&self) -> SFW149_R {
        SFW149_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Shadowed fuse word 150
    #[inline(always)]
    pub fn sfw150(&self) -> SFW150_R {
        SFW150_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Shadowed fuse word 151
    #[inline(always)]
    pub fn sfw151(&self) -> SFW151_R {
        SFW151_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Shadowed fuse word 152
    #[inline(always)]
    pub fn sfw152(&self) -> SFW152_R {
        SFW152_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Shadowed fuse word 153
    #[inline(always)]
    pub fn sfw153(&self) -> SFW153_R {
        SFW153_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Shadowed fuse word 154
    #[inline(always)]
    pub fn sfw154(&self) -> SFW154_R {
        SFW154_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Shadowed fuse word 155
    #[inline(always)]
    pub fn sfw155(&self) -> SFW155_R {
        SFW155_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Shadowed fuse word 156
    #[inline(always)]
    pub fn sfw156(&self) -> SFW156_R {
        SFW156_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Shadowed fuse word 157
    #[inline(always)]
    pub fn sfw157(&self) -> SFW157_R {
        SFW157_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Shadowed fuse word 158
    #[inline(always)]
    pub fn sfw158(&self) -> SFW158_R {
        SFW158_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Shadowed fuse word 159
    #[inline(always)]
    pub fn sfw159(&self) -> SFW159_R {
        SFW159_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFSR4")
            .field("sfw128", &self.sfw128())
            .field("sfw129", &self.sfw129())
            .field("sfw130", &self.sfw130())
            .field("sfw131", &self.sfw131())
            .field("sfw132", &self.sfw132())
            .field("sfw133", &self.sfw133())
            .field("sfw134", &self.sfw134())
            .field("sfw135", &self.sfw135())
            .field("sfw136", &self.sfw136())
            .field("sfw137", &self.sfw137())
            .field("sfw138", &self.sfw138())
            .field("sfw139", &self.sfw139())
            .field("sfw140", &self.sfw140())
            .field("sfw141", &self.sfw141())
            .field("sfw142", &self.sfw142())
            .field("sfw143", &self.sfw143())
            .field("sfw144", &self.sfw144())
            .field("sfw145", &self.sfw145())
            .field("sfw146", &self.sfw146())
            .field("sfw147", &self.sfw147())
            .field("sfw148", &self.sfw148())
            .field("sfw149", &self.sfw149())
            .field("sfw150", &self.sfw150())
            .field("sfw151", &self.sfw151())
            .field("sfw152", &self.sfw152())
            .field("sfw153", &self.sfw153())
            .field("sfw154", &self.sfw154())
            .field("sfw155", &self.sfw155())
            .field("sfw156", &self.sfw156())
            .field("sfw157", &self.sfw157())
            .field("sfw158", &self.sfw158())
            .field("sfw159", &self.sfw159())
            .finish()
    }
}
/**BSEC shadowed fuses status register 4

You can [`read`](crate::Reg::read) this register and get [`sfsr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#BSEC:SFSR4)*/
pub struct SFSR4rs;
impl crate::RegisterSpec for SFSR4rs {
    type Ux = u32;
}
///`read()` method returns [`sfsr4::R`](R) reader structure
impl crate::Readable for SFSR4rs {}
///`reset()` method sets SFSR4 to value 0
impl crate::Resettable for SFSR4rs {}
