///Register `BDCR` reader
pub type R = crate::R<BDCRrs>;
///Register `BDCR` writer
pub type W = crate::W<BDCRrs>;
///Field `LSEON` reader - LSE oscillator enable
pub type LSEON_R = crate::BitReader;
///Field `LSEON` writer - LSE oscillator enable
pub type LSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSERDY` reader - LSE oscillator ready
pub type LSERDY_R = crate::BitReader;
///Field `LSEBYP` reader - LSE oscillator bypass
pub type LSEBYP_R = crate::BitReader;
///Field `LSEBYP` writer - LSE oscillator bypass
pub type LSEBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSEDRV` reader - SE oscillator drive capability
pub type LSEDRV_R = crate::FieldReader;
///Field `LSEDRV` writer - SE oscillator drive capability
pub type LSEDRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LSECSSON` reader - LSECSSON
pub type LSECSSON_R = crate::BitReader;
///Field `LSECSSON` writer - LSECSSON
pub type LSECSSON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSECSSD` reader - LSECSSD
pub type LSECSSD_R = crate::BitReader;
/**RTC clock source selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCSEL {
    ///0: No clock
    NoClock = 0,
    ///1: LSE oscillator clock selected
    Lse = 1,
    ///2: LSI oscillator clock selected
    Lsi = 2,
    ///3: HSE oscillator clock divided by 32 selected
    Hse = 3,
}
impl From<RTCSEL> for u8 {
    #[inline(always)]
    fn from(variant: RTCSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTCSEL {
    type Ux = u8;
}
impl crate::IsEnum for RTCSEL {}
///Field `RTCSEL` reader - RTC clock source selection
pub type RTCSEL_R = crate::FieldReader<RTCSEL>;
impl RTCSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTCSEL {
        match self.bits {
            0 => RTCSEL::NoClock,
            1 => RTCSEL::Lse,
            2 => RTCSEL::Lsi,
            3 => RTCSEL::Hse,
            _ => unreachable!(),
        }
    }
    ///No clock
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == RTCSEL::NoClock
    }
    ///LSE oscillator clock selected
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RTCSEL::Lse
    }
    ///LSI oscillator clock selected
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == RTCSEL::Lsi
    }
    ///HSE oscillator clock divided by 32 selected
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == RTCSEL::Hse
    }
}
///Field `RTCSEL` writer - RTC clock source selection
pub type RTCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RTCSEL, crate::Safe>;
impl<'a, REG> RTCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No clock
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::NoClock)
    }
    ///LSE oscillator clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::Lse)
    }
    ///LSI oscillator clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::Lsi)
    }
    ///HSE oscillator clock divided by 32 selected
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::Hse)
    }
}
/**RTC clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCEN {
    ///0: RTC clock disabled
    Disabled = 0,
    ///1: RTC clock enabled
    Enabled = 1,
}
impl From<RTCEN> for bool {
    #[inline(always)]
    fn from(variant: RTCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `RTCEN` reader - RTC clock enable
pub type RTCEN_R = crate::BitReader<RTCEN>;
impl RTCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTCEN {
        match self.bits {
            false => RTCEN::Disabled,
            true => RTCEN::Enabled,
        }
    }
    ///RTC clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCEN::Disabled
    }
    ///RTC clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTCEN::Enabled
    }
}
///Field `RTCEN` writer - RTC clock enable
pub type RTCEN_W<'a, REG> = crate::BitWriter<'a, REG, RTCEN>;
impl<'a, REG> RTCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RTC clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEN::Disabled)
    }
    ///RTC clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEN::Enabled)
    }
}
///Field `BDRST` reader - Backup domain software reset
pub type BDRST_R = crate::BitReader;
///Field `BDRST` writer - Backup domain software reset
pub type BDRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSCOEN` reader - Low speed clock output enable
pub type LSCOEN_R = crate::BitReader;
///Field `LSCOEN` writer - Low speed clock output enable
pub type LSCOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSCOSEL` reader - Low speed clock output selection
pub type LSCOSEL_R = crate::BitReader;
///Field `LSCOSEL` writer - Low speed clock output selection
pub type LSCOSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LSE oscillator enable
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE oscillator ready
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LSE oscillator bypass
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - SE oscillator drive capability
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - LSECSSON
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LSECSSD
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:9 - RTC clock source selection
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 15 - RTC clock enable
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Backup domain software reset
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Low speed clock output enable
    #[inline(always)]
    pub fn lscoen(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Low speed clock output selection
    #[inline(always)]
    pub fn lscosel(&self) -> LSCOSEL_R {
        LSCOSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDCR")
            .field("lscosel", &self.lscosel())
            .field("lscoen", &self.lscoen())
            .field("bdrst", &self.bdrst())
            .field("rtcen", &self.rtcen())
            .field("rtcsel", &self.rtcsel())
            .field("lsecssd", &self.lsecssd())
            .field("lsecsson", &self.lsecsson())
            .field("lsedrv", &self.lsedrv())
            .field("lsebyp", &self.lsebyp())
            .field("lserdy", &self.lserdy())
            .field("lseon", &self.lseon())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSE oscillator enable
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W<'_, BDCRrs> {
        LSEON_W::new(self, 0)
    }
    ///Bit 2 - LSE oscillator bypass
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W<'_, BDCRrs> {
        LSEBYP_W::new(self, 2)
    }
    ///Bits 3:4 - SE oscillator drive capability
    #[inline(always)]
    pub fn lsedrv(&mut self) -> LSEDRV_W<'_, BDCRrs> {
        LSEDRV_W::new(self, 3)
    }
    ///Bit 5 - LSECSSON
    #[inline(always)]
    pub fn lsecsson(&mut self) -> LSECSSON_W<'_, BDCRrs> {
        LSECSSON_W::new(self, 5)
    }
    ///Bits 8:9 - RTC clock source selection
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RTCSEL_W<'_, BDCRrs> {
        RTCSEL_W::new(self, 8)
    }
    ///Bit 15 - RTC clock enable
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W<'_, BDCRrs> {
        RTCEN_W::new(self, 15)
    }
    ///Bit 16 - Backup domain software reset
    #[inline(always)]
    pub fn bdrst(&mut self) -> BDRST_W<'_, BDCRrs> {
        BDRST_W::new(self, 16)
    }
    ///Bit 24 - Low speed clock output enable
    #[inline(always)]
    pub fn lscoen(&mut self) -> LSCOEN_W<'_, BDCRrs> {
        LSCOEN_W::new(self, 24)
    }
    ///Bit 25 - Low speed clock output selection
    #[inline(always)]
    pub fn lscosel(&mut self) -> LSCOSEL_W<'_, BDCRrs> {
        LSCOSEL_W::new(self, 25)
    }
}
/**BDCR

You can [`read`](crate::Reg::read) this register and get [`bdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#RCC:BDCR)*/
pub struct BDCRrs;
impl crate::RegisterSpec for BDCRrs {
    type Ux = u32;
}
///`read()` method returns [`bdcr::R`](R) reader structure
impl crate::Readable for BDCRrs {}
///`write(|w| ..)` method takes [`bdcr::W`](W) writer structure
impl crate::Writable for BDCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BDCR to value 0
impl crate::Resettable for BDCRrs {}
