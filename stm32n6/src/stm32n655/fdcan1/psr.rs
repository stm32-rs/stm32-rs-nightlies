///Register `PSR` reader
pub type R = crate::R<PSRrs>;
///Register `PSR` writer
pub type W = crate::W<PSRrs>;
///Field `LEC` reader - Last error code
pub type LEC_R = crate::FieldReader;
///Field `ACT` reader - Activity
pub type ACT_R = crate::FieldReader;
///Field `EP` reader - Error passive
pub type EP_R = crate::BitReader;
///Field `EW` reader - Warning status
pub type EW_R = crate::BitReader;
///Field `BO` reader - Bus_Off status
pub type BO_R = crate::BitReader;
///Field `DLEC` reader - Data last error code
pub type DLEC_R = crate::FieldReader;
///Field `RESI` reader - ESI flag of last received FDCAN message
pub type RESI_R = crate::BitReader;
///Field `RESI` writer - ESI flag of last received FDCAN message
pub type RESI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RBRS` reader - BRS flag of last received FDCAN message
pub type RBRS_R = crate::BitReader;
///Field `RBRS` writer - BRS flag of last received FDCAN message
pub type RBRS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REDL` reader - Received FDCAN message
pub type REDL_R = crate::BitReader;
///Field `REDL` writer - Received FDCAN message
pub type REDL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PXE` reader - Protocol exception event
pub type PXE_R = crate::BitReader;
///Field `PXE` writer - Protocol exception event
pub type PXE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TDCV` reader - Transmitter delay compensation value
pub type TDCV_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - Last error code
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:4 - Activity
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - Error passive
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Warning status
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Bus_Off status
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:10 - Data last error code
    #[inline(always)]
    pub fn dlec(&self) -> DLEC_R {
        DLEC_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 11 - ESI flag of last received FDCAN message
    #[inline(always)]
    pub fn resi(&self) -> RESI_R {
        RESI_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - BRS flag of last received FDCAN message
    #[inline(always)]
    pub fn rbrs(&self) -> RBRS_R {
        RBRS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Received FDCAN message
    #[inline(always)]
    pub fn redl(&self) -> REDL_R {
        REDL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Protocol exception event
    #[inline(always)]
    pub fn pxe(&self) -> PXE_R {
        PXE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:22 - Transmitter delay compensation value
    #[inline(always)]
    pub fn tdcv(&self) -> TDCV_R {
        TDCV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSR")
            .field("lec", &self.lec())
            .field("act", &self.act())
            .field("ep", &self.ep())
            .field("ew", &self.ew())
            .field("bo", &self.bo())
            .field("dlec", &self.dlec())
            .field("resi", &self.resi())
            .field("rbrs", &self.rbrs())
            .field("redl", &self.redl())
            .field("pxe", &self.pxe())
            .field("tdcv", &self.tdcv())
            .finish()
    }
}
impl W {
    ///Bit 11 - ESI flag of last received FDCAN message
    #[inline(always)]
    pub fn resi(&mut self) -> RESI_W<PSRrs> {
        RESI_W::new(self, 11)
    }
    ///Bit 12 - BRS flag of last received FDCAN message
    #[inline(always)]
    pub fn rbrs(&mut self) -> RBRS_W<PSRrs> {
        RBRS_W::new(self, 12)
    }
    ///Bit 13 - Received FDCAN message
    #[inline(always)]
    pub fn redl(&mut self) -> REDL_W<PSRrs> {
        REDL_W::new(self, 13)
    }
    ///Bit 14 - Protocol exception event
    #[inline(always)]
    pub fn pxe(&mut self) -> PXE_W<PSRrs> {
        PXE_W::new(self, 14)
    }
}
/**FDCAN protocol status register

You can [`read`](crate::Reg::read) this register and get [`psr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#FDCAN1:PSR)*/
pub struct PSRrs;
impl crate::RegisterSpec for PSRrs {
    type Ux = u32;
}
///`read()` method returns [`psr::R`](R) reader structure
impl crate::Readable for PSRrs {}
///`write(|w| ..)` method takes [`psr::W`](W) writer structure
impl crate::Writable for PSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PSR to value 0x0707
impl crate::Resettable for PSRrs {
    const RESET_VALUE: u32 = 0x0707;
}
