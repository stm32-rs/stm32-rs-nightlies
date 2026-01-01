///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
/**I/O pin of port C secure bit enable y These bits are written by software to enabled the security I/O port pin.

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
///Field `SEC13` writer - I/O pin of port C secure bit enable y These bits are written by software to enabled the security I/O port pin.
pub type SEC13_W<'a, REG> = crate::BitWriter<'a, REG, SECURE_PIN>;
impl<'a, REG> SEC13_W<'a, REG>
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
///Field `SEC14` writer - I/O pin of port C secure bit enable y These bits are written by software to enabled the security I/O port pin.
pub use SEC13_W as SEC14_W;
///Field `SEC15` writer - I/O pin of port C secure bit enable y These bits are written by software to enabled the security I/O port pin.
pub use SEC13_W as SEC15_W;
impl core::fmt::Debug for crate::generic::Reg<SECCFGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 13 - I/O pin of port C secure bit enable y These bits are written by software to enabled the security I/O port pin.
    #[inline(always)]
    pub fn sec13(&mut self) -> SEC13_W<'_, SECCFGRrs> {
        SEC13_W::new(self, 13)
    }
    ///Bit 14 - I/O pin of port C secure bit enable y These bits are written by software to enabled the security I/O port pin.
    #[inline(always)]
    pub fn sec14(&mut self) -> SEC14_W<'_, SECCFGRrs> {
        SEC14_W::new(self, 14)
    }
    ///Bit 15 - I/O pin of port C secure bit enable y These bits are written by software to enabled the security I/O port pin.
    #[inline(always)]
    pub fn sec15(&mut self) -> SEC15_W<'_, SECCFGRrs> {
        SEC15_W::new(self, 15)
    }
}
/**GPIO port C secure configuration register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#GPIOC:SECCFGR)*/
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
