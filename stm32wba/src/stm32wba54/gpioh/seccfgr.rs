///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
/**I/O pin of port H secure bit enable 3 This bit is written by software to enabled the security I/O port pin.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECURE_PIN {
    ///0: The I/O pin is non-secure
    NonSecure = 0,
    ///1: The I/O pin is secure
    Secure = 1,
}
impl From<SECURE_PIN> for bool {
    #[inline(always)]
    fn from(variant: SECURE_PIN) -> Self {
        variant as u8 != 0
    }
}
///Field `SEC3` writer - I/O pin of port H secure bit enable 3 This bit is written by software to enabled the security I/O port pin.
pub type SEC3_W<'a, REG> = crate::BitWriter<'a, REG, SECURE_PIN>;
impl<'a, REG> SEC3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The I/O pin is non-secure
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(SECURE_PIN::NonSecure)
    }
    ///The I/O pin is secure
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(SECURE_PIN::Secure)
    }
}
impl core::fmt::Debug for crate::generic::Reg<SECCFGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 3 - I/O pin of port H secure bit enable 3 This bit is written by software to enabled the security I/O port pin.
    #[inline(always)]
    pub fn sec3(&mut self) -> SEC3_W<'_, SECCFGRrs> {
        SEC3_W::new(self, 3)
    }
}
/**GPIO port H secure configuration register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#GPIOH:SECCFGR)*/
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR to value 0xffff
impl crate::Resettable for SECCFGRrs {
    const RESET_VALUE: u32 = 0xffff;
}
