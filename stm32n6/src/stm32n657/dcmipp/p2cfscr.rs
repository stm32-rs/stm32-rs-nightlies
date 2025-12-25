///Register `P2CFSCR` reader
pub type R = crate::R<P2CFSCRrs>;
///Field `DTIDA` reader - Current data type ID
pub type DTIDA_R = crate::FieldReader;
///Field `VC` reader - Current flow selection mode
pub type VC_R = crate::FieldReader;
///Field `FDTF` reader - Current force data type format
pub type FDTF_R = crate::FieldReader;
///Field `FDTFEN` reader - Current force data type format enable
pub type FDTFEN_R = crate::BitReader;
///Field `PIPEN` reader - Current activation of PipeN
pub type PIPEN_R = crate::BitReader;
impl R {
    ///Bits 0:5 - Current data type ID
    #[inline(always)]
    pub fn dtida(&self) -> DTIDA_R {
        DTIDA_R::new((self.bits & 0x3f) as u8)
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
        f.debug_struct("P2CFSCR")
            .field("dtida", &self.dtida())
            .field("vc", &self.vc())
            .field("fdtf", &self.fdtf())
            .field("fdtfen", &self.fdtfen())
            .field("pipen", &self.pipen())
            .finish()
    }
}
/**DCMIPP Pipe2 current flow selection configuration register

You can [`read`](crate::Reg::read) this register and get [`p2cfscr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CFSCR)*/
pub struct P2CFSCRrs;
impl crate::RegisterSpec for P2CFSCRrs {
    type Ux = u32;
}
///`read()` method returns [`p2cfscr::R`](R) reader structure
impl crate::Readable for P2CFSCRrs {}
///`reset()` method sets P2CFSCR to value 0
impl crate::Resettable for P2CFSCRrs {}
