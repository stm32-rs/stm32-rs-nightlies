///Register `P1CPPCR` reader
pub type R = crate::R<P1CPPCRrs>;
///Field `FORMAT` reader - Memory format
pub type FORMAT_R = crate::FieldReader;
///Field `SWAPRB` reader - Swaps R-vs-B components if RGB, and U-vs-V components if YUV
pub type SWAPRB_R = crate::BitReader;
///Field `LINEMULT` reader - Amount of capture completed lines for LINE Event and Interrupt
pub type LINEMULT_R = crate::FieldReader;
///Field `DBM` reader - Double buffer mode
pub type DBM_R = crate::BitReader;
///Field `LMAWM` reader - Line multi address wrapping modulo
pub type LMAWM_R = crate::FieldReader;
///Field `LMAWE` reader - Line multi address wrapping enable bit
pub type LMAWE_R = crate::BitReader;
impl R {
    ///Bits 0:3 - Memory format
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - Swaps R-vs-B components if RGB, and U-vs-V components if YUV
    #[inline(always)]
    pub fn swaprb(&self) -> SWAPRB_R {
        SWAPRB_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 13:15 - Amount of capture completed lines for LINE Event and Interrupt
    #[inline(always)]
    pub fn linemult(&self) -> LINEMULT_R {
        LINEMULT_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bit 16 - Double buffer mode
    #[inline(always)]
    pub fn dbm(&self) -> DBM_R {
        DBM_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:19 - Line multi address wrapping modulo
    #[inline(always)]
    pub fn lmawm(&self) -> LMAWM_R {
        LMAWM_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 20 - Line multi address wrapping enable bit
    #[inline(always)]
    pub fn lmawe(&self) -> LMAWE_R {
        LMAWE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CPPCR")
            .field("format", &self.format())
            .field("swaprb", &self.swaprb())
            .field("linemult", &self.linemult())
            .field("dbm", &self.dbm())
            .field("lmawm", &self.lmawm())
            .field("lmawe", &self.lmawe())
            .finish()
    }
}
/**DCMIPP Pipe1 current pixel packer configuration register

You can [`read`](crate::Reg::read) this register and get [`p1cppcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:P1CPPCR)*/
pub struct P1CPPCRrs;
impl crate::RegisterSpec for P1CPPCRrs {
    type Ux = u32;
}
///`read()` method returns [`p1cppcr::R`](R) reader structure
impl crate::Readable for P1CPPCRrs {}
///`reset()` method sets P1CPPCR to value 0
impl crate::Resettable for P1CPPCRrs {}
