///Register `LPMCSR` reader
pub type R = crate::R<LPMCSRrs>;
///Register `LPMCSR` writer
pub type W = crate::W<LPMCSRrs>;
///Field `LPMEN` reader - LPM support enable Device mode This bit is set by the software to enable the LPM support within the USB Device. If this bit is at 0 no LPM transactions are handled.
pub type LPMEN_R = crate::BitReader;
///Field `LPMEN` writer - LPM support enable Device mode This bit is set by the software to enable the LPM support within the USB Device. If this bit is at 0 no LPM transactions are handled.
pub type LPMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
/**LPM token acknowledge enable Device mode: The NYET/ACK is returned only on a successful LPM transaction: No errors in both the EXT token and the LPM token (else ERROR) A valid bLinkState = 0001B (L1) is received (else STALL)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPMACK {
    ///0: The valid LPM Token will be NYET / NYET answer
    Nyet = 0,
    ///1: The valid LPM Token will be ACK / ACK answer
    Ack = 1,
}
impl From<LPMACK> for bool {
    #[inline(always)]
    fn from(variant: LPMACK) -> Self {
        variant as u8 != 0
    }
}
///Field `LPMACK` reader - LPM token acknowledge enable Device mode: The NYET/ACK is returned only on a successful LPM transaction: No errors in both the EXT token and the LPM token (else ERROR) A valid bLinkState = 0001B (L1) is received (else STALL)
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
    ///The valid LPM Token will be NYET / NYET answer
    #[inline(always)]
    pub fn is_nyet(&self) -> bool {
        *self == LPMACK::Nyet
    }
    ///The valid LPM Token will be ACK / ACK answer
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == LPMACK::Ack
    }
}
///Field `LPMACK` writer - LPM token acknowledge enable Device mode: The NYET/ACK is returned only on a successful LPM transaction: No errors in both the EXT token and the LPM token (else ERROR) A valid bLinkState = 0001B (L1) is received (else STALL)
pub type LPMACK_W<'a, REG> = crate::BitWriter<'a, REG, LPMACK>;
impl<'a, REG> LPMACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The valid LPM Token will be NYET / NYET answer
    #[inline(always)]
    pub fn nyet(self) -> &'a mut crate::W<REG> {
        self.variant(LPMACK::Nyet)
    }
    ///The valid LPM Token will be ACK / ACK answer
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(LPMACK::Ack)
    }
}
///Field `REMWAKE` reader - bRemoteWake value Device mode This bit contains the bRemoteWake value received with last ACKed LPM Token
pub type REMWAKE_R = crate::BitReader;
///Field `BESL` reader - BESL value Device mode These bits contain the BESL value received with last ACKed LPM Token
pub type BESL_R = crate::FieldReader;
impl R {
    ///Bit 0 - LPM support enable Device mode This bit is set by the software to enable the LPM support within the USB Device. If this bit is at 0 no LPM transactions are handled.
    #[inline(always)]
    pub fn lpmen(&self) -> LPMEN_R {
        LPMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LPM token acknowledge enable Device mode: The NYET/ACK is returned only on a successful LPM transaction: No errors in both the EXT token and the LPM token (else ERROR) A valid bLinkState = 0001B (L1) is received (else STALL)
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - bRemoteWake value Device mode This bit contains the bRemoteWake value received with last ACKed LPM Token
    #[inline(always)]
    pub fn remwake(&self) -> REMWAKE_R {
        REMWAKE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - BESL value Device mode These bits contain the BESL value received with last ACKed LPM Token
    #[inline(always)]
    pub fn besl(&self) -> BESL_R {
        BESL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPMCSR")
            .field("lpmen", &self.lpmen())
            .field("lpmack", &self.lpmack())
            .field("remwake", &self.remwake())
            .field("besl", &self.besl())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPM support enable Device mode This bit is set by the software to enable the LPM support within the USB Device. If this bit is at 0 no LPM transactions are handled.
    #[inline(always)]
    pub fn lpmen(&mut self) -> LPMEN_W<'_, LPMCSRrs> {
        LPMEN_W::new(self, 0)
    }
    ///Bit 1 - LPM token acknowledge enable Device mode: The NYET/ACK is returned only on a successful LPM transaction: No errors in both the EXT token and the LPM token (else ERROR) A valid bLinkState = 0001B (L1) is received (else STALL)
    #[inline(always)]
    pub fn lpmack(&mut self) -> LPMACK_W<'_, LPMCSRrs> {
        LPMACK_W::new(self, 1)
    }
}
/**LPM control and status register

You can [`read`](crate::Reg::read) this register and get [`lpmcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#USB:LPMCSR)*/
pub struct LPMCSRrs;
impl crate::RegisterSpec for LPMCSRrs {
    type Ux = u32;
}
///`read()` method returns [`lpmcsr::R`](R) reader structure
impl crate::Readable for LPMCSRrs {}
///`write(|w| ..)` method takes [`lpmcsr::W`](W) writer structure
impl crate::Writable for LPMCSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LPMCSR to value 0
impl crate::Resettable for LPMCSRrs {}
