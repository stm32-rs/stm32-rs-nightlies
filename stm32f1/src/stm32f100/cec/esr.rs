///Register `ESR` reader
pub type R = crate::R<ESRrs>;
///Field `BTE` reader - Bit timing error
pub type BTE_R = crate::BitReader;
///Field `BPE` reader - Bit period error
pub type BPE_R = crate::BitReader;
///Field `RBTFE` reader - Rx block transfer finished error
pub type RBTFE_R = crate::BitReader;
///Field `SBE` reader - Start bit error
pub type SBE_R = crate::BitReader;
///Field `ACKE` reader - Block acknowledge error
pub type ACKE_R = crate::BitReader;
///Field `LINE` reader - Line error
pub type LINE_R = crate::BitReader;
///Field `TBTFE` reader - Tx block transfer finished error
pub type TBTFE_R = crate::BitReader;
impl R {
    ///Bit 0 - Bit timing error
    #[inline(always)]
    pub fn bte(&self) -> BTE_R {
        BTE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Bit period error
    #[inline(always)]
    pub fn bpe(&self) -> BPE_R {
        BPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rx block transfer finished error
    #[inline(always)]
    pub fn rbtfe(&self) -> RBTFE_R {
        RBTFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Start bit error
    #[inline(always)]
    pub fn sbe(&self) -> SBE_R {
        SBE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Block acknowledge error
    #[inline(always)]
    pub fn acke(&self) -> ACKE_R {
        ACKE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Line error
    #[inline(always)]
    pub fn line(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Tx block transfer finished error
    #[inline(always)]
    pub fn tbtfe(&self) -> TBTFE_R {
        TBTFE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESR")
            .field("bte", &self.bte())
            .field("bpe", &self.bpe())
            .field("rbtfe", &self.rbtfe())
            .field("sbe", &self.sbe())
            .field("acke", &self.acke())
            .field("line", &self.line())
            .field("tbtfe", &self.tbtfe())
            .finish()
    }
}
/**CEC error status register

You can [`read`](crate::Reg::read) this register and get [`esr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#CEC:ESR)*/
pub struct ESRrs;
impl crate::RegisterSpec for ESRrs {
    type Ux = u32;
}
///`read()` method returns [`esr::R`](R) reader structure
impl crate::Readable for ESRrs {}
///`reset()` method sets ESR to value 0
impl crate::Resettable for ESRrs {}
