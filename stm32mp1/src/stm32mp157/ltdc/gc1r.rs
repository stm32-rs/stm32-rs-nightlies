///Register `GC1R` reader
pub type R = crate::R<GC1Rrs>;
///Field `WBCH` reader - WBCH
pub type WBCH_R = crate::FieldReader;
///Field `WGCH` reader - WGCH
pub type WGCH_R = crate::FieldReader;
///Field `WRCH` reader - WRCH
pub type WRCH_R = crate::FieldReader;
///Field `PRBEN` reader - PRBEN
pub type PRBEN_R = crate::BitReader;
///Field `DT` reader - DT
pub type DT_R = crate::FieldReader;
///Field `GCT` reader - GCT
pub type GCT_R = crate::FieldReader;
///Field `SHREN` reader - SHREN
pub type SHREN_R = crate::BitReader;
///Field `BCP` reader - BCP
pub type BCP_R = crate::BitReader;
///Field `BBEN` reader - BBEN
pub type BBEN_R = crate::BitReader;
///Field `LNIP` reader - LNIP
pub type LNIP_R = crate::BitReader;
///Field `TP` reader - TP
pub type TP_R = crate::BitReader;
///Field `IPP` reader - IPP
pub type IPP_R = crate::BitReader;
///Field `SPP` reader - SPP
pub type SPP_R = crate::BitReader;
///Field `DWP` reader - DWP
pub type DWP_R = crate::BitReader;
///Field `STREN` reader - STREN
pub type STREN_R = crate::BitReader;
///Field `BMEN` reader - BMEN
pub type BMEN_R = crate::BitReader;
impl R {
    ///Bits 0:3 - WBCH
    #[inline(always)]
    pub fn wbch(&self) -> WBCH_R {
        WBCH_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - WGCH
    #[inline(always)]
    pub fn wgch(&self) -> WGCH_R {
        WGCH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - WRCH
    #[inline(always)]
    pub fn wrch(&self) -> WRCH_R {
        WRCH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - PRBEN
    #[inline(always)]
    pub fn prben(&self) -> PRBEN_R {
        PRBEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 14:15 - DT
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 17:19 - GCT
    #[inline(always)]
    pub fn gct(&self) -> GCT_R {
        GCT_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 21 - SHREN
    #[inline(always)]
    pub fn shren(&self) -> SHREN_R {
        SHREN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - BCP
    #[inline(always)]
    pub fn bcp(&self) -> BCP_R {
        BCP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - BBEN
    #[inline(always)]
    pub fn bben(&self) -> BBEN_R {
        BBEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - LNIP
    #[inline(always)]
    pub fn lnip(&self) -> LNIP_R {
        LNIP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - TP
    #[inline(always)]
    pub fn tp(&self) -> TP_R {
        TP_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - IPP
    #[inline(always)]
    pub fn ipp(&self) -> IPP_R {
        IPP_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - SPP
    #[inline(always)]
    pub fn spp(&self) -> SPP_R {
        SPP_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - DWP
    #[inline(always)]
    pub fn dwp(&self) -> DWP_R {
        DWP_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - STREN
    #[inline(always)]
    pub fn stren(&self) -> STREN_R {
        STREN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - BMEN
    #[inline(always)]
    pub fn bmen(&self) -> BMEN_R {
        BMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GC1R")
            .field("wbch", &self.wbch())
            .field("wgch", &self.wgch())
            .field("wrch", &self.wrch())
            .field("prben", &self.prben())
            .field("dt", &self.dt())
            .field("gct", &self.gct())
            .field("shren", &self.shren())
            .field("bcp", &self.bcp())
            .field("bben", &self.bben())
            .field("lnip", &self.lnip())
            .field("tp", &self.tp())
            .field("ipp", &self.ipp())
            .field("spp", &self.spp())
            .field("dwp", &self.dwp())
            .field("stren", &self.stren())
            .field("bmen", &self.bmen())
            .finish()
    }
}
/**LTDC global configuration 1 register

You can [`read`](crate::Reg::read) this register and get [`gc1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LTDC:GC1R)*/
pub struct GC1Rrs;
impl crate::RegisterSpec for GC1Rrs {
    type Ux = u32;
}
///`read()` method returns [`gc1r::R`](R) reader structure
impl crate::Readable for GC1Rrs {}
///`reset()` method sets GC1R to value 0x6be2_d888
impl crate::Resettable for GC1Rrs {
    const RESET_VALUE: u32 = 0x6be2_d888;
}
