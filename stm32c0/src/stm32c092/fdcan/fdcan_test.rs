///Register `FDCAN_TEST` reader
pub type R = crate::R<FDCAN_TESTrs>;
///Register `FDCAN_TEST` writer
pub type W = crate::W<FDCAN_TESTrs>;
/**Loop-back mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBCK {
    ///0: Reset value, loop-back mode is disabled
    B0x0 = 0,
    ///1: Loop-back mode is enabled (see Power-down (Sleep mode))
    B0x1 = 1,
}
impl From<LBCK> for bool {
    #[inline(always)]
    fn from(variant: LBCK) -> Self {
        variant as u8 != 0
    }
}
///Field `LBCK` reader - Loop-back mode
pub type LBCK_R = crate::BitReader<LBCK>;
impl LBCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LBCK {
        match self.bits {
            false => LBCK::B0x0,
            true => LBCK::B0x1,
        }
    }
    ///Reset value, loop-back mode is disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LBCK::B0x0
    }
    ///Loop-back mode is enabled (see Power-down (Sleep mode))
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LBCK::B0x1
    }
}
///Field `LBCK` writer - Loop-back mode
pub type LBCK_W<'a, REG> = crate::BitWriter<'a, REG, LBCK>;
impl<'a, REG> LBCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset value, loop-back mode is disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LBCK::B0x0)
    }
    ///Loop-back mode is enabled (see Power-down (Sleep mode))
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LBCK::B0x1)
    }
}
/**Control of transmit pin

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX {
    ///0: Reset value, FDCANx_TX TX is controlled by the CAN core, updated at the end of the CAN bit time
    B0x0 = 0,
    ///1: Sample point can be monitored at pin FDCANx_TX
    B0x1 = 1,
    ///2: Dominant (0) level at pin FDCANx_TX
    B0x2 = 2,
    ///3: Recessive (1) at pin FDCANx_TX
    B0x3 = 3,
}
impl From<TX> for u8 {
    #[inline(always)]
    fn from(variant: TX) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TX {
    type Ux = u8;
}
impl crate::IsEnum for TX {}
///Field `TX` reader - Control of transmit pin
pub type TX_R = crate::FieldReader<TX>;
impl TX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TX {
        match self.bits {
            0 => TX::B0x0,
            1 => TX::B0x1,
            2 => TX::B0x2,
            3 => TX::B0x3,
            _ => unreachable!(),
        }
    }
    ///Reset value, FDCANx_TX TX is controlled by the CAN core, updated at the end of the CAN bit time
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TX::B0x0
    }
    ///Sample point can be monitored at pin FDCANx_TX
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TX::B0x1
    }
    ///Dominant (0) level at pin FDCANx_TX
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TX::B0x2
    }
    ///Recessive (1) at pin FDCANx_TX
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == TX::B0x3
    }
}
///Field `TX` writer - Control of transmit pin
pub type TX_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TX, crate::Safe>;
impl<'a, REG> TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Reset value, FDCANx_TX TX is controlled by the CAN core, updated at the end of the CAN bit time
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TX::B0x0)
    }
    ///Sample point can be monitored at pin FDCANx_TX
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TX::B0x1)
    }
    ///Dominant (0) level at pin FDCANx_TX
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TX::B0x2)
    }
    ///Recessive (1) at pin FDCANx_TX
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TX::B0x3)
    }
}
/**Receive pin

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX {
    ///0: The CAN bus is dominant (FDCANx_RX=0)
    B0x0 = 0,
    ///1: The CAN bus is recessive (FDCANx_RX=1)
    B0x1 = 1,
}
impl From<RX> for bool {
    #[inline(always)]
    fn from(variant: RX) -> Self {
        variant as u8 != 0
    }
}
///Field `RX` reader - Receive pin
pub type RX_R = crate::BitReader<RX>;
impl RX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RX {
        match self.bits {
            false => RX::B0x0,
            true => RX::B0x1,
        }
    }
    ///The CAN bus is dominant (FDCANx_RX=0)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RX::B0x0
    }
    ///The CAN bus is recessive (FDCANx_RX=1)
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RX::B0x1
    }
}
impl R {
    ///Bit 4 - Loop-back mode
    #[inline(always)]
    pub fn lbck(&self) -> LBCK_R {
        LBCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - Control of transmit pin
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Receive pin
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TEST")
            .field("lbck", &self.lbck())
            .field("tx", &self.tx())
            .field("rx", &self.rx())
            .finish()
    }
}
impl W {
    ///Bit 4 - Loop-back mode
    #[inline(always)]
    pub fn lbck(&mut self) -> LBCK_W<'_, FDCAN_TESTrs> {
        LBCK_W::new(self, 4)
    }
    ///Bits 5:6 - Control of transmit pin
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W<'_, FDCAN_TESTrs> {
        TX_W::new(self, 5)
    }
}
/**FDCAN test register

You can [`read`](crate::Reg::read) this register and get [`fdcan_test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_TEST)*/
pub struct FDCAN_TESTrs;
impl crate::RegisterSpec for FDCAN_TESTrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_test::R`](R) reader structure
impl crate::Readable for FDCAN_TESTrs {}
///`write(|w| ..)` method takes [`fdcan_test::W`](W) writer structure
impl crate::Writable for FDCAN_TESTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDCAN_TEST to value 0
impl crate::Resettable for FDCAN_TESTrs {}
