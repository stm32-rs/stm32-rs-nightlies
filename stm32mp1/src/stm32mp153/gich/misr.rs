///Register `MISR` reader
pub type R = crate::R<MISRrs>;
///Field `EOI` reader - EOI
pub type EOI_R = crate::BitReader;
///Field `U` reader - U
pub type U_R = crate::BitReader;
///Field `LRENP` reader - LRENP
pub type LRENP_R = crate::BitReader;
///Field `NP` reader - NP
pub type NP_R = crate::BitReader;
///Field `VGRP0E` reader - VGRP0E
pub type VGRP0E_R = crate::BitReader;
///Field `VGRP0D` reader - VGRP0D
pub type VGRP0D_R = crate::BitReader;
///Field `VGRP1E` reader - VGRP1E
pub type VGRP1E_R = crate::BitReader;
///Field `VGRP1D` reader - VGRP1D
pub type VGRP1D_R = crate::BitReader;
impl R {
    ///Bit 0 - EOI
    #[inline(always)]
    pub fn eoi(&self) -> EOI_R {
        EOI_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - U
    #[inline(always)]
    pub fn u(&self) -> U_R {
        U_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LRENP
    #[inline(always)]
    pub fn lrenp(&self) -> LRENP_R {
        LRENP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - NP
    #[inline(always)]
    pub fn np(&self) -> NP_R {
        NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - VGRP0E
    #[inline(always)]
    pub fn vgrp0e(&self) -> VGRP0E_R {
        VGRP0E_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - VGRP0D
    #[inline(always)]
    pub fn vgrp0d(&self) -> VGRP0D_R {
        VGRP0D_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - VGRP1E
    #[inline(always)]
    pub fn vgrp1e(&self) -> VGRP1E_R {
        VGRP1E_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - VGRP1D
    #[inline(always)]
    pub fn vgrp1d(&self) -> VGRP1D_R {
        VGRP1D_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISR")
            .field("eoi", &self.eoi())
            .field("u", &self.u())
            .field("lrenp", &self.lrenp())
            .field("np", &self.np())
            .field("vgrp0e", &self.vgrp0e())
            .field("vgrp0d", &self.vgrp0d())
            .field("vgrp1e", &self.vgrp1e())
            .field("vgrp1d", &self.vgrp1d())
            .finish()
    }
}
/**GICH maintenance interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICH:MISR)*/
pub struct MISRrs;
impl crate::RegisterSpec for MISRrs {
    type Ux = u32;
}
///`read()` method returns [`misr::R`](R) reader structure
impl crate::Readable for MISRrs {}
///`reset()` method sets MISR to value 0
impl crate::Resettable for MISRrs {}
