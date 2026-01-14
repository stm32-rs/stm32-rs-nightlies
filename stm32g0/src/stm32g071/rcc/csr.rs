///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
/**LSI oscillator enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSION {
    ///0: LSI oscillator powered off
    Disabled = 0,
    ///1: LSI oscillator enabled
    Enabled = 1,
}
impl From<LSION> for bool {
    #[inline(always)]
    fn from(variant: LSION) -> Self {
        variant as u8 != 0
    }
}
///Field `LSION` reader - LSI oscillator enable
pub type LSION_R = crate::BitReader<LSION>;
impl LSION_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSION {
        match self.bits {
            false => LSION::Disabled,
            true => LSION::Enabled,
        }
    }
    ///LSI oscillator powered off
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSION::Disabled
    }
    ///LSI oscillator enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSION::Enabled
    }
}
///Field `LSION` writer - LSI oscillator enable
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG, LSION>;
impl<'a, REG> LSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSI oscillator powered off
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::Disabled)
    }
    ///LSI oscillator enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::Enabled)
    }
}
/**LSI oscillator ready

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDY {
    ///0: LSI oscillator not ready
    NotReady = 0,
    ///1: LSI oscillator ready
    Ready = 1,
}
impl From<LSIRDY> for bool {
    #[inline(always)]
    fn from(variant: LSIRDY) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDY` reader - LSI oscillator ready
pub type LSIRDY_R = crate::BitReader<LSIRDY>;
impl LSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDY {
        match self.bits {
            false => LSIRDY::NotReady,
            true => LSIRDY::Ready,
        }
    }
    ///LSI oscillator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSIRDY::NotReady
    }
    ///LSI oscillator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSIRDY::Ready
    }
}
///Field `LSIRDY` writer - LSI oscillator ready
pub type LSIRDY_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDY>;
impl<'a, REG> LSIRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LSI oscillator not ready
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDY::NotReady)
    }
    ///LSI oscillator ready
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDY::Ready)
    }
}
/**Remove reset flags

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMVF {
    ///1: Clear reset flags
    Clear = 1,
}
impl From<RMVF> for bool {
    #[inline(always)]
    fn from(variant: RMVF) -> Self {
        variant as u8 != 0
    }
}
///Field `RMVF` reader - Remove reset flags
pub type RMVF_R = crate::BitReader<RMVF>;
impl RMVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RMVF> {
        match self.bits {
            true => Some(RMVF::Clear),
            _ => None,
        }
    }
    ///Clear reset flags
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RMVF::Clear
    }
}
///Field `RMVF` writer - Remove reset flags
pub type RMVF_W<'a, REG> = crate::BitWriter<'a, REG, RMVF>;
impl<'a, REG> RMVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear reset flags
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
    ///0: This reset type has not occurred
    NoReset = 0,
    ///1: This reset type has occurred
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
    ///This reset type has not occurred
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == OBLRSTF::NoReset
    }
    ///This reset type has occurred
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OBLRSTF::Reset
    }
}
///Field `OBLRSTF` writer - Option byte loader reset flag
pub type OBLRSTF_W<'a, REG> = crate::BitWriter<'a, REG, OBLRSTF>;
impl<'a, REG> OBLRSTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///This reset type has not occurred
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(OBLRSTF::NoReset)
    }
    ///This reset type has occurred
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(OBLRSTF::Reset)
    }
}
///Field `PINRSTF` reader - Pin reset flag
pub use OBLRSTF_R as PINRSTF_R;
///Field `PWRRSTF` reader - BOR or POR/PDR flag
pub use OBLRSTF_R as PWRRSTF_R;
///Field `SFTRSTF` reader - Software reset flag
pub use OBLRSTF_R as SFTRSTF_R;
///Field `IWDGRSTF` reader - Independent window watchdog reset flag
pub use OBLRSTF_R as IWDGRSTF_R;
///Field `WWDGRSTF` reader - Window watchdog reset flag
pub use OBLRSTF_R as WWDGRSTF_R;
///Field `LPWRRSTF` reader - Low-power reset flag
pub use OBLRSTF_R as LPWRRSTF_R;
///Field `PINRSTF` writer - Pin reset flag
pub use OBLRSTF_W as PINRSTF_W;
///Field `PWRRSTF` writer - BOR or POR/PDR flag
pub use OBLRSTF_W as PWRRSTF_W;
///Field `SFTRSTF` writer - Software reset flag
pub use OBLRSTF_W as SFTRSTF_W;
///Field `IWDGRSTF` writer - Independent window watchdog reset flag
pub use OBLRSTF_W as IWDGRSTF_W;
///Field `WWDGRSTF` writer - Window watchdog reset flag
pub use OBLRSTF_W as WWDGRSTF_W;
///Field `LPWRRSTF` writer - Low-power reset flag
pub use OBLRSTF_W as LPWRRSTF_W;
impl R {
    ///Bit 0 - LSI oscillator enable
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSI oscillator ready
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 23 - Remove reset flags
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
    ///Bit 27 - BOR or POR/PDR flag
    #[inline(always)]
    pub fn pwrrstf(&self) -> PWRRSTF_R {
        PWRRSTF_R::new(((self.bits >> 27) & 1) != 0)
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
            .field("lsion", &self.lsion())
            .field("lsirdy", &self.lsirdy())
            .field("rmvf", &self.rmvf())
            .field("oblrstf", &self.oblrstf())
            .field("pinrstf", &self.pinrstf())
            .field("pwrrstf", &self.pwrrstf())
            .field("sftrstf", &self.sftrstf())
            .field("iwdgrstf", &self.iwdgrstf())
            .field("wwdgrstf", &self.wwdgrstf())
            .field("lpwrrstf", &self.lpwrrstf())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI oscillator enable
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W<'_, CSRrs> {
        LSION_W::new(self, 0)
    }
    ///Bit 1 - LSI oscillator ready
    #[inline(always)]
    pub fn lsirdy(&mut self) -> LSIRDY_W<'_, CSRrs> {
        LSIRDY_W::new(self, 1)
    }
    ///Bit 23 - Remove reset flags
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W<'_, CSRrs> {
        RMVF_W::new(self, 23)
    }
    ///Bit 25 - Option byte loader reset flag
    #[inline(always)]
    pub fn oblrstf(&mut self) -> OBLRSTF_W<'_, CSRrs> {
        OBLRSTF_W::new(self, 25)
    }
    ///Bit 26 - Pin reset flag
    #[inline(always)]
    pub fn pinrstf(&mut self) -> PINRSTF_W<'_, CSRrs> {
        PINRSTF_W::new(self, 26)
    }
    ///Bit 27 - BOR or POR/PDR flag
    #[inline(always)]
    pub fn pwrrstf(&mut self) -> PWRRSTF_W<'_, CSRrs> {
        PWRRSTF_W::new(self, 27)
    }
    ///Bit 28 - Software reset flag
    #[inline(always)]
    pub fn sftrstf(&mut self) -> SFTRSTF_W<'_, CSRrs> {
        SFTRSTF_W::new(self, 28)
    }
    ///Bit 29 - Independent window watchdog reset flag
    #[inline(always)]
    pub fn iwdgrstf(&mut self) -> IWDGRSTF_W<'_, CSRrs> {
        IWDGRSTF_W::new(self, 29)
    }
    ///Bit 30 - Window watchdog reset flag
    #[inline(always)]
    pub fn wwdgrstf(&mut self) -> WWDGRSTF_W<'_, CSRrs> {
        WWDGRSTF_W::new(self, 30)
    }
    ///Bit 31 - Low-power reset flag
    #[inline(always)]
    pub fn lpwrrstf(&mut self) -> LPWRRSTF_W<'_, CSRrs> {
        LPWRRSTF_W::new(self, 31)
    }
}
/**Control/status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#RCC:CSR)*/
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
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}
