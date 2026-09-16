///Register `IOPSMEN` reader
pub type R = crate::R<IOPSMENrs>;
///Register `IOPSMEN` writer
pub type W = crate::W<IOPSMENrs>;
/**Port A clock enable during Sleep mode bit

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOPASMEN {
    ///0: Port x clock is disabled in Sleep mode
    Disabled = 0,
    ///1: Port x clock is enabled in Sleep mode (if enabled by IOPHEN)
    Enabled = 1,
}
impl From<IOPASMEN> for bool {
    #[inline(always)]
    fn from(variant: IOPASMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `IOPASMEN` reader - Port A clock enable during Sleep mode bit
pub type IOPASMEN_R = crate::BitReader<IOPASMEN>;
impl IOPASMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IOPASMEN {
        match self.bits {
            false => IOPASMEN::Disabled,
            true => IOPASMEN::Enabled,
        }
    }
    ///Port x clock is disabled in Sleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IOPASMEN::Disabled
    }
    ///Port x clock is enabled in Sleep mode (if enabled by IOPHEN)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IOPASMEN::Enabled
    }
}
///Field `IOPASMEN` writer - Port A clock enable during Sleep mode bit
pub type IOPASMEN_W<'a, REG> = crate::BitWriter<'a, REG, IOPASMEN>;
impl<'a, REG> IOPASMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port x clock is disabled in Sleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IOPASMEN::Disabled)
    }
    ///Port x clock is enabled in Sleep mode (if enabled by IOPHEN)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IOPASMEN::Enabled)
    }
}
///Field `IOPBSMEN` reader - Port B clock enable during Sleep mode bit
pub use IOPASMEN_R as IOPBSMEN_R;
///Field `IOPCSMEN` reader - Port C clock enable during Sleep mode bit
pub use IOPASMEN_R as IOPCSMEN_R;
///Field `IOPDSMEN` reader - Port D clock enable during Sleep mode bit
pub use IOPASMEN_R as IOPDSMEN_R;
///Field `IOPESMEN` reader - Port E clock enable during Sleep mode bit
pub use IOPASMEN_R as IOPESMEN_R;
///Field `IOPHSMEN` reader - Port H clock enable during Sleep mode bit
pub use IOPASMEN_R as IOPHSMEN_R;
///Field `IOPBSMEN` writer - Port B clock enable during Sleep mode bit
pub use IOPASMEN_W as IOPBSMEN_W;
///Field `IOPCSMEN` writer - Port C clock enable during Sleep mode bit
pub use IOPASMEN_W as IOPCSMEN_W;
///Field `IOPDSMEN` writer - Port D clock enable during Sleep mode bit
pub use IOPASMEN_W as IOPDSMEN_W;
///Field `IOPESMEN` writer - Port E clock enable during Sleep mode bit
pub use IOPASMEN_W as IOPESMEN_W;
///Field `IOPHSMEN` writer - Port H clock enable during Sleep mode bit
pub use IOPASMEN_W as IOPHSMEN_W;
impl R {
    ///Bit 0 - Port A clock enable during Sleep mode bit
    #[inline(always)]
    pub fn iopasmen(&self) -> IOPASMEN_R {
        IOPASMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port B clock enable during Sleep mode bit
    #[inline(always)]
    pub fn iopbsmen(&self) -> IOPBSMEN_R {
        IOPBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port C clock enable during Sleep mode bit
    #[inline(always)]
    pub fn iopcsmen(&self) -> IOPCSMEN_R {
        IOPCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port D clock enable during Sleep mode bit
    #[inline(always)]
    pub fn iopdsmen(&self) -> IOPDSMEN_R {
        IOPDSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port E clock enable during Sleep mode bit
    #[inline(always)]
    pub fn iopesmen(&self) -> IOPESMEN_R {
        IOPESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Port H clock enable during Sleep mode bit
    #[inline(always)]
    pub fn iophsmen(&self) -> IOPHSMEN_R {
        IOPHSMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOPSMEN")
            .field("iopasmen", &self.iopasmen())
            .field("iophsmen", &self.iophsmen())
            .field("iopdsmen", &self.iopdsmen())
            .field("iopcsmen", &self.iopcsmen())
            .field("iopbsmen", &self.iopbsmen())
            .field("iopesmen", &self.iopesmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port A clock enable during Sleep mode bit
    #[inline(always)]
    pub fn iopasmen(&mut self) -> IOPASMEN_W<'_, IOPSMENrs> {
        IOPASMEN_W::new(self, 0)
    }
    ///Bit 1 - Port B clock enable during Sleep mode bit
    #[inline(always)]
    pub fn iopbsmen(&mut self) -> IOPBSMEN_W<'_, IOPSMENrs> {
        IOPBSMEN_W::new(self, 1)
    }
    ///Bit 2 - Port C clock enable during Sleep mode bit
    #[inline(always)]
    pub fn iopcsmen(&mut self) -> IOPCSMEN_W<'_, IOPSMENrs> {
        IOPCSMEN_W::new(self, 2)
    }
    ///Bit 3 - Port D clock enable during Sleep mode bit
    #[inline(always)]
    pub fn iopdsmen(&mut self) -> IOPDSMEN_W<'_, IOPSMENrs> {
        IOPDSMEN_W::new(self, 3)
    }
    ///Bit 4 - Port E clock enable during Sleep mode bit
    #[inline(always)]
    pub fn iopesmen(&mut self) -> IOPESMEN_W<'_, IOPSMENrs> {
        IOPESMEN_W::new(self, 4)
    }
    ///Bit 7 - Port H clock enable during Sleep mode bit
    #[inline(always)]
    pub fn iophsmen(&mut self) -> IOPHSMEN_W<'_, IOPSMENrs> {
        IOPHSMEN_W::new(self, 7)
    }
}
/**GPIO clock enable in sleep mode register

You can [`read`](crate::Reg::read) this register and get [`iopsmen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopsmen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:IOPSMEN)*/
pub struct IOPSMENrs;
impl crate::RegisterSpec for IOPSMENrs {
    type Ux = u32;
}
///`read()` method returns [`iopsmen::R`](R) reader structure
impl crate::Readable for IOPSMENrs {}
///`write(|w| ..)` method takes [`iopsmen::W`](W) writer structure
impl crate::Writable for IOPSMENrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IOPSMEN to value 0x8f
impl crate::Resettable for IOPSMENrs {
    const RESET_VALUE: u32 = 0x8f;
}
