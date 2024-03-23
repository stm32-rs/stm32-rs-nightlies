#[doc = "Register `TXEFS` reader"]
pub type R = crate::R<TXEFSrs>;
#[doc = "Field `EFFL` reader - EFFL"]
pub type EFFL_R = crate::FieldReader;
#[doc = "Field `EFGI` reader - EFGI"]
pub type EFGI_R = crate::FieldReader;
#[doc = "Field `EFPI` reader - EFPI"]
pub type EFPI_R = crate::FieldReader;
#[doc = "Field `EFF` reader - EFF"]
pub type EFF_R = crate::BitReader;
#[doc = "Field `TEFL` reader - TEFL"]
pub type TEFL_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - EFFL"]
    #[inline(always)]
    pub fn effl(&self) -> EFFL_R {
        EFFL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - EFGI"]
    #[inline(always)]
    pub fn efgi(&self) -> EFGI_R {
        EFGI_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - EFPI"]
    #[inline(always)]
    pub fn efpi(&self) -> EFPI_R {
        EFPI_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - EFF"]
    #[inline(always)]
    pub fn eff(&self) -> EFF_R {
        EFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TEFL"]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "FDCAN Tx Event FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefs::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXEFSrs;
impl crate::RegisterSpec for TXEFSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txefs::R`](R) reader structure"]
impl crate::Readable for TXEFSrs {}
#[doc = "`reset()` method sets TXEFS to value 0"]
impl crate::Resettable for TXEFSrs {
    const RESET_VALUE: u32 = 0;
}
