///Register `SFSR8` reader
pub type R = crate::R<SFSR8rs>;
///Field `SFW256` reader - Shadowed fuse word 256
pub type SFW256_R = crate::BitReader;
///Field `SFW257` reader - Shadowed fuse word 257
pub type SFW257_R = crate::BitReader;
///Field `SFW258` reader - Shadowed fuse word 258
pub type SFW258_R = crate::BitReader;
///Field `SFW259` reader - Shadowed fuse word 259
pub type SFW259_R = crate::BitReader;
///Field `SFW260` reader - Shadowed fuse word 260
pub type SFW260_R = crate::BitReader;
///Field `SFW261` reader - Shadowed fuse word 261
pub type SFW261_R = crate::BitReader;
///Field `SFW262` reader - Shadowed fuse word 262
pub type SFW262_R = crate::BitReader;
///Field `SFW263` reader - Shadowed fuse word 263
pub type SFW263_R = crate::BitReader;
///Field `SFW264` reader - Shadowed fuse word 264
pub type SFW264_R = crate::BitReader;
///Field `SFW265` reader - Shadowed fuse word 265
pub type SFW265_R = crate::BitReader;
///Field `SFW266` reader - Shadowed fuse word 266
pub type SFW266_R = crate::BitReader;
///Field `SFW267` reader - Shadowed fuse word 267
pub type SFW267_R = crate::BitReader;
///Field `SFW268` reader - Shadowed fuse word 268
pub type SFW268_R = crate::BitReader;
///Field `SFW269` reader - Shadowed fuse word 269
pub type SFW269_R = crate::BitReader;
///Field `SFW270` reader - Shadowed fuse word 270
pub type SFW270_R = crate::BitReader;
///Field `SFW271` reader - Shadowed fuse word 271
pub type SFW271_R = crate::BitReader;
///Field `SFW272` reader - Shadowed fuse word 272
pub type SFW272_R = crate::BitReader;
///Field `SFW273` reader - Shadowed fuse word 273
pub type SFW273_R = crate::BitReader;
///Field `SFW274` reader - Shadowed fuse word 274
pub type SFW274_R = crate::BitReader;
///Field `SFW275` reader - Shadowed fuse word 275
pub type SFW275_R = crate::BitReader;
///Field `SFW276` reader - Shadowed fuse word 276
pub type SFW276_R = crate::BitReader;
///Field `SFW277` reader - Shadowed fuse word 277
pub type SFW277_R = crate::BitReader;
///Field `SFW278` reader - Shadowed fuse word 278
pub type SFW278_R = crate::BitReader;
///Field `SFW279` reader - Shadowed fuse word 279
pub type SFW279_R = crate::BitReader;
///Field `SFW280` reader - Shadowed fuse word 280
pub type SFW280_R = crate::BitReader;
///Field `SFW281` reader - Shadowed fuse word 281
pub type SFW281_R = crate::BitReader;
///Field `SFW282` reader - Shadowed fuse word 282
pub type SFW282_R = crate::BitReader;
///Field `SFW283` reader - Shadowed fuse word 283
pub type SFW283_R = crate::BitReader;
///Field `SFW284` reader - Shadowed fuse word 284
pub type SFW284_R = crate::BitReader;
///Field `SFW285` reader - Shadowed fuse word 285
pub type SFW285_R = crate::BitReader;
///Field `SFW286` reader - Shadowed fuse word 286
pub type SFW286_R = crate::BitReader;
///Field `SFW287` reader - Shadowed fuse word 287
pub type SFW287_R = crate::BitReader;
impl R {
    ///Bit 0 - Shadowed fuse word 256
    #[inline(always)]
    pub fn sfw256(&self) -> SFW256_R {
        SFW256_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Shadowed fuse word 257
    #[inline(always)]
    pub fn sfw257(&self) -> SFW257_R {
        SFW257_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Shadowed fuse word 258
    #[inline(always)]
    pub fn sfw258(&self) -> SFW258_R {
        SFW258_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Shadowed fuse word 259
    #[inline(always)]
    pub fn sfw259(&self) -> SFW259_R {
        SFW259_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Shadowed fuse word 260
    #[inline(always)]
    pub fn sfw260(&self) -> SFW260_R {
        SFW260_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Shadowed fuse word 261
    #[inline(always)]
    pub fn sfw261(&self) -> SFW261_R {
        SFW261_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Shadowed fuse word 262
    #[inline(always)]
    pub fn sfw262(&self) -> SFW262_R {
        SFW262_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Shadowed fuse word 263
    #[inline(always)]
    pub fn sfw263(&self) -> SFW263_R {
        SFW263_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Shadowed fuse word 264
    #[inline(always)]
    pub fn sfw264(&self) -> SFW264_R {
        SFW264_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Shadowed fuse word 265
    #[inline(always)]
    pub fn sfw265(&self) -> SFW265_R {
        SFW265_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Shadowed fuse word 266
    #[inline(always)]
    pub fn sfw266(&self) -> SFW266_R {
        SFW266_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Shadowed fuse word 267
    #[inline(always)]
    pub fn sfw267(&self) -> SFW267_R {
        SFW267_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Shadowed fuse word 268
    #[inline(always)]
    pub fn sfw268(&self) -> SFW268_R {
        SFW268_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Shadowed fuse word 269
    #[inline(always)]
    pub fn sfw269(&self) -> SFW269_R {
        SFW269_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Shadowed fuse word 270
    #[inline(always)]
    pub fn sfw270(&self) -> SFW270_R {
        SFW270_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Shadowed fuse word 271
    #[inline(always)]
    pub fn sfw271(&self) -> SFW271_R {
        SFW271_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Shadowed fuse word 272
    #[inline(always)]
    pub fn sfw272(&self) -> SFW272_R {
        SFW272_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Shadowed fuse word 273
    #[inline(always)]
    pub fn sfw273(&self) -> SFW273_R {
        SFW273_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Shadowed fuse word 274
    #[inline(always)]
    pub fn sfw274(&self) -> SFW274_R {
        SFW274_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Shadowed fuse word 275
    #[inline(always)]
    pub fn sfw275(&self) -> SFW275_R {
        SFW275_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Shadowed fuse word 276
    #[inline(always)]
    pub fn sfw276(&self) -> SFW276_R {
        SFW276_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Shadowed fuse word 277
    #[inline(always)]
    pub fn sfw277(&self) -> SFW277_R {
        SFW277_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Shadowed fuse word 278
    #[inline(always)]
    pub fn sfw278(&self) -> SFW278_R {
        SFW278_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Shadowed fuse word 279
    #[inline(always)]
    pub fn sfw279(&self) -> SFW279_R {
        SFW279_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Shadowed fuse word 280
    #[inline(always)]
    pub fn sfw280(&self) -> SFW280_R {
        SFW280_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Shadowed fuse word 281
    #[inline(always)]
    pub fn sfw281(&self) -> SFW281_R {
        SFW281_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Shadowed fuse word 282
    #[inline(always)]
    pub fn sfw282(&self) -> SFW282_R {
        SFW282_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Shadowed fuse word 283
    #[inline(always)]
    pub fn sfw283(&self) -> SFW283_R {
        SFW283_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Shadowed fuse word 284
    #[inline(always)]
    pub fn sfw284(&self) -> SFW284_R {
        SFW284_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Shadowed fuse word 285
    #[inline(always)]
    pub fn sfw285(&self) -> SFW285_R {
        SFW285_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Shadowed fuse word 286
    #[inline(always)]
    pub fn sfw286(&self) -> SFW286_R {
        SFW286_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Shadowed fuse word 287
    #[inline(always)]
    pub fn sfw287(&self) -> SFW287_R {
        SFW287_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFSR8")
            .field("sfw256", &self.sfw256())
            .field("sfw257", &self.sfw257())
            .field("sfw258", &self.sfw258())
            .field("sfw259", &self.sfw259())
            .field("sfw260", &self.sfw260())
            .field("sfw261", &self.sfw261())
            .field("sfw262", &self.sfw262())
            .field("sfw263", &self.sfw263())
            .field("sfw264", &self.sfw264())
            .field("sfw265", &self.sfw265())
            .field("sfw266", &self.sfw266())
            .field("sfw267", &self.sfw267())
            .field("sfw268", &self.sfw268())
            .field("sfw269", &self.sfw269())
            .field("sfw270", &self.sfw270())
            .field("sfw271", &self.sfw271())
            .field("sfw272", &self.sfw272())
            .field("sfw273", &self.sfw273())
            .field("sfw274", &self.sfw274())
            .field("sfw275", &self.sfw275())
            .field("sfw276", &self.sfw276())
            .field("sfw277", &self.sfw277())
            .field("sfw278", &self.sfw278())
            .field("sfw279", &self.sfw279())
            .field("sfw280", &self.sfw280())
            .field("sfw281", &self.sfw281())
            .field("sfw282", &self.sfw282())
            .field("sfw283", &self.sfw283())
            .field("sfw284", &self.sfw284())
            .field("sfw285", &self.sfw285())
            .field("sfw286", &self.sfw286())
            .field("sfw287", &self.sfw287())
            .finish()
    }
}
/**BSEC shadowed fuses status register 8

You can [`read`](crate::Reg::read) this register and get [`sfsr8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#BSEC:SFSR8)*/
pub struct SFSR8rs;
impl crate::RegisterSpec for SFSR8rs {
    type Ux = u32;
}
///`read()` method returns [`sfsr8::R`](R) reader structure
impl crate::Readable for SFSR8rs {}
///`reset()` method sets SFSR8 to value 0
impl crate::Resettable for SFSR8rs {}
