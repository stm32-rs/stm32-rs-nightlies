///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
/**I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.

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
///Field `SEC0` writer - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
pub type SEC0_W<'a, REG> = crate::BitWriter<'a, REG, SECURE_PIN>;
impl<'a, REG> SEC0_W<'a, REG>
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
///Field `SEC1` writer - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use SEC0_W as SEC1_W;
///Field `SEC2` writer - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use SEC0_W as SEC2_W;
///Field `SEC3` writer - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use SEC0_W as SEC3_W;
///Field `SEC4` writer - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use SEC0_W as SEC4_W;
///Field `SEC5` writer - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use SEC0_W as SEC5_W;
///Field `SEC6` writer - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use SEC0_W as SEC6_W;
///Field `SEC7` writer - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use SEC0_W as SEC7_W;
///Field `SEC8` writer - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use SEC0_W as SEC8_W;
///Field `SEC9` writer - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use SEC0_W as SEC9_W;
///Field `SEC10` writer - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use SEC0_W as SEC10_W;
///Field `SEC11` writer - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use SEC0_W as SEC11_W;
///Field `SEC12` writer - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use SEC0_W as SEC12_W;
///Field `SEC13` writer - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use SEC0_W as SEC13_W;
///Field `SEC14` writer - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use SEC0_W as SEC14_W;
///Field `SEC15` writer - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use SEC0_W as SEC15_W;
impl core::fmt::Debug for crate::generic::Reg<SECCFGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn sec0(&mut self) -> SEC0_W<'_, SECCFGRrs> {
        SEC0_W::new(self, 0)
    }
    ///Bit 1 - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn sec1(&mut self) -> SEC1_W<'_, SECCFGRrs> {
        SEC1_W::new(self, 1)
    }
    ///Bit 2 - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn sec2(&mut self) -> SEC2_W<'_, SECCFGRrs> {
        SEC2_W::new(self, 2)
    }
    ///Bit 3 - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn sec3(&mut self) -> SEC3_W<'_, SECCFGRrs> {
        SEC3_W::new(self, 3)
    }
    ///Bit 4 - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn sec4(&mut self) -> SEC4_W<'_, SECCFGRrs> {
        SEC4_W::new(self, 4)
    }
    ///Bit 5 - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn sec5(&mut self) -> SEC5_W<'_, SECCFGRrs> {
        SEC5_W::new(self, 5)
    }
    ///Bit 6 - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn sec6(&mut self) -> SEC6_W<'_, SECCFGRrs> {
        SEC6_W::new(self, 6)
    }
    ///Bit 7 - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn sec7(&mut self) -> SEC7_W<'_, SECCFGRrs> {
        SEC7_W::new(self, 7)
    }
    ///Bit 8 - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn sec8(&mut self) -> SEC8_W<'_, SECCFGRrs> {
        SEC8_W::new(self, 8)
    }
    ///Bit 9 - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn sec9(&mut self) -> SEC9_W<'_, SECCFGRrs> {
        SEC9_W::new(self, 9)
    }
    ///Bit 10 - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn sec10(&mut self) -> SEC10_W<'_, SECCFGRrs> {
        SEC10_W::new(self, 10)
    }
    ///Bit 11 - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn sec11(&mut self) -> SEC11_W<'_, SECCFGRrs> {
        SEC11_W::new(self, 11)
    }
    ///Bit 12 - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn sec12(&mut self) -> SEC12_W<'_, SECCFGRrs> {
        SEC12_W::new(self, 12)
    }
    ///Bit 13 - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn sec13(&mut self) -> SEC13_W<'_, SECCFGRrs> {
        SEC13_W::new(self, 13)
    }
    ///Bit 14 - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn sec14(&mut self) -> SEC14_W<'_, SECCFGRrs> {
        SEC14_W::new(self, 14)
    }
    ///Bit 15 - I/O pin of port secure bit enable y These bits are written by software to enabled the security I/O port pin. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn sec15(&mut self) -> SEC15_W<'_, SECCFGRrs> {
        SEC15_W::new(self, 15)
    }
}
/**GPIO port B secure configuration register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GPIOB:SECCFGR)*/
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
