///Register `PCR` reader
pub type R = crate::R<PCRrs>;
///Register `PCR` writer
pub type W = crate::W<PCRrs>;
/**EoTp transmission enable This bit enables the EoTP transmission.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETTXE {
    ///0: EoTp transmission is disabled.
    B0x0 = 0,
    ///1: EoTp transmission is enabled.
    B0x1 = 1,
}
impl From<ETTXE> for bool {
    #[inline(always)]
    fn from(variant: ETTXE) -> Self {
        variant as u8 != 0
    }
}
///Field `ETTXE` reader - EoTp transmission enable This bit enables the EoTP transmission.
pub type ETTXE_R = crate::BitReader<ETTXE>;
impl ETTXE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ETTXE {
        match self.bits {
            false => ETTXE::B0x0,
            true => ETTXE::B0x1,
        }
    }
    ///EoTp transmission is disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETTXE::B0x0
    }
    ///EoTp transmission is enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETTXE::B0x1
    }
}
///Field `ETTXE` writer - EoTp transmission enable This bit enables the EoTP transmission.
pub type ETTXE_W<'a, REG> = crate::BitWriter<'a, REG, ETTXE>;
impl<'a, REG> ETTXE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///EoTp transmission is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ETTXE::B0x0)
    }
    ///EoTp transmission is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ETTXE::B0x1)
    }
}
/**EoTp reception enable This bit enables the EoTp reception.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETRXE {
    ///0: EoTp reception is disabled.
    B0x0 = 0,
    ///1: EoTp reception is enabled.
    B0x1 = 1,
}
impl From<ETRXE> for bool {
    #[inline(always)]
    fn from(variant: ETRXE) -> Self {
        variant as u8 != 0
    }
}
///Field `ETRXE` reader - EoTp reception enable This bit enables the EoTp reception.
pub type ETRXE_R = crate::BitReader<ETRXE>;
impl ETRXE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ETRXE {
        match self.bits {
            false => ETRXE::B0x0,
            true => ETRXE::B0x1,
        }
    }
    ///EoTp reception is disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETRXE::B0x0
    }
    ///EoTp reception is enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETRXE::B0x1
    }
}
///Field `ETRXE` writer - EoTp reception enable This bit enables the EoTp reception.
pub type ETRXE_W<'a, REG> = crate::BitWriter<'a, REG, ETRXE>;
impl<'a, REG> ETRXE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///EoTp reception is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ETRXE::B0x0)
    }
    ///EoTp reception is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ETRXE::B0x1)
    }
}
/**Bus-turn-around enable This bit enables the bus-turn-around (BTA) request.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BTAE {
    ///0: Bus-turn-around request is disabled.
    B0x0 = 0,
    ///1: Bus-turn-around request is enabled.
    B0x1 = 1,
}
impl From<BTAE> for bool {
    #[inline(always)]
    fn from(variant: BTAE) -> Self {
        variant as u8 != 0
    }
}
///Field `BTAE` reader - Bus-turn-around enable This bit enables the bus-turn-around (BTA) request.
pub type BTAE_R = crate::BitReader<BTAE>;
impl BTAE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BTAE {
        match self.bits {
            false => BTAE::B0x0,
            true => BTAE::B0x1,
        }
    }
    ///Bus-turn-around request is disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BTAE::B0x0
    }
    ///Bus-turn-around request is enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BTAE::B0x1
    }
}
///Field `BTAE` writer - Bus-turn-around enable This bit enables the bus-turn-around (BTA) request.
pub type BTAE_W<'a, REG> = crate::BitWriter<'a, REG, BTAE>;
impl<'a, REG> BTAE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bus-turn-around request is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BTAE::B0x0)
    }
    ///Bus-turn-around request is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BTAE::B0x1)
    }
}
/**ECC reception enable This bit enables the ECC reception, error correction and reporting.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCRXE {
    ///0: ECC reception is disabled.
    B0x0 = 0,
    ///1: ECC reception is enabled.
    B0x1 = 1,
}
impl From<ECCRXE> for bool {
    #[inline(always)]
    fn from(variant: ECCRXE) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCRXE` reader - ECC reception enable This bit enables the ECC reception, error correction and reporting.
pub type ECCRXE_R = crate::BitReader<ECCRXE>;
impl ECCRXE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ECCRXE {
        match self.bits {
            false => ECCRXE::B0x0,
            true => ECCRXE::B0x1,
        }
    }
    ///ECC reception is disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ECCRXE::B0x0
    }
    ///ECC reception is enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ECCRXE::B0x1
    }
}
///Field `ECCRXE` writer - ECC reception enable This bit enables the ECC reception, error correction and reporting.
pub type ECCRXE_W<'a, REG> = crate::BitWriter<'a, REG, ECCRXE>;
impl<'a, REG> ECCRXE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ECC reception is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ECCRXE::B0x0)
    }
    ///ECC reception is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ECCRXE::B0x1)
    }
}
/**CRC reception enable This bit enables the CRC reception and error reporting.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCRXE {
    ///0: CRC reception is disabled.
    B0x0 = 0,
    ///1: CRC reception is enabled.
    B0x1 = 1,
}
impl From<CRCRXE> for bool {
    #[inline(always)]
    fn from(variant: CRCRXE) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCRXE` reader - CRC reception enable This bit enables the CRC reception and error reporting.
pub type CRCRXE_R = crate::BitReader<CRCRXE>;
impl CRCRXE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCRXE {
        match self.bits {
            false => CRCRXE::B0x0,
            true => CRCRXE::B0x1,
        }
    }
    ///CRC reception is disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRCRXE::B0x0
    }
    ///CRC reception is enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRCRXE::B0x1
    }
}
///Field `CRCRXE` writer - CRC reception enable This bit enables the CRC reception and error reporting.
pub type CRCRXE_W<'a, REG> = crate::BitWriter<'a, REG, CRCRXE>;
impl<'a, REG> CRCRXE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CRC reception is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCRXE::B0x0)
    }
    ///CRC reception is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCRXE::B0x1)
    }
}
impl R {
    ///Bit 0 - EoTp transmission enable This bit enables the EoTP transmission.
    #[inline(always)]
    pub fn ettxe(&self) -> ETTXE_R {
        ETTXE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EoTp reception enable This bit enables the EoTp reception.
    #[inline(always)]
    pub fn etrxe(&self) -> ETRXE_R {
        ETRXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Bus-turn-around enable This bit enables the bus-turn-around (BTA) request.
    #[inline(always)]
    pub fn btae(&self) -> BTAE_R {
        BTAE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ECC reception enable This bit enables the ECC reception, error correction and reporting.
    #[inline(always)]
    pub fn eccrxe(&self) -> ECCRXE_R {
        ECCRXE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CRC reception enable This bit enables the CRC reception and error reporting.
    #[inline(always)]
    pub fn crcrxe(&self) -> CRCRXE_R {
        CRCRXE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCR")
            .field("ettxe", &self.ettxe())
            .field("etrxe", &self.etrxe())
            .field("btae", &self.btae())
            .field("eccrxe", &self.eccrxe())
            .field("crcrxe", &self.crcrxe())
            .finish()
    }
}
impl W {
    ///Bit 0 - EoTp transmission enable This bit enables the EoTP transmission.
    #[inline(always)]
    pub fn ettxe(&mut self) -> ETTXE_W<PCRrs> {
        ETTXE_W::new(self, 0)
    }
    ///Bit 1 - EoTp reception enable This bit enables the EoTp reception.
    #[inline(always)]
    pub fn etrxe(&mut self) -> ETRXE_W<PCRrs> {
        ETRXE_W::new(self, 1)
    }
    ///Bit 2 - Bus-turn-around enable This bit enables the bus-turn-around (BTA) request.
    #[inline(always)]
    pub fn btae(&mut self) -> BTAE_W<PCRrs> {
        BTAE_W::new(self, 2)
    }
    ///Bit 3 - ECC reception enable This bit enables the ECC reception, error correction and reporting.
    #[inline(always)]
    pub fn eccrxe(&mut self) -> ECCRXE_W<PCRrs> {
        ECCRXE_W::new(self, 3)
    }
    ///Bit 4 - CRC reception enable This bit enables the CRC reception and error reporting.
    #[inline(always)]
    pub fn crcrxe(&mut self) -> CRCRXE_W<PCRrs> {
        CRCRXE_W::new(self, 4)
    }
}
/**DSI Host protocol configuration register

You can [`read`](crate::Reg::read) this register and get [`pcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:PCR)*/
pub struct PCRrs;
impl crate::RegisterSpec for PCRrs {
    type Ux = u32;
}
///`read()` method returns [`pcr::R`](R) reader structure
impl crate::Readable for PCRrs {}
///`write(|w| ..)` method takes [`pcr::W`](W) writer structure
impl crate::Writable for PCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PCR to value 0
impl crate::Resettable for PCRrs {
    const RESET_VALUE: u32 = 0;
}
