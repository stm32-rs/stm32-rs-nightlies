#[doc = "Register `FDCAN_TXEFS` reader"]
pub type R = crate::R<FDCAN_TXEFSrs>;
#[doc = "Field `EFFL` reader - Event FIFO Fill Level"]
pub type EFFL_R = crate::FieldReader;
#[doc = "Field `EFGI` reader - Event FIFO Get Index."]
pub type EFGI_R = crate::FieldReader;
#[doc = "Field `EFPI` reader - Event FIFO Put Index"]
pub type EFPI_R = crate::FieldReader;
#[doc = "Field `EFF` reader - Event FIFO Full."]
pub type EFF_R = crate::BitReader;
#[doc = "Field `TEFL` reader - Tx Event FIFO Element Lost."]
pub type TEFL_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Event FIFO Fill Level"]
    #[inline(always)]
    pub fn effl(&self) -> EFFL_R {
        EFFL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - Event FIFO Get Index."]
    #[inline(always)]
    pub fn efgi(&self) -> EFGI_R {
        EFGI_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Event FIFO Put Index"]
    #[inline(always)]
    pub fn efpi(&self) -> EFPI_R {
        EFPI_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Event FIFO Full."]
    #[inline(always)]
    pub fn eff(&self) -> EFF_R {
        EFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Tx Event FIFO Element Lost."]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "FDCAN Tx Event FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txefs::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TXEFSrs;
impl crate::RegisterSpec for FDCAN_TXEFSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txefs::R`](R) reader structure"]
impl crate::Readable for FDCAN_TXEFSrs {}
#[doc = "`reset()` method sets FDCAN_TXEFS to value 0"]
impl crate::Resettable for FDCAN_TXEFSrs {
    const RESET_VALUE: u32 = 0;
}
