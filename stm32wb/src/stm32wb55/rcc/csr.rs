///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
/**LSI1 oscillator enabled

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSI1ON {
    ///0: LSI oscillator off
    Off = 0,
    ///1: LSI oscillator on
    On = 1,
}
impl From<LSI1ON> for bool {
    #[inline(always)]
    fn from(variant: LSI1ON) -> Self {
        variant as u8 != 0
    }
}
///Field `LSI1ON` reader - LSI1 oscillator enabled
pub type LSI1ON_R = crate::BitReader<LSI1ON>;
impl LSI1ON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSI1ON {
        match self.bits {
            false => LSI1ON::Off,
            true => LSI1ON::On,
        }
    }
    ///LSI oscillator off
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LSI1ON::Off
    }
    ///LSI oscillator on
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LSI1ON::On
    }
}
///Field `LSI1ON` writer - LSI1 oscillator enabled
pub type LSI1ON_W<'a, REG> = crate::BitWriter<'a, REG, LSI1ON>;
impl<'a, REG> LSI1ON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSI oscillator off
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(LSI1ON::Off)
    }
    ///LSI oscillator on
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(LSI1ON::On)
    }
}
/**LSI1 oscillator ready

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSI1RDY {
    ///0: LSI oscillator not ready
    NotReady = 0,
    ///1: LSI oscillator ready
    Ready = 1,
}
impl From<LSI1RDY> for bool {
    #[inline(always)]
    fn from(variant: LSI1RDY) -> Self {
        variant as u8 != 0
    }
}
///Field `LSI1RDY` reader - LSI1 oscillator ready
pub type LSI1RDY_R = crate::BitReader<LSI1RDY>;
impl LSI1RDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSI1RDY {
        match self.bits {
            false => LSI1RDY::NotReady,
            true => LSI1RDY::Ready,
        }
    }
    ///LSI oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSI1RDY::NotReady
    }
    ///LSI oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSI1RDY::Ready
    }
}
///Field `LSI2ON` reader - LSI2 oscillator enabled
pub use LSI1ON_R as LSI2ON_R;
///Field `LSI2ON` writer - LSI2 oscillator enabled
pub use LSI1ON_W as LSI2ON_W;
///Field `LSI2RDY` reader - LSI2 oscillator ready
pub use LSI1RDY_R as LSI2RDY_R;
///Field `LSI2TRIMEN` reader - LSI2 oscillator trimming enable
pub type LSI2TRIMEN_R = crate::BitReader;
///Field `LSI2TRIMEN` writer - LSI2 oscillator trimming enable
pub type LSI2TRIMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSI2TRIMOK` reader - LSI2 oscillator trim OK
pub type LSI2TRIMOK_R = crate::BitReader;
///Field `LSI2BW` reader - LSI2 oscillator bias configuration
pub type LSI2BW_R = crate::FieldReader;
///Field `LSI2BW` writer - LSI2 oscillator bias configuration
pub type LSI2BW_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**RF system wakeup clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RFWKPSEL {
    ///0: No clock
    NoClock = 0,
    ///1: LSE oscillator clock selected
    Lse = 1,
    ///3: HSE oscillator clock selected
    Hse = 3,
}
impl From<RFWKPSEL> for u8 {
    #[inline(always)]
    fn from(variant: RFWKPSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RFWKPSEL {
    type Ux = u8;
}
impl crate::IsEnum for RFWKPSEL {}
///Field `RFWKPSEL` reader - RF system wakeup clock source selection
pub type RFWKPSEL_R = crate::FieldReader<RFWKPSEL>;
impl RFWKPSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RFWKPSEL> {
        match self.bits {
            0 => Some(RFWKPSEL::NoClock),
            1 => Some(RFWKPSEL::Lse),
            3 => Some(RFWKPSEL::Hse),
            _ => None,
        }
    }
    ///No clock
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == RFWKPSEL::NoClock
    }
    ///LSE oscillator clock selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RFWKPSEL::Lse
    }
    ///HSE oscillator clock selected
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == RFWKPSEL::Hse
    }
}
///Field `RFWKPSEL` writer - RF system wakeup clock source selection
pub type RFWKPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RFWKPSEL>;
impl<'a, REG> RFWKPSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No clock
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(RFWKPSEL::NoClock)
    }
    ///LSE oscillator clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(RFWKPSEL::Lse)
    }
    ///HSE oscillator clock selected
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(RFWKPSEL::Hse)
    }
}
/**Radio system BLE and 802.15.4 reset status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFRSTS {
    ///0: Radio system BLE and 802.15.4 not in reset
    NoReset = 0,
    ///1: Radio system BLE and 802.15.4 under reset
    Reset = 1,
}
impl From<RFRSTS> for bool {
    #[inline(always)]
    fn from(variant: RFRSTS) -> Self {
        variant as u8 != 0
    }
}
///Field `RFRSTS` reader - Radio system BLE and 802.15.4 reset status
pub type RFRSTS_R = crate::BitReader<RFRSTS>;
impl RFRSTS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFRSTS {
        match self.bits {
            false => RFRSTS::NoReset,
            true => RFRSTS::Reset,
        }
    }
    ///Radio system BLE and 802.15.4 not in reset
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == RFRSTS::NoReset
    }
    ///Radio system BLE and 802.15.4 under reset
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RFRSTS::Reset
    }
}
/**Remove reset flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMVF {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset flags reset
    Clear = 1,
}
impl From<RMVF> for bool {
    #[inline(always)]
    fn from(variant: RMVF) -> Self {
        variant as u8 != 0
    }
}
///Field `RMVF` reader - Remove reset flag
pub type RMVF_R = crate::BitReader<RMVF>;
impl RMVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RMVF {
        match self.bits {
            false => RMVF::NoEffect,
            true => RMVF::Clear,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RMVF::NoEffect
    }
    ///Reset flags reset
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RMVF::Clear
    }
}
///Field `RMVF` writer - Remove reset flag
pub type RMVF_W<'a, REG> = crate::BitWriter<'a, REG, RMVF>;
impl<'a, REG> RMVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(RMVF::NoEffect)
    }
    ///Reset flags reset
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RMVF::Clear)
    }
}
/**Option byte loader reset flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBLRSTF {
    ///0: No reset occurred
    NoReset = 0,
    ///1: Reset occurred
    Reset = 1,
}
impl From<OBLRSTF> for bool {
    #[inline(always)]
    fn from(variant: OBLRSTF) -> Self {
        variant as u8 != 0
    }
}
///Field `OBLRSTF` reader - Option byte loader reset flag
pub type OBLRSTF_R = crate::BitReader<OBLRSTF>;
impl OBLRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OBLRSTF {
        match self.bits {
            false => OBLRSTF::NoReset,
            true => OBLRSTF::Reset,
        }
    }
    ///No reset occurred
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == OBLRSTF::NoReset
    }
    ///Reset occurred
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OBLRSTF::Reset
    }
}
///Field `PINRSTF` reader - Pin reset flag
pub use OBLRSTF_R as PINRSTF_R;
///Field `BORRSTF` reader - BOR flag
pub use OBLRSTF_R as BORRSTF_R;
///Field `SFTRSTF` reader - Software reset flag
pub use OBLRSTF_R as SFTRSTF_R;
///Field `IWDGRSTF` reader - Independent window watchdog reset flag
pub use OBLRSTF_R as IWDGRSTF_R;
///Field `WWDGRSTF` reader - Window watchdog reset flag
pub use OBLRSTF_R as WWDGRSTF_R;
///Field `LPWRRSTF` reader - Low-power reset flag
pub use OBLRSTF_R as LPWRRSTF_R;
impl R {
    ///Bit 0 - LSI1 oscillator enabled
    #[inline(always)]
    pub fn lsi1on(&self) -> LSI1ON_R {
        LSI1ON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSI1 oscillator ready
    #[inline(always)]
    pub fn lsi1rdy(&self) -> LSI1RDY_R {
        LSI1RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LSI2 oscillator enabled
    #[inline(always)]
    pub fn lsi2on(&self) -> LSI2ON_R {
        LSI2ON_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LSI2 oscillator ready
    #[inline(always)]
    pub fn lsi2rdy(&self) -> LSI2RDY_R {
        LSI2RDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - LSI2 oscillator trimming enable
    #[inline(always)]
    pub fn lsi2trimen(&self) -> LSI2TRIMEN_R {
        LSI2TRIMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - LSI2 oscillator trim OK
    #[inline(always)]
    pub fn lsi2trimok(&self) -> LSI2TRIMOK_R {
        LSI2TRIMOK_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:11 - LSI2 oscillator bias configuration
    #[inline(always)]
    pub fn lsi2bw(&self) -> LSI2BW_R {
        LSI2BW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 14:15 - RF system wakeup clock source selection
    #[inline(always)]
    pub fn rfwkpsel(&self) -> RFWKPSEL_R {
        RFWKPSEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - Radio system BLE and 802.15.4 reset status
    #[inline(always)]
    pub fn rfrsts(&self) -> RFRSTS_R {
        RFRSTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 23 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - Option byte loader reset flag
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Pin reset flag
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - BOR flag
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Software reset flag
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Independent window watchdog reset flag
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Window watchdog reset flag
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low-power reset flag
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("oblrstf", &self.oblrstf())
            .field("lpwrrstf", &self.lpwrrstf())
            .field("wwdgrstf", &self.wwdgrstf())
            .field("iwdgrstf", &self.iwdgrstf())
            .field("sftrstf", &self.sftrstf())
            .field("borrstf", &self.borrstf())
            .field("pinrstf", &self.pinrstf())
            .field("rmvf", &self.rmvf())
            .field("rfwkpsel", &self.rfwkpsel())
            .field("lsi2bw", &self.lsi2bw())
            .field("lsi2trimok", &self.lsi2trimok())
            .field("lsi2trimen", &self.lsi2trimen())
            .field("lsi1rdy", &self.lsi1rdy())
            .field("lsi2rdy", &self.lsi2rdy())
            .field("lsi1on", &self.lsi1on())
            .field("lsi2on", &self.lsi2on())
            .field("rfrsts", &self.rfrsts())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI1 oscillator enabled
    #[inline(always)]
    pub fn lsi1on(&mut self) -> LSI1ON_W<'_, CSRrs> {
        LSI1ON_W::new(self, 0)
    }
    ///Bit 2 - LSI2 oscillator enabled
    #[inline(always)]
    pub fn lsi2on(&mut self) -> LSI2ON_W<'_, CSRrs> {
        LSI2ON_W::new(self, 2)
    }
    ///Bit 4 - LSI2 oscillator trimming enable
    #[inline(always)]
    pub fn lsi2trimen(&mut self) -> LSI2TRIMEN_W<'_, CSRrs> {
        LSI2TRIMEN_W::new(self, 4)
    }
    ///Bits 8:11 - LSI2 oscillator bias configuration
    #[inline(always)]
    pub fn lsi2bw(&mut self) -> LSI2BW_W<'_, CSRrs> {
        LSI2BW_W::new(self, 8)
    }
    ///Bits 14:15 - RF system wakeup clock source selection
    #[inline(always)]
    pub fn rfwkpsel(&mut self) -> RFWKPSEL_W<'_, CSRrs> {
        RFWKPSEL_W::new(self, 14)
    }
    ///Bit 23 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W<'_, CSRrs> {
        RMVF_W::new(self, 23)
    }
}
/**CSR

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR to value 0x0c00_0000
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
