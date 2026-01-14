///Register `SFSR5` reader
pub type R = crate::R<SFSR5rs>;
///Field `SFW160` reader - Shadowed fuse word 160
pub type SFW160_R = crate::BitReader;
///Field `SFW161` reader - Shadowed fuse word 161
pub type SFW161_R = crate::BitReader;
///Field `SFW162` reader - Shadowed fuse word 162
pub type SFW162_R = crate::BitReader;
///Field `SFW163` reader - Shadowed fuse word 163
pub type SFW163_R = crate::BitReader;
///Field `SFW164` reader - Shadowed fuse word 164
pub type SFW164_R = crate::BitReader;
///Field `SFW165` reader - Shadowed fuse word 165
pub type SFW165_R = crate::BitReader;
///Field `SFW166` reader - Shadowed fuse word 166
pub type SFW166_R = crate::BitReader;
///Field `SFW167` reader - Shadowed fuse word 167
pub type SFW167_R = crate::BitReader;
///Field `SFW168` reader - Shadowed fuse word 168
pub type SFW168_R = crate::BitReader;
///Field `SFW169` reader - Shadowed fuse word 169
pub type SFW169_R = crate::BitReader;
///Field `SFW170` reader - Shadowed fuse word 170
pub type SFW170_R = crate::BitReader;
///Field `SFW171` reader - Shadowed fuse word 171
pub type SFW171_R = crate::BitReader;
///Field `SFW172` reader - Shadowed fuse word 172
pub type SFW172_R = crate::BitReader;
///Field `SFW173` reader - Shadowed fuse word 173
pub type SFW173_R = crate::BitReader;
///Field `SFW174` reader - Shadowed fuse word 174
pub type SFW174_R = crate::BitReader;
///Field `SFW175` reader - Shadowed fuse word 175
pub type SFW175_R = crate::BitReader;
///Field `SFW176` reader - Shadowed fuse word 176
pub type SFW176_R = crate::BitReader;
///Field `SFW177` reader - Shadowed fuse word 177
pub type SFW177_R = crate::BitReader;
///Field `SFW178` reader - Shadowed fuse word 178
pub type SFW178_R = crate::BitReader;
///Field `SFW179` reader - Shadowed fuse word 179
pub type SFW179_R = crate::BitReader;
///Field `SFW180` reader - Shadowed fuse word 180
pub type SFW180_R = crate::BitReader;
///Field `SFW181` reader - Shadowed fuse word 181
pub type SFW181_R = crate::BitReader;
///Field `SFW182` reader - Shadowed fuse word 182
pub type SFW182_R = crate::BitReader;
///Field `SFW183` reader - Shadowed fuse word 183
pub type SFW183_R = crate::BitReader;
///Field `SFW184` reader - Shadowed fuse word 184
pub type SFW184_R = crate::BitReader;
///Field `SFW185` reader - Shadowed fuse word 185
pub type SFW185_R = crate::BitReader;
///Field `SFW186` reader - Shadowed fuse word 186
pub type SFW186_R = crate::BitReader;
///Field `SFW187` reader - Shadowed fuse word 187
pub type SFW187_R = crate::BitReader;
///Field `SFW188` reader - Shadowed fuse word 188
pub type SFW188_R = crate::BitReader;
///Field `SFW189` reader - Shadowed fuse word 189
pub type SFW189_R = crate::BitReader;
///Field `SFW190` reader - Shadowed fuse word 190
pub type SFW190_R = crate::BitReader;
///Field `SFW191` reader - Shadowed fuse word 191
pub type SFW191_R = crate::BitReader;
impl R {
    ///Bit 0 - Shadowed fuse word 160
    #[inline(always)]
    pub fn sfw160(&self) -> SFW160_R {
        SFW160_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Shadowed fuse word 161
    #[inline(always)]
    pub fn sfw161(&self) -> SFW161_R {
        SFW161_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Shadowed fuse word 162
    #[inline(always)]
    pub fn sfw162(&self) -> SFW162_R {
        SFW162_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Shadowed fuse word 163
    #[inline(always)]
    pub fn sfw163(&self) -> SFW163_R {
        SFW163_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Shadowed fuse word 164
    #[inline(always)]
    pub fn sfw164(&self) -> SFW164_R {
        SFW164_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Shadowed fuse word 165
    #[inline(always)]
    pub fn sfw165(&self) -> SFW165_R {
        SFW165_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Shadowed fuse word 166
    #[inline(always)]
    pub fn sfw166(&self) -> SFW166_R {
        SFW166_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Shadowed fuse word 167
    #[inline(always)]
    pub fn sfw167(&self) -> SFW167_R {
        SFW167_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Shadowed fuse word 168
    #[inline(always)]
    pub fn sfw168(&self) -> SFW168_R {
        SFW168_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Shadowed fuse word 169
    #[inline(always)]
    pub fn sfw169(&self) -> SFW169_R {
        SFW169_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Shadowed fuse word 170
    #[inline(always)]
    pub fn sfw170(&self) -> SFW170_R {
        SFW170_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Shadowed fuse word 171
    #[inline(always)]
    pub fn sfw171(&self) -> SFW171_R {
        SFW171_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Shadowed fuse word 172
    #[inline(always)]
    pub fn sfw172(&self) -> SFW172_R {
        SFW172_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Shadowed fuse word 173
    #[inline(always)]
    pub fn sfw173(&self) -> SFW173_R {
        SFW173_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Shadowed fuse word 174
    #[inline(always)]
    pub fn sfw174(&self) -> SFW174_R {
        SFW174_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Shadowed fuse word 175
    #[inline(always)]
    pub fn sfw175(&self) -> SFW175_R {
        SFW175_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Shadowed fuse word 176
    #[inline(always)]
    pub fn sfw176(&self) -> SFW176_R {
        SFW176_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Shadowed fuse word 177
    #[inline(always)]
    pub fn sfw177(&self) -> SFW177_R {
        SFW177_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Shadowed fuse word 178
    #[inline(always)]
    pub fn sfw178(&self) -> SFW178_R {
        SFW178_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Shadowed fuse word 179
    #[inline(always)]
    pub fn sfw179(&self) -> SFW179_R {
        SFW179_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Shadowed fuse word 180
    #[inline(always)]
    pub fn sfw180(&self) -> SFW180_R {
        SFW180_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Shadowed fuse word 181
    #[inline(always)]
    pub fn sfw181(&self) -> SFW181_R {
        SFW181_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Shadowed fuse word 182
    #[inline(always)]
    pub fn sfw182(&self) -> SFW182_R {
        SFW182_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Shadowed fuse word 183
    #[inline(always)]
    pub fn sfw183(&self) -> SFW183_R {
        SFW183_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Shadowed fuse word 184
    #[inline(always)]
    pub fn sfw184(&self) -> SFW184_R {
        SFW184_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Shadowed fuse word 185
    #[inline(always)]
    pub fn sfw185(&self) -> SFW185_R {
        SFW185_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Shadowed fuse word 186
    #[inline(always)]
    pub fn sfw186(&self) -> SFW186_R {
        SFW186_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Shadowed fuse word 187
    #[inline(always)]
    pub fn sfw187(&self) -> SFW187_R {
        SFW187_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Shadowed fuse word 188
    #[inline(always)]
    pub fn sfw188(&self) -> SFW188_R {
        SFW188_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Shadowed fuse word 189
    #[inline(always)]
    pub fn sfw189(&self) -> SFW189_R {
        SFW189_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Shadowed fuse word 190
    #[inline(always)]
    pub fn sfw190(&self) -> SFW190_R {
        SFW190_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Shadowed fuse word 191
    #[inline(always)]
    pub fn sfw191(&self) -> SFW191_R {
        SFW191_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFSR5")
            .field("sfw160", &self.sfw160())
            .field("sfw161", &self.sfw161())
            .field("sfw162", &self.sfw162())
            .field("sfw163", &self.sfw163())
            .field("sfw164", &self.sfw164())
            .field("sfw165", &self.sfw165())
            .field("sfw166", &self.sfw166())
            .field("sfw167", &self.sfw167())
            .field("sfw168", &self.sfw168())
            .field("sfw169", &self.sfw169())
            .field("sfw170", &self.sfw170())
            .field("sfw171", &self.sfw171())
            .field("sfw172", &self.sfw172())
            .field("sfw173", &self.sfw173())
            .field("sfw174", &self.sfw174())
            .field("sfw175", &self.sfw175())
            .field("sfw176", &self.sfw176())
            .field("sfw177", &self.sfw177())
            .field("sfw178", &self.sfw178())
            .field("sfw179", &self.sfw179())
            .field("sfw180", &self.sfw180())
            .field("sfw181", &self.sfw181())
            .field("sfw182", &self.sfw182())
            .field("sfw183", &self.sfw183())
            .field("sfw184", &self.sfw184())
            .field("sfw185", &self.sfw185())
            .field("sfw186", &self.sfw186())
            .field("sfw187", &self.sfw187())
            .field("sfw188", &self.sfw188())
            .field("sfw189", &self.sfw189())
            .field("sfw190", &self.sfw190())
            .field("sfw191", &self.sfw191())
            .finish()
    }
}
/**BSEC shadowed fuses status register 5

You can [`read`](crate::Reg::read) this register and get [`sfsr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#BSEC:SFSR5)*/
pub struct SFSR5rs;
impl crate::RegisterSpec for SFSR5rs {
    type Ux = u32;
}
///`read()` method returns [`sfsr5::R`](R) reader structure
impl crate::Readable for SFSR5rs {}
///`reset()` method sets SFSR5 to value 0
impl crate::Resettable for SFSR5rs {}
