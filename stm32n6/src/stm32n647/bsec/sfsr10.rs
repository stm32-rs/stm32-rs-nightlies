///Register `SFSR10` reader
pub type R = crate::R<SFSR10rs>;
///Field `SFW320` reader - Shadowed fuse word 320
pub type SFW320_R = crate::BitReader;
///Field `SFW321` reader - Shadowed fuse word 321
pub type SFW321_R = crate::BitReader;
///Field `SFW322` reader - Shadowed fuse word 322
pub type SFW322_R = crate::BitReader;
///Field `SFW323` reader - Shadowed fuse word 323
pub type SFW323_R = crate::BitReader;
///Field `SFW324` reader - Shadowed fuse word 324
pub type SFW324_R = crate::BitReader;
///Field `SFW325` reader - Shadowed fuse word 325
pub type SFW325_R = crate::BitReader;
///Field `SFW326` reader - Shadowed fuse word 326
pub type SFW326_R = crate::BitReader;
///Field `SFW327` reader - Shadowed fuse word 327
pub type SFW327_R = crate::BitReader;
///Field `SFW328` reader - Shadowed fuse word 328
pub type SFW328_R = crate::BitReader;
///Field `SFW329` reader - Shadowed fuse word 329
pub type SFW329_R = crate::BitReader;
///Field `SFW330` reader - Shadowed fuse word 330
pub type SFW330_R = crate::BitReader;
///Field `SFW331` reader - Shadowed fuse word 331
pub type SFW331_R = crate::BitReader;
///Field `SFW332` reader - Shadowed fuse word 332
pub type SFW332_R = crate::BitReader;
///Field `SFW333` reader - Shadowed fuse word 333
pub type SFW333_R = crate::BitReader;
///Field `SFW334` reader - Shadowed fuse word 334
pub type SFW334_R = crate::BitReader;
///Field `SFW335` reader - Shadowed fuse word 335
pub type SFW335_R = crate::BitReader;
///Field `SFW336` reader - Shadowed fuse word 336
pub type SFW336_R = crate::BitReader;
///Field `SFW337` reader - Shadowed fuse word 337
pub type SFW337_R = crate::BitReader;
///Field `SFW338` reader - Shadowed fuse word 338
pub type SFW338_R = crate::BitReader;
///Field `SFW339` reader - Shadowed fuse word 339
pub type SFW339_R = crate::BitReader;
///Field `SFW340` reader - Shadowed fuse word 340
pub type SFW340_R = crate::BitReader;
///Field `SFW341` reader - Shadowed fuse word 341
pub type SFW341_R = crate::BitReader;
///Field `SFW342` reader - Shadowed fuse word 342
pub type SFW342_R = crate::BitReader;
///Field `SFW343` reader - Shadowed fuse word 343
pub type SFW343_R = crate::BitReader;
///Field `SFW344` reader - Shadowed fuse word 344
pub type SFW344_R = crate::BitReader;
///Field `SFW345` reader - Shadowed fuse word 345
pub type SFW345_R = crate::BitReader;
///Field `SFW346` reader - Shadowed fuse word 346
pub type SFW346_R = crate::BitReader;
///Field `SFW347` reader - Shadowed fuse word 347
pub type SFW347_R = crate::BitReader;
///Field `SFW348` reader - Shadowed fuse word 348
pub type SFW348_R = crate::BitReader;
///Field `SFW349` reader - Shadowed fuse word 349
pub type SFW349_R = crate::BitReader;
///Field `SFW350` reader - Shadowed fuse word 350
pub type SFW350_R = crate::BitReader;
///Field `SFW351` reader - Shadowed fuse word 351
pub type SFW351_R = crate::BitReader;
impl R {
    ///Bit 0 - Shadowed fuse word 320
    #[inline(always)]
    pub fn sfw320(&self) -> SFW320_R {
        SFW320_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Shadowed fuse word 321
    #[inline(always)]
    pub fn sfw321(&self) -> SFW321_R {
        SFW321_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Shadowed fuse word 322
    #[inline(always)]
    pub fn sfw322(&self) -> SFW322_R {
        SFW322_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Shadowed fuse word 323
    #[inline(always)]
    pub fn sfw323(&self) -> SFW323_R {
        SFW323_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Shadowed fuse word 324
    #[inline(always)]
    pub fn sfw324(&self) -> SFW324_R {
        SFW324_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Shadowed fuse word 325
    #[inline(always)]
    pub fn sfw325(&self) -> SFW325_R {
        SFW325_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Shadowed fuse word 326
    #[inline(always)]
    pub fn sfw326(&self) -> SFW326_R {
        SFW326_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Shadowed fuse word 327
    #[inline(always)]
    pub fn sfw327(&self) -> SFW327_R {
        SFW327_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Shadowed fuse word 328
    #[inline(always)]
    pub fn sfw328(&self) -> SFW328_R {
        SFW328_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Shadowed fuse word 329
    #[inline(always)]
    pub fn sfw329(&self) -> SFW329_R {
        SFW329_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Shadowed fuse word 330
    #[inline(always)]
    pub fn sfw330(&self) -> SFW330_R {
        SFW330_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Shadowed fuse word 331
    #[inline(always)]
    pub fn sfw331(&self) -> SFW331_R {
        SFW331_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Shadowed fuse word 332
    #[inline(always)]
    pub fn sfw332(&self) -> SFW332_R {
        SFW332_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Shadowed fuse word 333
    #[inline(always)]
    pub fn sfw333(&self) -> SFW333_R {
        SFW333_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Shadowed fuse word 334
    #[inline(always)]
    pub fn sfw334(&self) -> SFW334_R {
        SFW334_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Shadowed fuse word 335
    #[inline(always)]
    pub fn sfw335(&self) -> SFW335_R {
        SFW335_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Shadowed fuse word 336
    #[inline(always)]
    pub fn sfw336(&self) -> SFW336_R {
        SFW336_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Shadowed fuse word 337
    #[inline(always)]
    pub fn sfw337(&self) -> SFW337_R {
        SFW337_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Shadowed fuse word 338
    #[inline(always)]
    pub fn sfw338(&self) -> SFW338_R {
        SFW338_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Shadowed fuse word 339
    #[inline(always)]
    pub fn sfw339(&self) -> SFW339_R {
        SFW339_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Shadowed fuse word 340
    #[inline(always)]
    pub fn sfw340(&self) -> SFW340_R {
        SFW340_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Shadowed fuse word 341
    #[inline(always)]
    pub fn sfw341(&self) -> SFW341_R {
        SFW341_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Shadowed fuse word 342
    #[inline(always)]
    pub fn sfw342(&self) -> SFW342_R {
        SFW342_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Shadowed fuse word 343
    #[inline(always)]
    pub fn sfw343(&self) -> SFW343_R {
        SFW343_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Shadowed fuse word 344
    #[inline(always)]
    pub fn sfw344(&self) -> SFW344_R {
        SFW344_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Shadowed fuse word 345
    #[inline(always)]
    pub fn sfw345(&self) -> SFW345_R {
        SFW345_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Shadowed fuse word 346
    #[inline(always)]
    pub fn sfw346(&self) -> SFW346_R {
        SFW346_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Shadowed fuse word 347
    #[inline(always)]
    pub fn sfw347(&self) -> SFW347_R {
        SFW347_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Shadowed fuse word 348
    #[inline(always)]
    pub fn sfw348(&self) -> SFW348_R {
        SFW348_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Shadowed fuse word 349
    #[inline(always)]
    pub fn sfw349(&self) -> SFW349_R {
        SFW349_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Shadowed fuse word 350
    #[inline(always)]
    pub fn sfw350(&self) -> SFW350_R {
        SFW350_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Shadowed fuse word 351
    #[inline(always)]
    pub fn sfw351(&self) -> SFW351_R {
        SFW351_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFSR10")
            .field("sfw320", &self.sfw320())
            .field("sfw321", &self.sfw321())
            .field("sfw322", &self.sfw322())
            .field("sfw323", &self.sfw323())
            .field("sfw324", &self.sfw324())
            .field("sfw325", &self.sfw325())
            .field("sfw326", &self.sfw326())
            .field("sfw327", &self.sfw327())
            .field("sfw328", &self.sfw328())
            .field("sfw329", &self.sfw329())
            .field("sfw330", &self.sfw330())
            .field("sfw331", &self.sfw331())
            .field("sfw332", &self.sfw332())
            .field("sfw333", &self.sfw333())
            .field("sfw334", &self.sfw334())
            .field("sfw335", &self.sfw335())
            .field("sfw336", &self.sfw336())
            .field("sfw337", &self.sfw337())
            .field("sfw338", &self.sfw338())
            .field("sfw339", &self.sfw339())
            .field("sfw340", &self.sfw340())
            .field("sfw341", &self.sfw341())
            .field("sfw342", &self.sfw342())
            .field("sfw343", &self.sfw343())
            .field("sfw344", &self.sfw344())
            .field("sfw345", &self.sfw345())
            .field("sfw346", &self.sfw346())
            .field("sfw347", &self.sfw347())
            .field("sfw348", &self.sfw348())
            .field("sfw349", &self.sfw349())
            .field("sfw350", &self.sfw350())
            .field("sfw351", &self.sfw351())
            .finish()
    }
}
/**BSEC shadowed fuses status register 10

You can [`read`](crate::Reg::read) this register and get [`sfsr10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#BSEC:SFSR10)*/
pub struct SFSR10rs;
impl crate::RegisterSpec for SFSR10rs {
    type Ux = u32;
}
///`read()` method returns [`sfsr10::R`](R) reader structure
impl crate::Readable for SFSR10rs {}
///`reset()` method sets SFSR10 to value 0
impl crate::Resettable for SFSR10rs {}
