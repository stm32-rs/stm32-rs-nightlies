///Register `SFSR11` reader
pub type R = crate::R<SFSR11rs>;
///Field `SFW352` reader - Shadowed fuse word 352
pub type SFW352_R = crate::BitReader;
///Field `SFW353` reader - Shadowed fuse word 353
pub type SFW353_R = crate::BitReader;
///Field `SFW354` reader - Shadowed fuse word 354
pub type SFW354_R = crate::BitReader;
///Field `SFW355` reader - Shadowed fuse word 355
pub type SFW355_R = crate::BitReader;
///Field `SFW356` reader - Shadowed fuse word 356
pub type SFW356_R = crate::BitReader;
///Field `SFW357` reader - Shadowed fuse word 357
pub type SFW357_R = crate::BitReader;
///Field `SFW358` reader - Shadowed fuse word 358
pub type SFW358_R = crate::BitReader;
///Field `SFW359` reader - Shadowed fuse word 359
pub type SFW359_R = crate::BitReader;
///Field `SFW360` reader - Shadowed fuse word 360
pub type SFW360_R = crate::BitReader;
///Field `SFW361` reader - Shadowed fuse word 361
pub type SFW361_R = crate::BitReader;
///Field `SFW362` reader - Shadowed fuse word 362
pub type SFW362_R = crate::BitReader;
///Field `SFW363` reader - Shadowed fuse word 363
pub type SFW363_R = crate::BitReader;
///Field `SFW364` reader - Shadowed fuse word 364
pub type SFW364_R = crate::BitReader;
///Field `SFW365` reader - Shadowed fuse word 365
pub type SFW365_R = crate::BitReader;
///Field `SFW366` reader - Shadowed fuse word 366
pub type SFW366_R = crate::BitReader;
///Field `SFW367` reader - Shadowed fuse word 367
pub type SFW367_R = crate::BitReader;
///Field `SFW368` reader - Shadowed fuse word 368
pub type SFW368_R = crate::BitReader;
///Field `SFW369` reader - Shadowed fuse word 369
pub type SFW369_R = crate::BitReader;
///Field `SFW370` reader - Shadowed fuse word 370
pub type SFW370_R = crate::BitReader;
///Field `SFW371` reader - Shadowed fuse word 371
pub type SFW371_R = crate::BitReader;
///Field `SFW372` reader - Shadowed fuse word 372
pub type SFW372_R = crate::BitReader;
///Field `SFW373` reader - Shadowed fuse word 373
pub type SFW373_R = crate::BitReader;
///Field `SFW374` reader - Shadowed fuse word 374
pub type SFW374_R = crate::BitReader;
///Field `SFW375` reader - Shadowed fuse word 375
pub type SFW375_R = crate::BitReader;
///Field `SFW376` reader - Shadowed fuse word 376
pub type SFW376_R = crate::BitReader;
///Field `SFW377` reader - Shadowed fuse word 377
pub type SFW377_R = crate::BitReader;
///Field `SFW378` reader - Shadowed fuse word 378
pub type SFW378_R = crate::BitReader;
///Field `SFW379` reader - Shadowed fuse word 379
pub type SFW379_R = crate::BitReader;
///Field `SFW380` reader - Shadowed fuse word 380
pub type SFW380_R = crate::BitReader;
///Field `SFW381` reader - Shadowed fuse word 381
pub type SFW381_R = crate::BitReader;
///Field `SFW382` reader - Shadowed fuse word 382
pub type SFW382_R = crate::BitReader;
///Field `SFW383` reader - Shadowed fuse word 383
pub type SFW383_R = crate::BitReader;
impl R {
    ///Bit 0 - Shadowed fuse word 352
    #[inline(always)]
    pub fn sfw352(&self) -> SFW352_R {
        SFW352_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Shadowed fuse word 353
    #[inline(always)]
    pub fn sfw353(&self) -> SFW353_R {
        SFW353_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Shadowed fuse word 354
    #[inline(always)]
    pub fn sfw354(&self) -> SFW354_R {
        SFW354_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Shadowed fuse word 355
    #[inline(always)]
    pub fn sfw355(&self) -> SFW355_R {
        SFW355_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Shadowed fuse word 356
    #[inline(always)]
    pub fn sfw356(&self) -> SFW356_R {
        SFW356_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Shadowed fuse word 357
    #[inline(always)]
    pub fn sfw357(&self) -> SFW357_R {
        SFW357_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Shadowed fuse word 358
    #[inline(always)]
    pub fn sfw358(&self) -> SFW358_R {
        SFW358_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Shadowed fuse word 359
    #[inline(always)]
    pub fn sfw359(&self) -> SFW359_R {
        SFW359_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Shadowed fuse word 360
    #[inline(always)]
    pub fn sfw360(&self) -> SFW360_R {
        SFW360_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Shadowed fuse word 361
    #[inline(always)]
    pub fn sfw361(&self) -> SFW361_R {
        SFW361_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Shadowed fuse word 362
    #[inline(always)]
    pub fn sfw362(&self) -> SFW362_R {
        SFW362_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Shadowed fuse word 363
    #[inline(always)]
    pub fn sfw363(&self) -> SFW363_R {
        SFW363_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Shadowed fuse word 364
    #[inline(always)]
    pub fn sfw364(&self) -> SFW364_R {
        SFW364_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Shadowed fuse word 365
    #[inline(always)]
    pub fn sfw365(&self) -> SFW365_R {
        SFW365_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Shadowed fuse word 366
    #[inline(always)]
    pub fn sfw366(&self) -> SFW366_R {
        SFW366_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Shadowed fuse word 367
    #[inline(always)]
    pub fn sfw367(&self) -> SFW367_R {
        SFW367_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Shadowed fuse word 368
    #[inline(always)]
    pub fn sfw368(&self) -> SFW368_R {
        SFW368_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Shadowed fuse word 369
    #[inline(always)]
    pub fn sfw369(&self) -> SFW369_R {
        SFW369_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Shadowed fuse word 370
    #[inline(always)]
    pub fn sfw370(&self) -> SFW370_R {
        SFW370_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Shadowed fuse word 371
    #[inline(always)]
    pub fn sfw371(&self) -> SFW371_R {
        SFW371_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Shadowed fuse word 372
    #[inline(always)]
    pub fn sfw372(&self) -> SFW372_R {
        SFW372_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Shadowed fuse word 373
    #[inline(always)]
    pub fn sfw373(&self) -> SFW373_R {
        SFW373_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Shadowed fuse word 374
    #[inline(always)]
    pub fn sfw374(&self) -> SFW374_R {
        SFW374_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Shadowed fuse word 375
    #[inline(always)]
    pub fn sfw375(&self) -> SFW375_R {
        SFW375_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Shadowed fuse word 376
    #[inline(always)]
    pub fn sfw376(&self) -> SFW376_R {
        SFW376_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Shadowed fuse word 377
    #[inline(always)]
    pub fn sfw377(&self) -> SFW377_R {
        SFW377_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Shadowed fuse word 378
    #[inline(always)]
    pub fn sfw378(&self) -> SFW378_R {
        SFW378_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Shadowed fuse word 379
    #[inline(always)]
    pub fn sfw379(&self) -> SFW379_R {
        SFW379_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Shadowed fuse word 380
    #[inline(always)]
    pub fn sfw380(&self) -> SFW380_R {
        SFW380_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Shadowed fuse word 381
    #[inline(always)]
    pub fn sfw381(&self) -> SFW381_R {
        SFW381_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Shadowed fuse word 382
    #[inline(always)]
    pub fn sfw382(&self) -> SFW382_R {
        SFW382_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Shadowed fuse word 383
    #[inline(always)]
    pub fn sfw383(&self) -> SFW383_R {
        SFW383_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFSR11")
            .field("sfw352", &self.sfw352())
            .field("sfw353", &self.sfw353())
            .field("sfw354", &self.sfw354())
            .field("sfw355", &self.sfw355())
            .field("sfw356", &self.sfw356())
            .field("sfw357", &self.sfw357())
            .field("sfw358", &self.sfw358())
            .field("sfw359", &self.sfw359())
            .field("sfw360", &self.sfw360())
            .field("sfw361", &self.sfw361())
            .field("sfw362", &self.sfw362())
            .field("sfw363", &self.sfw363())
            .field("sfw364", &self.sfw364())
            .field("sfw365", &self.sfw365())
            .field("sfw366", &self.sfw366())
            .field("sfw367", &self.sfw367())
            .field("sfw368", &self.sfw368())
            .field("sfw369", &self.sfw369())
            .field("sfw370", &self.sfw370())
            .field("sfw371", &self.sfw371())
            .field("sfw372", &self.sfw372())
            .field("sfw373", &self.sfw373())
            .field("sfw374", &self.sfw374())
            .field("sfw375", &self.sfw375())
            .field("sfw376", &self.sfw376())
            .field("sfw377", &self.sfw377())
            .field("sfw378", &self.sfw378())
            .field("sfw379", &self.sfw379())
            .field("sfw380", &self.sfw380())
            .field("sfw381", &self.sfw381())
            .field("sfw382", &self.sfw382())
            .field("sfw383", &self.sfw383())
            .finish()
    }
}
/**BSEC shadowed fuses status register 11

You can [`read`](crate::Reg::read) this register and get [`sfsr11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#BSEC:SFSR11)*/
pub struct SFSR11rs;
impl crate::RegisterSpec for SFSR11rs {
    type Ux = u32;
}
///`read()` method returns [`sfsr11::R`](R) reader structure
impl crate::Readable for SFSR11rs {}
///`reset()` method sets SFSR11 to value 0
impl crate::Resettable for SFSR11rs {}
