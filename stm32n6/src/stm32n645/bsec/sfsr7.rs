///Register `SFSR7` reader
pub type R = crate::R<SFSR7rs>;
///Field `SFW224` reader - Shadowed fuse word 224
pub type SFW224_R = crate::BitReader;
///Field `SFW225` reader - Shadowed fuse word 225
pub type SFW225_R = crate::BitReader;
///Field `SFW226` reader - Shadowed fuse word 226
pub type SFW226_R = crate::BitReader;
///Field `SFW227` reader - Shadowed fuse word 227
pub type SFW227_R = crate::BitReader;
///Field `SFW228` reader - Shadowed fuse word 228
pub type SFW228_R = crate::BitReader;
///Field `SFW229` reader - Shadowed fuse word 229
pub type SFW229_R = crate::BitReader;
///Field `SFW230` reader - Shadowed fuse word 230
pub type SFW230_R = crate::BitReader;
///Field `SFW231` reader - Shadowed fuse word 231
pub type SFW231_R = crate::BitReader;
///Field `SFW232` reader - Shadowed fuse word 232
pub type SFW232_R = crate::BitReader;
///Field `SFW233` reader - Shadowed fuse word 233
pub type SFW233_R = crate::BitReader;
///Field `SFW234` reader - Shadowed fuse word 234
pub type SFW234_R = crate::BitReader;
///Field `SFW235` reader - Shadowed fuse word 235
pub type SFW235_R = crate::BitReader;
///Field `SFW236` reader - Shadowed fuse word 236
pub type SFW236_R = crate::BitReader;
///Field `SFW237` reader - Shadowed fuse word 237
pub type SFW237_R = crate::BitReader;
///Field `SFW238` reader - Shadowed fuse word 238
pub type SFW238_R = crate::BitReader;
///Field `SFW239` reader - Shadowed fuse word 239
pub type SFW239_R = crate::BitReader;
///Field `SFW240` reader - Shadowed fuse word 240
pub type SFW240_R = crate::BitReader;
///Field `SFW241` reader - Shadowed fuse word 241
pub type SFW241_R = crate::BitReader;
///Field `SFW242` reader - Shadowed fuse word 242
pub type SFW242_R = crate::BitReader;
///Field `SFW243` reader - Shadowed fuse word 243
pub type SFW243_R = crate::BitReader;
///Field `SFW244` reader - Shadowed fuse word 244
pub type SFW244_R = crate::BitReader;
///Field `SFW245` reader - Shadowed fuse word 245
pub type SFW245_R = crate::BitReader;
///Field `SFW246` reader - Shadowed fuse word 246
pub type SFW246_R = crate::BitReader;
///Field `SFW247` reader - Shadowed fuse word 247
pub type SFW247_R = crate::BitReader;
///Field `SFW248` reader - Shadowed fuse word 248
pub type SFW248_R = crate::BitReader;
///Field `SFW249` reader - Shadowed fuse word 249
pub type SFW249_R = crate::BitReader;
///Field `SFW250` reader - Shadowed fuse word 250
pub type SFW250_R = crate::BitReader;
///Field `SFW251` reader - Shadowed fuse word 251
pub type SFW251_R = crate::BitReader;
///Field `SFW252` reader - Shadowed fuse word 252
pub type SFW252_R = crate::BitReader;
///Field `SFW253` reader - Shadowed fuse word 253
pub type SFW253_R = crate::BitReader;
///Field `SFW254` reader - Shadowed fuse word 254
pub type SFW254_R = crate::BitReader;
///Field `SFW255` reader - Shadowed fuse word 255
pub type SFW255_R = crate::BitReader;
impl R {
    ///Bit 0 - Shadowed fuse word 224
    #[inline(always)]
    pub fn sfw224(&self) -> SFW224_R {
        SFW224_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Shadowed fuse word 225
    #[inline(always)]
    pub fn sfw225(&self) -> SFW225_R {
        SFW225_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Shadowed fuse word 226
    #[inline(always)]
    pub fn sfw226(&self) -> SFW226_R {
        SFW226_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Shadowed fuse word 227
    #[inline(always)]
    pub fn sfw227(&self) -> SFW227_R {
        SFW227_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Shadowed fuse word 228
    #[inline(always)]
    pub fn sfw228(&self) -> SFW228_R {
        SFW228_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Shadowed fuse word 229
    #[inline(always)]
    pub fn sfw229(&self) -> SFW229_R {
        SFW229_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Shadowed fuse word 230
    #[inline(always)]
    pub fn sfw230(&self) -> SFW230_R {
        SFW230_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Shadowed fuse word 231
    #[inline(always)]
    pub fn sfw231(&self) -> SFW231_R {
        SFW231_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Shadowed fuse word 232
    #[inline(always)]
    pub fn sfw232(&self) -> SFW232_R {
        SFW232_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Shadowed fuse word 233
    #[inline(always)]
    pub fn sfw233(&self) -> SFW233_R {
        SFW233_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Shadowed fuse word 234
    #[inline(always)]
    pub fn sfw234(&self) -> SFW234_R {
        SFW234_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Shadowed fuse word 235
    #[inline(always)]
    pub fn sfw235(&self) -> SFW235_R {
        SFW235_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Shadowed fuse word 236
    #[inline(always)]
    pub fn sfw236(&self) -> SFW236_R {
        SFW236_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Shadowed fuse word 237
    #[inline(always)]
    pub fn sfw237(&self) -> SFW237_R {
        SFW237_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Shadowed fuse word 238
    #[inline(always)]
    pub fn sfw238(&self) -> SFW238_R {
        SFW238_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Shadowed fuse word 239
    #[inline(always)]
    pub fn sfw239(&self) -> SFW239_R {
        SFW239_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Shadowed fuse word 240
    #[inline(always)]
    pub fn sfw240(&self) -> SFW240_R {
        SFW240_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Shadowed fuse word 241
    #[inline(always)]
    pub fn sfw241(&self) -> SFW241_R {
        SFW241_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Shadowed fuse word 242
    #[inline(always)]
    pub fn sfw242(&self) -> SFW242_R {
        SFW242_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Shadowed fuse word 243
    #[inline(always)]
    pub fn sfw243(&self) -> SFW243_R {
        SFW243_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Shadowed fuse word 244
    #[inline(always)]
    pub fn sfw244(&self) -> SFW244_R {
        SFW244_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Shadowed fuse word 245
    #[inline(always)]
    pub fn sfw245(&self) -> SFW245_R {
        SFW245_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Shadowed fuse word 246
    #[inline(always)]
    pub fn sfw246(&self) -> SFW246_R {
        SFW246_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Shadowed fuse word 247
    #[inline(always)]
    pub fn sfw247(&self) -> SFW247_R {
        SFW247_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Shadowed fuse word 248
    #[inline(always)]
    pub fn sfw248(&self) -> SFW248_R {
        SFW248_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Shadowed fuse word 249
    #[inline(always)]
    pub fn sfw249(&self) -> SFW249_R {
        SFW249_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Shadowed fuse word 250
    #[inline(always)]
    pub fn sfw250(&self) -> SFW250_R {
        SFW250_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Shadowed fuse word 251
    #[inline(always)]
    pub fn sfw251(&self) -> SFW251_R {
        SFW251_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Shadowed fuse word 252
    #[inline(always)]
    pub fn sfw252(&self) -> SFW252_R {
        SFW252_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Shadowed fuse word 253
    #[inline(always)]
    pub fn sfw253(&self) -> SFW253_R {
        SFW253_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Shadowed fuse word 254
    #[inline(always)]
    pub fn sfw254(&self) -> SFW254_R {
        SFW254_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Shadowed fuse word 255
    #[inline(always)]
    pub fn sfw255(&self) -> SFW255_R {
        SFW255_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFSR7")
            .field("sfw224", &self.sfw224())
            .field("sfw225", &self.sfw225())
            .field("sfw226", &self.sfw226())
            .field("sfw227", &self.sfw227())
            .field("sfw228", &self.sfw228())
            .field("sfw229", &self.sfw229())
            .field("sfw230", &self.sfw230())
            .field("sfw231", &self.sfw231())
            .field("sfw232", &self.sfw232())
            .field("sfw233", &self.sfw233())
            .field("sfw234", &self.sfw234())
            .field("sfw235", &self.sfw235())
            .field("sfw236", &self.sfw236())
            .field("sfw237", &self.sfw237())
            .field("sfw238", &self.sfw238())
            .field("sfw239", &self.sfw239())
            .field("sfw240", &self.sfw240())
            .field("sfw241", &self.sfw241())
            .field("sfw242", &self.sfw242())
            .field("sfw243", &self.sfw243())
            .field("sfw244", &self.sfw244())
            .field("sfw245", &self.sfw245())
            .field("sfw246", &self.sfw246())
            .field("sfw247", &self.sfw247())
            .field("sfw248", &self.sfw248())
            .field("sfw249", &self.sfw249())
            .field("sfw250", &self.sfw250())
            .field("sfw251", &self.sfw251())
            .field("sfw252", &self.sfw252())
            .field("sfw253", &self.sfw253())
            .field("sfw254", &self.sfw254())
            .field("sfw255", &self.sfw255())
            .finish()
    }
}
/**BSEC shadowed fuses status register 7

You can [`read`](crate::Reg::read) this register and get [`sfsr7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SFSR7)*/
pub struct SFSR7rs;
impl crate::RegisterSpec for SFSR7rs {
    type Ux = u32;
}
///`read()` method returns [`sfsr7::R`](R) reader structure
impl crate::Readable for SFSR7rs {}
///`reset()` method sets SFSR7 to value 0
impl crate::Resettable for SFSR7rs {}
