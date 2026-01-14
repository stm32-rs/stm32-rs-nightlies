///Register `LPMCSR` reader
pub type R = crate::R<LPMCSRrs>;
///Register `LPMCSR` writer
pub type W = crate::W<LPMCSRrs>;
/**LPM support enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPMEN {
    ///0: No LPM transactions are handled
    Disabled = 0,
    ///1: Enable the LPM support within the USB device
    Enabled = 1,
}
impl From<LPMEN> for bool {
    #[inline(always)]
    fn from(variant: LPMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `LPMEN` reader - LPM support enable
pub type LPMEN_R = crate::BitReader<LPMEN>;
impl LPMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPMEN {
        match self.bits {
            false => LPMEN::Disabled,
            true => LPMEN::Enabled,
        }
    }
    ///No LPM transactions are handled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPMEN::Disabled
    }
    ///Enable the LPM support within the USB device
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPMEN::Enabled
    }
}
///Field `LPMEN` writer - LPM support enable
pub type LPMEN_W<'a, REG> = crate::BitWriter<'a, REG, LPMEN>;
impl<'a, REG> LPMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No LPM transactions are handled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPMEN::Disabled)
    }
    ///Enable the LPM support within the USB device
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPMEN::Enabled)
    }
}
/**LPM Token acknowledge enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPMACK {
    ///0: The valid LPM Token will be NYET
    Nyet = 0,
    ///1: The valid LPM Token will be ACK
    Ack = 1,
}
impl From<LPMACK> for bool {
    #[inline(always)]
    fn from(variant: LPMACK) -> Self {
        variant as u8 != 0
    }
}
///Field `LPMACK` reader - LPM Token acknowledge enable
pub type LPMACK_R = crate::BitReader<LPMACK>;
impl LPMACK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPMACK {
        match self.bits {
            false => LPMACK::Nyet,
            true => LPMACK::Ack,
        }
    }
    ///The valid LPM Token will be NYET
    #[inline(always)]
    pub fn is_nyet(&self) -> bool {
        *self == LPMACK::Nyet
    }
    ///The valid LPM Token will be ACK
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == LPMACK::Ack
    }
}
///Field `LPMACK` writer - LPM Token acknowledge enable
pub type LPMACK_W<'a, REG> = crate::BitWriter<'a, REG, LPMACK>;
impl<'a, REG> LPMACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The valid LPM Token will be NYET
    #[inline(always)]
    pub fn nyet(self) -> &'a mut crate::W<REG> {
        self.variant(LPMACK::Nyet)
    }
    ///The valid LPM Token will be ACK
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(LPMACK::Ack)
    }
}
///Field `REMWAKE` reader - RemoteWake value
pub type REMWAKE_R = crate::BitReader;
///Field `BESL` reader - BESL value
pub type BESL_R = crate::FieldReader;
impl R {
    ///Bit 0 - LPM support enable
    #[inline(always)]
    pub fn lpmen(&self) -> LPMEN_R {
        LPMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LPM Token acknowledge enable
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - RemoteWake value
    #[inline(always)]
    pub fn remwake(&self) -> REMWAKE_R {
        REMWAKE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - BESL value
    #[inline(always)]
    pub fn besl(&self) -> BESL_R {
        BESL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPMCSR")
            .field("besl", &self.besl())
            .field("remwake", &self.remwake())
            .field("lpmack", &self.lpmack())
            .field("lpmen", &self.lpmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPM support enable
    #[inline(always)]
    pub fn lpmen(&mut self) -> LPMEN_W<'_, LPMCSRrs> {
        LPMEN_W::new(self, 0)
    }
    ///Bit 1 - LPM Token acknowledge enable
    #[inline(always)]
    pub fn lpmack(&mut self) -> LPMACK_W<'_, LPMCSRrs> {
        LPMACK_W::new(self, 1)
    }
}
/**LPM control and status register

You can [`read`](crate::Reg::read) this register and get [`lpmcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#USB:LPMCSR)*/
pub struct LPMCSRrs;
impl crate::RegisterSpec for LPMCSRrs {
    type Ux = u16;
}
///`read()` method returns [`lpmcsr::R`](R) reader structure
impl crate::Readable for LPMCSRrs {}
///`write(|w| ..)` method takes [`lpmcsr::W`](W) writer structure
impl crate::Writable for LPMCSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LPMCSR to value 0
impl crate::Resettable for LPMCSRrs {}
