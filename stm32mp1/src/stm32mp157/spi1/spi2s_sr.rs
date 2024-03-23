#[doc = "Register `SPI2S_SR` reader"]
pub type R = crate::R<SPI2S_SRrs>;
#[doc = "Field `RXP` reader - RXP"]
pub type RXP_R = crate::BitReader;
#[doc = "Field `TXP` reader - TXP"]
pub type TXP_R = crate::BitReader;
#[doc = "Field `DXP` reader - DXP"]
pub type DXP_R = crate::BitReader;
#[doc = "Field `EOT` reader - EOT"]
pub type EOT_R = crate::BitReader;
#[doc = "Field `TXTF` reader - TXTF"]
pub type TXTF_R = crate::BitReader;
#[doc = "Field `UDR` reader - UDR"]
pub type UDR_R = crate::BitReader;
#[doc = "Field `OVR` reader - OVR"]
pub type OVR_R = crate::BitReader;
#[doc = "Field `CRCE` reader - CRCE"]
pub type CRCE_R = crate::BitReader;
#[doc = "Field `TIFRE` reader - TIFRE"]
pub type TIFRE_R = crate::BitReader;
#[doc = "Field `MODF` reader - MODF"]
pub type MODF_R = crate::BitReader;
#[doc = "Field `TSERF` reader - TSERF"]
pub type TSERF_R = crate::BitReader;
#[doc = "Field `SUSP` reader - SUSP"]
pub type SUSP_R = crate::BitReader;
#[doc = "Field `TXC` reader - TXC"]
pub type TXC_R = crate::BitReader;
#[doc = "Field `RXPLVL` reader - RXPLVL"]
pub type RXPLVL_R = crate::FieldReader;
#[doc = "Field `RXWNE` reader - RXWNE"]
pub type RXWNE_R = crate::BitReader;
#[doc = "Field `CTSIZE` reader - CTSIZE"]
pub type CTSIZE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - RXP"]
    #[inline(always)]
    pub fn rxp(&self) -> RXP_R {
        RXP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXP"]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DXP"]
    #[inline(always)]
    pub fn dxp(&self) -> DXP_R {
        DXP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOT"]
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXTF"]
    #[inline(always)]
    pub fn txtf(&self) -> TXTF_R {
        TXTF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UDR"]
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OVR"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRCE"]
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIFRE"]
    #[inline(always)]
    pub fn tifre(&self) -> TIFRE_R {
        TIFRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MODF"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TSERF"]
    #[inline(always)]
    pub fn tserf(&self) -> TSERF_R {
        TSERF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SUSP"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TXC"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - RXPLVL"]
    #[inline(always)]
    pub fn rxplvl(&self) -> RXPLVL_R {
        RXPLVL_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - RXWNE"]
    #[inline(always)]
    pub fn rxwne(&self) -> RXWNE_R {
        RXWNE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - CTSIZE"]
    #[inline(always)]
    pub fn ctsize(&self) -> CTSIZE_R {
        CTSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "SPI/I2S status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi2s_sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI2S_SRrs;
impl crate::RegisterSpec for SPI2S_SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi2s_sr::R`](R) reader structure"]
impl crate::Readable for SPI2S_SRrs {}
#[doc = "`reset()` method sets SPI2S_SR to value 0x1002"]
impl crate::Resettable for SPI2S_SRrs {
    const RESET_VALUE: u32 = 0x1002;
}
