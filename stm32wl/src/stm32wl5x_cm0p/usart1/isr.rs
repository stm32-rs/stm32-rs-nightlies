#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Field `PE` reader - PE"]
pub type PE_R = crate::BitReader;
#[doc = "Field `FE` reader - FE"]
pub type FE_R = crate::BitReader;
#[doc = "Field `NE` reader - NE"]
pub type NE_R = crate::BitReader;
#[doc = "Field `ORE` reader - ORE"]
pub type ORE_R = crate::BitReader;
#[doc = "Field `IDLE` reader - IDLE"]
pub type IDLE_R = crate::BitReader;
#[doc = "Field `RXNE` reader - RXNE"]
pub type RXNE_R = crate::BitReader;
#[doc = "Field `TC` reader - TC"]
pub type TC_R = crate::BitReader;
#[doc = "Field `TXE` reader - TXE"]
pub type TXE_R = crate::BitReader;
#[doc = "Field `LBDF` reader - LBDF"]
pub type LBDF_R = crate::BitReader;
#[doc = "Field `CTSIF` reader - CTSIF"]
pub type CTSIF_R = crate::BitReader;
#[doc = "Field `CTS` reader - CTS"]
pub type CTS_R = crate::BitReader;
#[doc = "Field `RTOF` reader - RTOF"]
pub type RTOF_R = crate::BitReader;
#[doc = "Field `EOBF` reader - EOBF"]
pub type EOBF_R = crate::BitReader;
#[doc = "Field `UDR` reader - UDR"]
pub type UDR_R = crate::BitReader;
#[doc = "Field `ABRE` reader - ABRE"]
pub type ABRE_R = crate::BitReader;
#[doc = "Field `ABRF` reader - ABRF"]
pub type ABRF_R = crate::BitReader;
#[doc = "Field `BUSY` reader - BUSY"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `CMF` reader - CMF"]
pub type CMF_R = crate::BitReader;
#[doc = "Field `SBKF` reader - SBKF"]
pub type SBKF_R = crate::BitReader;
#[doc = "Field `RWU` reader - RWU"]
pub type RWU_R = crate::BitReader;
#[doc = "Field `WUF` reader - WUF"]
pub type WUF_R = crate::BitReader;
#[doc = "Field `TEACK` reader - TEACK"]
pub type TEACK_R = crate::BitReader;
#[doc = "Field `REACK` reader - REACK"]
pub type REACK_R = crate::BitReader;
#[doc = "Field `TXFE` reader - TXFE"]
pub type TXFE_R = crate::BitReader;
#[doc = "Field `RXFF` reader - RXFF"]
pub type RXFF_R = crate::BitReader;
#[doc = "TCBGT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCBGT {
    #[doc = "0: Transmission not completed"]
    NotCompleted = 0,
    #[doc = "1: Transmission has completed"]
    Completed = 1,
}
impl From<TCBGT> for bool {
    #[inline(always)]
    fn from(variant: TCBGT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCBGT` reader - TCBGT"]
pub type TCBGT_R = crate::BitReader<TCBGT>;
impl TCBGT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCBGT {
        match self.bits {
            false => TCBGT::NotCompleted,
            true => TCBGT::Completed,
        }
    }
    #[doc = "Transmission not completed"]
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == TCBGT::NotCompleted
    }
    #[doc = "Transmission has completed"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == TCBGT::Completed
    }
}
#[doc = "Field `RXFT` reader - RXFT"]
pub type RXFT_R = crate::BitReader;
#[doc = "Field `TXFT` reader - TXFT"]
pub type TXFT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PE"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FE"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NE"]
    #[inline(always)]
    pub fn ne(&self) -> NE_R {
        NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ORE"]
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXNE"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TC"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LBDF"]
    #[inline(always)]
    pub fn lbdf(&self) -> LBDF_R {
        LBDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTSIF"]
    #[inline(always)]
    pub fn ctsif(&self) -> CTSIF_R {
        CTSIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RTOF"]
    #[inline(always)]
    pub fn rtof(&self) -> RTOF_R {
        RTOF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EOBF"]
    #[inline(always)]
    pub fn eobf(&self) -> EOBF_R {
        EOBF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - UDR"]
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ABRE"]
    #[inline(always)]
    pub fn abre(&self) -> ABRE_R {
        ABRE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ABRF"]
    #[inline(always)]
    pub fn abrf(&self) -> ABRF_R {
        ABRF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CMF"]
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SBKF"]
    #[inline(always)]
    pub fn sbkf(&self) -> SBKF_R {
        SBKF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RWU"]
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - WUF"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TEACK"]
    #[inline(always)]
    pub fn teack(&self) -> TEACK_R {
        TEACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - REACK"]
    #[inline(always)]
    pub fn reack(&self) -> REACK_R {
        REACK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TXFE"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - RXFF"]
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TCBGT"]
    #[inline(always)]
    pub fn tcbgt(&self) -> TCBGT_R {
        TCBGT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - RXFT"]
    #[inline(always)]
    pub fn rxft(&self) -> RXFT_R {
        RXFT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TXFT"]
    #[inline(always)]
    pub fn txft(&self) -> TXFT_R {
        TXFT_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISRrs {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0;
}
