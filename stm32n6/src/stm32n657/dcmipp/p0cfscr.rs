///Register `P0CFSCR` reader
pub type R = crate::R<P0CFSCRrs>;
///Field `DTIDA` reader - Current data type selection ID A
pub type DTIDA_R = crate::FieldReader;
///Field `DTIDB` reader - Current data type selection ID B
pub type DTIDB_R = crate::FieldReader;
///Field `DTMODE` reader - Flow selection mode
pub type DTMODE_R = crate::FieldReader;
///Field `VC` reader - Current flow selection mode
pub type VC_R = crate::FieldReader;
///Field `PIPEN` reader - Current activation of PipeN
pub type PIPEN_R = crate::BitReader;
impl R {
    ///Bits 0:5 - Current data type selection ID A
    #[inline(always)]
    pub fn dtida(&self) -> DTIDA_R {
        DTIDA_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:13 - Current data type selection ID B
    #[inline(always)]
    pub fn dtidb(&self) -> DTIDB_R {
        DTIDB_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:17 - Flow selection mode
    #[inline(always)]
    pub fn dtmode(&self) -> DTMODE_R {
        DTMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 19:20 - Current flow selection mode
    #[inline(always)]
    pub fn vc(&self) -> VC_R {
        VC_R::new(((self.bits >> 19) & 3) as u8)
    }
    ///Bit 31 - Current activation of PipeN
    #[inline(always)]
    pub fn pipen(&self) -> PIPEN_R {
        PIPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0CFSCR")
            .field("dtida", &self.dtida())
            .field("dtidb", &self.dtidb())
            .field("dtmode", &self.dtmode())
            .field("vc", &self.vc())
            .field("pipen", &self.pipen())
            .finish()
    }
}
/**DCMIPP Pipe0 current flow selection configuration register

You can [`read`](crate::Reg::read) this register and get [`p0cfscr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0CFSCR)*/
pub struct P0CFSCRrs;
impl crate::RegisterSpec for P0CFSCRrs {
    type Ux = u32;
}
///`read()` method returns [`p0cfscr::R`](R) reader structure
impl crate::Readable for P0CFSCRrs {}
///`reset()` method sets P0CFSCR to value 0
impl crate::Resettable for P0CFSCRrs {}
