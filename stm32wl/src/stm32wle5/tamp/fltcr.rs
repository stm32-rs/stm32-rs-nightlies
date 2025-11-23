///Register `FLTCR` reader
pub type R = crate::R<FLTCRrs>;
///Register `FLTCR` writer
pub type W = crate::W<FLTCRrs>;
/**TAMPFREQ

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAMPFREQ {
    ///0: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)
    Hz1 = 0,
    ///1: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)
    Hz2 = 1,
    ///2: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)
    Hz4 = 2,
    ///3: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)
    Hz8 = 3,
    ///4: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)
    Hz16 = 4,
    ///5: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)
    Hz32 = 5,
    ///6: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)
    Hz64 = 6,
    ///7: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)
    Hz128 = 7,
}
impl From<TAMPFREQ> for u8 {
    #[inline(always)]
    fn from(variant: TAMPFREQ) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TAMPFREQ {
    type Ux = u8;
}
impl crate::IsEnum for TAMPFREQ {}
///Field `TAMPFREQ` reader - TAMPFREQ
pub type TAMPFREQ_R = crate::FieldReader<TAMPFREQ>;
impl TAMPFREQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMPFREQ {
        match self.bits {
            0 => TAMPFREQ::Hz1,
            1 => TAMPFREQ::Hz2,
            2 => TAMPFREQ::Hz4,
            3 => TAMPFREQ::Hz8,
            4 => TAMPFREQ::Hz16,
            5 => TAMPFREQ::Hz32,
            6 => TAMPFREQ::Hz64,
            7 => TAMPFREQ::Hz128,
            _ => unreachable!(),
        }
    }
    ///RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_hz_1(&self) -> bool {
        *self == TAMPFREQ::Hz1
    }
    ///RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_hz_2(&self) -> bool {
        *self == TAMPFREQ::Hz2
    }
    ///RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_hz_4(&self) -> bool {
        *self == TAMPFREQ::Hz4
    }
    ///RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_hz_8(&self) -> bool {
        *self == TAMPFREQ::Hz8
    }
    ///RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_hz_16(&self) -> bool {
        *self == TAMPFREQ::Hz16
    }
    ///RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_hz_32(&self) -> bool {
        *self == TAMPFREQ::Hz32
    }
    ///RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_hz_64(&self) -> bool {
        *self == TAMPFREQ::Hz64
    }
    ///RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_hz_128(&self) -> bool {
        *self == TAMPFREQ::Hz128
    }
}
///Field `TAMPFREQ` writer - TAMPFREQ
pub type TAMPFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TAMPFREQ, crate::Safe>;
impl<'a, REG> TAMPFREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Hz1)
    }
    ///RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_2(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Hz2)
    }
    ///RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_4(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Hz4)
    }
    ///RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_8(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Hz8)
    }
    ///RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_16(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Hz16)
    }
    ///RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_32(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Hz32)
    }
    ///RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_64(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Hz64)
    }
    ///RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_128(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Hz128)
    }
}
/**TAMPFLT

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAMPFLT {
    ///0: Tamper event is activated on edge of TAMP_INx input transitions to the active level (no internal pull-up on TAMP_INx input)"
    NoFilter = 0,
    ///1: Tamper event is activated after 2 consecutive samples at the active level"
    Filter2 = 1,
    ///2: Tamper event is activated after 4 consecutive samples at the active level"
    Filter4 = 2,
    ///3: Tamper event is activated after 8 consecutive samples at the active level"
    Filter8 = 3,
}
impl From<TAMPFLT> for u8 {
    #[inline(always)]
    fn from(variant: TAMPFLT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TAMPFLT {
    type Ux = u8;
}
impl crate::IsEnum for TAMPFLT {}
///Field `TAMPFLT` reader - TAMPFLT
pub type TAMPFLT_R = crate::FieldReader<TAMPFLT>;
impl TAMPFLT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMPFLT {
        match self.bits {
            0 => TAMPFLT::NoFilter,
            1 => TAMPFLT::Filter2,
            2 => TAMPFLT::Filter4,
            3 => TAMPFLT::Filter8,
            _ => unreachable!(),
        }
    }
    ///Tamper event is activated on edge of TAMP_INx input transitions to the active level (no internal pull-up on TAMP_INx input)"
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == TAMPFLT::NoFilter
    }
    ///Tamper event is activated after 2 consecutive samples at the active level"
    #[inline(always)]
    pub fn is_filter2(&self) -> bool {
        *self == TAMPFLT::Filter2
    }
    ///Tamper event is activated after 4 consecutive samples at the active level"
    #[inline(always)]
    pub fn is_filter4(&self) -> bool {
        *self == TAMPFLT::Filter4
    }
    ///Tamper event is activated after 8 consecutive samples at the active level"
    #[inline(always)]
    pub fn is_filter8(&self) -> bool {
        *self == TAMPFLT::Filter8
    }
}
///Field `TAMPFLT` writer - TAMPFLT
pub type TAMPFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TAMPFLT, crate::Safe>;
impl<'a, REG> TAMPFLT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Tamper event is activated on edge of TAMP_INx input transitions to the active level (no internal pull-up on TAMP_INx input)"
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFLT::NoFilter)
    }
    ///Tamper event is activated after 2 consecutive samples at the active level"
    #[inline(always)]
    pub fn filter2(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFLT::Filter2)
    }
    ///Tamper event is activated after 4 consecutive samples at the active level"
    #[inline(always)]
    pub fn filter4(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFLT::Filter4)
    }
    ///Tamper event is activated after 8 consecutive samples at the active level"
    #[inline(always)]
    pub fn filter8(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFLT::Filter8)
    }
}
/**TAMPPRCH

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAMPPRCH {
    ///0: 1 RTCCLK cycle
    Cycles1 = 0,
    ///1: 2 RTCCLK cycles
    Cycles2 = 1,
    ///2: 4 RTCCLK cycles
    Cycles4 = 2,
    ///3: 8 RTCCLK cycles
    Cycles8 = 3,
}
impl From<TAMPPRCH> for u8 {
    #[inline(always)]
    fn from(variant: TAMPPRCH) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TAMPPRCH {
    type Ux = u8;
}
impl crate::IsEnum for TAMPPRCH {}
///Field `TAMPPRCH` reader - TAMPPRCH
pub type TAMPPRCH_R = crate::FieldReader<TAMPPRCH>;
impl TAMPPRCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMPPRCH {
        match self.bits {
            0 => TAMPPRCH::Cycles1,
            1 => TAMPPRCH::Cycles2,
            2 => TAMPPRCH::Cycles4,
            3 => TAMPPRCH::Cycles8,
            _ => unreachable!(),
        }
    }
    ///1 RTCCLK cycle
    #[inline(always)]
    pub fn is_cycles1(&self) -> bool {
        *self == TAMPPRCH::Cycles1
    }
    ///2 RTCCLK cycles
    #[inline(always)]
    pub fn is_cycles2(&self) -> bool {
        *self == TAMPPRCH::Cycles2
    }
    ///4 RTCCLK cycles
    #[inline(always)]
    pub fn is_cycles4(&self) -> bool {
        *self == TAMPPRCH::Cycles4
    }
    ///8 RTCCLK cycles
    #[inline(always)]
    pub fn is_cycles8(&self) -> bool {
        *self == TAMPPRCH::Cycles8
    }
}
///Field `TAMPPRCH` writer - TAMPPRCH
pub type TAMPPRCH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TAMPPRCH, crate::Safe>;
impl<'a, REG> TAMPPRCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 RTCCLK cycle
    #[inline(always)]
    pub fn cycles1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPRCH::Cycles1)
    }
    ///2 RTCCLK cycles
    #[inline(always)]
    pub fn cycles2(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPRCH::Cycles2)
    }
    ///4 RTCCLK cycles
    #[inline(always)]
    pub fn cycles4(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPRCH::Cycles4)
    }
    ///8 RTCCLK cycles
    #[inline(always)]
    pub fn cycles8(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPRCH::Cycles8)
    }
}
/**TAMPPUDIS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPPUDIS {
    ///0: Precharge TAMP_INx pins before sampling (enable internal pull-up)
    Enabled = 0,
    ///1: Disable precharge of TAMP_INx pins
    Disabled = 1,
}
impl From<TAMPPUDIS> for bool {
    #[inline(always)]
    fn from(variant: TAMPPUDIS) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMPPUDIS` reader - TAMPPUDIS
pub type TAMPPUDIS_R = crate::BitReader<TAMPPUDIS>;
impl TAMPPUDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMPPUDIS {
        match self.bits {
            false => TAMPPUDIS::Enabled,
            true => TAMPPUDIS::Disabled,
        }
    }
    ///Precharge TAMP_INx pins before sampling (enable internal pull-up)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMPPUDIS::Enabled
    }
    ///Disable precharge of TAMP_INx pins
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMPPUDIS::Disabled
    }
}
///Field `TAMPPUDIS` writer - TAMPPUDIS
pub type TAMPPUDIS_W<'a, REG> = crate::BitWriter<'a, REG, TAMPPUDIS>;
impl<'a, REG> TAMPPUDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Precharge TAMP_INx pins before sampling (enable internal pull-up)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPUDIS::Enabled)
    }
    ///Disable precharge of TAMP_INx pins
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPUDIS::Disabled)
    }
}
impl R {
    ///Bits 0:2 - TAMPFREQ
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:4 - TAMPFLT
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - TAMPPRCH
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - TAMPPUDIS
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLTCR")
            .field("tampfreq", &self.tampfreq())
            .field("tampflt", &self.tampflt())
            .field("tampprch", &self.tampprch())
            .field("tamppudis", &self.tamppudis())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - TAMPFREQ
    #[inline(always)]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W<'_, FLTCRrs> {
        TAMPFREQ_W::new(self, 0)
    }
    ///Bits 3:4 - TAMPFLT
    #[inline(always)]
    pub fn tampflt(&mut self) -> TAMPFLT_W<'_, FLTCRrs> {
        TAMPFLT_W::new(self, 3)
    }
    ///Bits 5:6 - TAMPPRCH
    #[inline(always)]
    pub fn tampprch(&mut self) -> TAMPPRCH_W<'_, FLTCRrs> {
        TAMPPRCH_W::new(self, 5)
    }
    ///Bit 7 - TAMPPUDIS
    #[inline(always)]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W<'_, FLTCRrs> {
        TAMPPUDIS_W::new(self, 7)
    }
}
/**TAMP filter control register

You can [`read`](crate::Reg::read) this register and get [`fltcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#TAMP:FLTCR)*/
pub struct FLTCRrs;
impl crate::RegisterSpec for FLTCRrs {
    type Ux = u32;
}
///`read()` method returns [`fltcr::R`](R) reader structure
impl crate::Readable for FLTCRrs {}
///`write(|w| ..)` method takes [`fltcr::W`](W) writer structure
impl crate::Writable for FLTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLTCR to value 0
impl crate::Resettable for FLTCRrs {}
