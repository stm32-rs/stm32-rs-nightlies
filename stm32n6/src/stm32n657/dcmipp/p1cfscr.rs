///Register `P1CFSCR` reader
pub type R = crate::R<P1CFSCRrs>;
///Field `DTIDA` reader - Current data type ID A
pub type DTIDA_R = crate::FieldReader;
///Field `DTIDB` reader - Current data type ID B
pub type DTIDB_R = crate::FieldReader;
///Field `DTMODE` reader - Flow selection mode
pub type DTMODE_R = crate::FieldReader;
///Field `PIPEDIFF` reader - Current differentiates Pipe2 vs. Pipe1
pub type PIPEDIFF_R = crate::BitReader;
///Field `VC` reader - Current flow selection mode
pub type VC_R = crate::FieldReader;
///Field `FDTF` reader - Current force data type format
pub type FDTF_R = crate::FieldReader;
///Field `FDTFEN` reader - Current force data type format enable
pub type FDTFEN_R = crate::BitReader;
///Field `PIPEN` reader - Current activation of PipeN
pub type PIPEN_R = crate::BitReader;
impl R {
    ///Bits 0:5 - Current data type ID A
    #[inline(always)]
    pub fn dtida(&self) -> DTIDA_R {
        DTIDA_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:13 - Current data type ID B
    #[inline(always)]
    pub fn dtidb(&self) -> DTIDB_R {
        DTIDB_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:17 - Flow selection mode
    #[inline(always)]
    pub fn dtmode(&self) -> DTMODE_R {
        DTMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - Current differentiates Pipe2 vs. Pipe1
    #[inline(always)]
    pub fn pipediff(&self) -> PIPEDIFF_R {
        PIPEDIFF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:20 - Current flow selection mode
    #[inline(always)]
    pub fn vc(&self) -> VC_R {
        VC_R::new(((self.bits >> 19) & 3) as u8)
    }
    ///Bits 24:29 - Current force data type format
    #[inline(always)]
    pub fn fdtf(&self) -> FDTF_R {
        FDTF_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    ///Bit 30 - Current force data type format enable
    #[inline(always)]
    pub fn fdtfen(&self) -> FDTFEN_R {
        FDTFEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Current activation of PipeN
    #[inline(always)]
    pub fn pipen(&self) -> PIPEN_R {
        PIPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CFSCR")
            .field("dtida", &self.dtida())
            .field("dtidb", &self.dtidb())
            .field("dtmode", &self.dtmode())
            .field("pipediff", &self.pipediff())
            .field("vc", &self.vc())
            .field("fdtf", &self.fdtf())
            .field("fdtfen", &self.fdtfen())
            .field("pipen", &self.pipen())
            .finish()
    }
}
/**DCMIPP Pipe1 current flow selection configuration register

You can [`read`](crate::Reg::read) this register and get [`p1cfscr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P1CFSCR)*/
pub struct P1CFSCRrs;
impl crate::RegisterSpec for P1CFSCRrs {
    type Ux = u32;
}
///`read()` method returns [`p1cfscr::R`](R) reader structure
impl crate::Readable for P1CFSCRrs {}
///`reset()` method sets P1CFSCR to value 0
impl crate::Resettable for P1CFSCRrs {}
