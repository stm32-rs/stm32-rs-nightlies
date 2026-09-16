///Register `FDCAN_DBTP` reader
pub type R = crate::R<FDCAN_DBTPrs>;
///Register `FDCAN_DBTP` writer
pub type W = crate::W<FDCAN_DBTPrs>;
///Field `DSJW` reader - Synchronization jump width
pub type DSJW_R = crate::FieldReader;
///Field `DSJW` writer - Synchronization jump width
pub type DSJW_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DTSEG2` reader - Data time segment after sample point
pub type DTSEG2_R = crate::FieldReader;
///Field `DTSEG2` writer - Data time segment after sample point
pub type DTSEG2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DTSEG1` reader - Data time segment before sample point
pub type DTSEG1_R = crate::FieldReader;
///Field `DTSEG1` writer - Data time segment before sample point
pub type DTSEG1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DBRP` reader - Data bit rate prescaler
pub type DBRP_R = crate::FieldReader;
///Field `DBRP` writer - Data bit rate prescaler
pub type DBRP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**Transceiver delay compensation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDC {
    ///0: Transceiver delay compensation disabled
    B0x0 = 0,
    ///1: Transceiver delay compensation enabled
    B0x1 = 1,
}
impl From<TDC> for bool {
    #[inline(always)]
    fn from(variant: TDC) -> Self {
        variant as u8 != 0
    }
}
///Field `TDC` reader - Transceiver delay compensation
pub type TDC_R = crate::BitReader<TDC>;
impl TDC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TDC {
        match self.bits {
            false => TDC::B0x0,
            true => TDC::B0x1,
        }
    }
    ///Transceiver delay compensation disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TDC::B0x0
    }
    ///Transceiver delay compensation enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TDC::B0x1
    }
}
///Field `TDC` writer - Transceiver delay compensation
pub type TDC_W<'a, REG> = crate::BitWriter<'a, REG, TDC>;
impl<'a, REG> TDC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transceiver delay compensation disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TDC::B0x0)
    }
    ///Transceiver delay compensation enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TDC::B0x1)
    }
}
impl R {
    ///Bits 0:3 - Synchronization jump width
    #[inline(always)]
    pub fn dsjw(&self) -> DSJW_R {
        DSJW_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Data time segment after sample point
    #[inline(always)]
    pub fn dtseg2(&self) -> DTSEG2_R {
        DTSEG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:12 - Data time segment before sample point
    #[inline(always)]
    pub fn dtseg1(&self) -> DTSEG1_R {
        DTSEG1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:20 - Data bit rate prescaler
    #[inline(always)]
    pub fn dbrp(&self) -> DBRP_R {
        DBRP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 23 - Transceiver delay compensation
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_DBTP")
            .field("dsjw", &self.dsjw())
            .field("dtseg2", &self.dtseg2())
            .field("dtseg1", &self.dtseg1())
            .field("dbrp", &self.dbrp())
            .field("tdc", &self.tdc())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Synchronization jump width
    #[inline(always)]
    pub fn dsjw(&mut self) -> DSJW_W<'_, FDCAN_DBTPrs> {
        DSJW_W::new(self, 0)
    }
    ///Bits 4:7 - Data time segment after sample point
    #[inline(always)]
    pub fn dtseg2(&mut self) -> DTSEG2_W<'_, FDCAN_DBTPrs> {
        DTSEG2_W::new(self, 4)
    }
    ///Bits 8:12 - Data time segment before sample point
    #[inline(always)]
    pub fn dtseg1(&mut self) -> DTSEG1_W<'_, FDCAN_DBTPrs> {
        DTSEG1_W::new(self, 8)
    }
    ///Bits 16:20 - Data bit rate prescaler
    #[inline(always)]
    pub fn dbrp(&mut self) -> DBRP_W<'_, FDCAN_DBTPrs> {
        DBRP_W::new(self, 16)
    }
    ///Bit 23 - Transceiver delay compensation
    #[inline(always)]
    pub fn tdc(&mut self) -> TDC_W<'_, FDCAN_DBTPrs> {
        TDC_W::new(self, 23)
    }
}
/**FDCAN data bit timing and prescaler register

You can [`read`](crate::Reg::read) this register and get [`fdcan_dbtp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_dbtp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_DBTP)*/
pub struct FDCAN_DBTPrs;
impl crate::RegisterSpec for FDCAN_DBTPrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_dbtp::R`](R) reader structure
impl crate::Readable for FDCAN_DBTPrs {}
///`write(|w| ..)` method takes [`fdcan_dbtp::W`](W) writer structure
impl crate::Writable for FDCAN_DBTPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDCAN_DBTP to value 0x0a33
impl crate::Resettable for FDCAN_DBTPrs {
    const RESET_VALUE: u32 = 0x0a33;
}
