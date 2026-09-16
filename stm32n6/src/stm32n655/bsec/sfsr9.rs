///Register `SFSR9` reader
pub type R = crate::R<SFSR9rs>;
///Field `SFW288` reader - Shadowed fuse word 288
pub type SFW288_R = crate::BitReader;
///Field `SFW289` reader - Shadowed fuse word 289
pub type SFW289_R = crate::BitReader;
///Field `SFW290` reader - Shadowed fuse word 290
pub type SFW290_R = crate::BitReader;
///Field `SFW291` reader - Shadowed fuse word 291
pub type SFW291_R = crate::BitReader;
///Field `SFW292` reader - Shadowed fuse word 292
pub type SFW292_R = crate::BitReader;
///Field `SFW293` reader - Shadowed fuse word 293
pub type SFW293_R = crate::BitReader;
///Field `SFW294` reader - Shadowed fuse word 294
pub type SFW294_R = crate::BitReader;
///Field `SFW295` reader - Shadowed fuse word 295
pub type SFW295_R = crate::BitReader;
///Field `SFW296` reader - Shadowed fuse word 296
pub type SFW296_R = crate::BitReader;
///Field `SFW297` reader - Shadowed fuse word 297
pub type SFW297_R = crate::BitReader;
///Field `SFW298` reader - Shadowed fuse word 298
pub type SFW298_R = crate::BitReader;
///Field `SFW299` reader - Shadowed fuse word 299
pub type SFW299_R = crate::BitReader;
///Field `SFW300` reader - Shadowed fuse word 300
pub type SFW300_R = crate::BitReader;
///Field `SFW301` reader - Shadowed fuse word 301
pub type SFW301_R = crate::BitReader;
///Field `SFW302` reader - Shadowed fuse word 302
pub type SFW302_R = crate::BitReader;
///Field `SFW303` reader - Shadowed fuse word 303
pub type SFW303_R = crate::BitReader;
///Field `SFW304` reader - Shadowed fuse word 304
pub type SFW304_R = crate::BitReader;
///Field `SFW305` reader - Shadowed fuse word 305
pub type SFW305_R = crate::BitReader;
///Field `SFW306` reader - Shadowed fuse word 306
pub type SFW306_R = crate::BitReader;
///Field `SFW307` reader - Shadowed fuse word 307
pub type SFW307_R = crate::BitReader;
///Field `SFW308` reader - Shadowed fuse word 308
pub type SFW308_R = crate::BitReader;
///Field `SFW309` reader - Shadowed fuse word 309
pub type SFW309_R = crate::BitReader;
///Field `SFW310` reader - Shadowed fuse word 310
pub type SFW310_R = crate::BitReader;
///Field `SFW311` reader - Shadowed fuse word 311
pub type SFW311_R = crate::BitReader;
///Field `SFW312` reader - Shadowed fuse word 312
pub type SFW312_R = crate::BitReader;
///Field `SFW313` reader - Shadowed fuse word 313
pub type SFW313_R = crate::BitReader;
///Field `SFW314` reader - Shadowed fuse word 314
pub type SFW314_R = crate::BitReader;
///Field `SFW315` reader - Shadowed fuse word 315
pub type SFW315_R = crate::BitReader;
///Field `SFW316` reader - Shadowed fuse word 316
pub type SFW316_R = crate::BitReader;
///Field `SFW317` reader - Shadowed fuse word 317
pub type SFW317_R = crate::BitReader;
///Field `SFW318` reader - Shadowed fuse word 318
pub type SFW318_R = crate::BitReader;
///Field `SFW319` reader - Shadowed fuse word 319
pub type SFW319_R = crate::BitReader;
impl R {
    ///Bit 0 - Shadowed fuse word 288
    #[inline(always)]
    pub fn sfw288(&self) -> SFW288_R {
        SFW288_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Shadowed fuse word 289
    #[inline(always)]
    pub fn sfw289(&self) -> SFW289_R {
        SFW289_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Shadowed fuse word 290
    #[inline(always)]
    pub fn sfw290(&self) -> SFW290_R {
        SFW290_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Shadowed fuse word 291
    #[inline(always)]
    pub fn sfw291(&self) -> SFW291_R {
        SFW291_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Shadowed fuse word 292
    #[inline(always)]
    pub fn sfw292(&self) -> SFW292_R {
        SFW292_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Shadowed fuse word 293
    #[inline(always)]
    pub fn sfw293(&self) -> SFW293_R {
        SFW293_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Shadowed fuse word 294
    #[inline(always)]
    pub fn sfw294(&self) -> SFW294_R {
        SFW294_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Shadowed fuse word 295
    #[inline(always)]
    pub fn sfw295(&self) -> SFW295_R {
        SFW295_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Shadowed fuse word 296
    #[inline(always)]
    pub fn sfw296(&self) -> SFW296_R {
        SFW296_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Shadowed fuse word 297
    #[inline(always)]
    pub fn sfw297(&self) -> SFW297_R {
        SFW297_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Shadowed fuse word 298
    #[inline(always)]
    pub fn sfw298(&self) -> SFW298_R {
        SFW298_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Shadowed fuse word 299
    #[inline(always)]
    pub fn sfw299(&self) -> SFW299_R {
        SFW299_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Shadowed fuse word 300
    #[inline(always)]
    pub fn sfw300(&self) -> SFW300_R {
        SFW300_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Shadowed fuse word 301
    #[inline(always)]
    pub fn sfw301(&self) -> SFW301_R {
        SFW301_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Shadowed fuse word 302
    #[inline(always)]
    pub fn sfw302(&self) -> SFW302_R {
        SFW302_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Shadowed fuse word 303
    #[inline(always)]
    pub fn sfw303(&self) -> SFW303_R {
        SFW303_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Shadowed fuse word 304
    #[inline(always)]
    pub fn sfw304(&self) -> SFW304_R {
        SFW304_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Shadowed fuse word 305
    #[inline(always)]
    pub fn sfw305(&self) -> SFW305_R {
        SFW305_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Shadowed fuse word 306
    #[inline(always)]
    pub fn sfw306(&self) -> SFW306_R {
        SFW306_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Shadowed fuse word 307
    #[inline(always)]
    pub fn sfw307(&self) -> SFW307_R {
        SFW307_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Shadowed fuse word 308
    #[inline(always)]
    pub fn sfw308(&self) -> SFW308_R {
        SFW308_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Shadowed fuse word 309
    #[inline(always)]
    pub fn sfw309(&self) -> SFW309_R {
        SFW309_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Shadowed fuse word 310
    #[inline(always)]
    pub fn sfw310(&self) -> SFW310_R {
        SFW310_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Shadowed fuse word 311
    #[inline(always)]
    pub fn sfw311(&self) -> SFW311_R {
        SFW311_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Shadowed fuse word 312
    #[inline(always)]
    pub fn sfw312(&self) -> SFW312_R {
        SFW312_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Shadowed fuse word 313
    #[inline(always)]
    pub fn sfw313(&self) -> SFW313_R {
        SFW313_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Shadowed fuse word 314
    #[inline(always)]
    pub fn sfw314(&self) -> SFW314_R {
        SFW314_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Shadowed fuse word 315
    #[inline(always)]
    pub fn sfw315(&self) -> SFW315_R {
        SFW315_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Shadowed fuse word 316
    #[inline(always)]
    pub fn sfw316(&self) -> SFW316_R {
        SFW316_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Shadowed fuse word 317
    #[inline(always)]
    pub fn sfw317(&self) -> SFW317_R {
        SFW317_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Shadowed fuse word 318
    #[inline(always)]
    pub fn sfw318(&self) -> SFW318_R {
        SFW318_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Shadowed fuse word 319
    #[inline(always)]
    pub fn sfw319(&self) -> SFW319_R {
        SFW319_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFSR9")
            .field("sfw288", &self.sfw288())
            .field("sfw289", &self.sfw289())
            .field("sfw290", &self.sfw290())
            .field("sfw291", &self.sfw291())
            .field("sfw292", &self.sfw292())
            .field("sfw293", &self.sfw293())
            .field("sfw294", &self.sfw294())
            .field("sfw295", &self.sfw295())
            .field("sfw296", &self.sfw296())
            .field("sfw297", &self.sfw297())
            .field("sfw298", &self.sfw298())
            .field("sfw299", &self.sfw299())
            .field("sfw300", &self.sfw300())
            .field("sfw301", &self.sfw301())
            .field("sfw302", &self.sfw302())
            .field("sfw303", &self.sfw303())
            .field("sfw304", &self.sfw304())
            .field("sfw305", &self.sfw305())
            .field("sfw306", &self.sfw306())
            .field("sfw307", &self.sfw307())
            .field("sfw308", &self.sfw308())
            .field("sfw309", &self.sfw309())
            .field("sfw310", &self.sfw310())
            .field("sfw311", &self.sfw311())
            .field("sfw312", &self.sfw312())
            .field("sfw313", &self.sfw313())
            .field("sfw314", &self.sfw314())
            .field("sfw315", &self.sfw315())
            .field("sfw316", &self.sfw316())
            .field("sfw317", &self.sfw317())
            .field("sfw318", &self.sfw318())
            .field("sfw319", &self.sfw319())
            .finish()
    }
}
/**BSEC shadowed fuses status register 9

You can [`read`](crate::Reg::read) this register and get [`sfsr9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#BSEC:SFSR9)*/
pub struct SFSR9rs;
impl crate::RegisterSpec for SFSR9rs {
    type Ux = u32;
}
///`read()` method returns [`sfsr9::R`](R) reader structure
impl crate::Readable for SFSR9rs {}
///`reset()` method sets SFSR9 to value 0
impl crate::Resettable for SFSR9rs {}
