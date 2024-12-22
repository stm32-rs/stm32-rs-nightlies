///Register `IOGCSR` reader
pub type R = crate::R<IOGCSRrs>;
///Register `IOGCSR` writer
pub type W = crate::W<IOGCSRrs>;
/**Analog I/O group x enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G1E {
    ///0: Acquisition on analog I/O group x disabled
    Disabled = 0,
    ///1: Acquisition on analog I/O group x enabled
    Enabled = 1,
}
impl From<G1E> for bool {
    #[inline(always)]
    fn from(variant: G1E) -> Self {
        variant as u8 != 0
    }
}
///Field `G1E` reader - Analog I/O group x enable
pub type G1E_R = crate::BitReader<G1E>;
impl G1E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> G1E {
        match self.bits {
            false => G1E::Disabled,
            true => G1E::Enabled,
        }
    }
    ///Acquisition on analog I/O group x disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == G1E::Disabled
    }
    ///Acquisition on analog I/O group x enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == G1E::Enabled
    }
}
///Field `G1E` writer - Analog I/O group x enable
pub type G1E_W<'a, REG> = crate::BitWriter<'a, REG, G1E>;
impl<'a, REG> G1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Acquisition on analog I/O group x disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(G1E::Disabled)
    }
    ///Acquisition on analog I/O group x enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(G1E::Enabled)
    }
}
///Field `G2E` reader - Analog I/O group x enable
pub use G1E_R as G2E_R;
///Field `G3E` reader - Analog I/O group x enable
pub use G1E_R as G3E_R;
///Field `G4E` reader - Analog I/O group x enable
pub use G1E_R as G4E_R;
///Field `G5E` reader - Analog I/O group x enable
pub use G1E_R as G5E_R;
///Field `G6E` reader - Analog I/O group x enable
pub use G1E_R as G6E_R;
///Field `G7E` reader - Analog I/O group x enable
pub use G1E_R as G7E_R;
///Field `G8E` reader - Analog I/O group x enable
pub use G1E_R as G8E_R;
///Field `G2E` writer - Analog I/O group x enable
pub use G1E_W as G2E_W;
///Field `G3E` writer - Analog I/O group x enable
pub use G1E_W as G3E_W;
///Field `G4E` writer - Analog I/O group x enable
pub use G1E_W as G4E_W;
///Field `G5E` writer - Analog I/O group x enable
pub use G1E_W as G5E_W;
///Field `G6E` writer - Analog I/O group x enable
pub use G1E_W as G6E_W;
///Field `G7E` writer - Analog I/O group x enable
pub use G1E_W as G7E_W;
///Field `G8E` writer - Analog I/O group x enable
pub use G1E_W as G8E_W;
/**Analog I/O group x status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G1S {
    ///0: Acquisition on analog I/O group x is ongoing or not started
    Ongoing = 0,
    ///1: Acquisition on analog I/O group x is complete
    Complete = 1,
}
impl From<G1S> for bool {
    #[inline(always)]
    fn from(variant: G1S) -> Self {
        variant as u8 != 0
    }
}
///Field `G1S` reader - Analog I/O group x status
pub type G1S_R = crate::BitReader<G1S>;
impl G1S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> G1S {
        match self.bits {
            false => G1S::Ongoing,
            true => G1S::Complete,
        }
    }
    ///Acquisition on analog I/O group x is ongoing or not started
    #[inline(always)]
    pub fn is_ongoing(&self) -> bool {
        *self == G1S::Ongoing
    }
    ///Acquisition on analog I/O group x is complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == G1S::Complete
    }
}
///Field `G2S` reader - Analog I/O group x status
pub use G1S_R as G2S_R;
///Field `G3S` reader - Analog I/O group x status
pub use G1S_R as G3S_R;
///Field `G4S` reader - Analog I/O group x status
pub use G1S_R as G4S_R;
///Field `G5S` reader - Analog I/O group x status
pub use G1S_R as G5S_R;
///Field `G6S` reader - Analog I/O group x status
pub use G1S_R as G6S_R;
///Field `G7S` reader - Analog I/O group x status
pub use G1S_R as G7S_R;
///Field `G8S` reader - Analog I/O group x status
pub use G1S_R as G8S_R;
impl R {
    ///Bit 0 - Analog I/O group x enable
    #[inline(always)]
    pub fn g1e(&self) -> G1E_R {
        G1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Analog I/O group x enable
    #[inline(always)]
    pub fn g2e(&self) -> G2E_R {
        G2E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Analog I/O group x enable
    #[inline(always)]
    pub fn g3e(&self) -> G3E_R {
        G3E_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Analog I/O group x enable
    #[inline(always)]
    pub fn g4e(&self) -> G4E_R {
        G4E_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Analog I/O group x enable
    #[inline(always)]
    pub fn g5e(&self) -> G5E_R {
        G5E_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Analog I/O group x enable
    #[inline(always)]
    pub fn g6e(&self) -> G6E_R {
        G6E_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Analog I/O group x enable
    #[inline(always)]
    pub fn g7e(&self) -> G7E_R {
        G7E_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Analog I/O group x enable
    #[inline(always)]
    pub fn g8e(&self) -> G8E_R {
        G8E_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - Analog I/O group x status
    #[inline(always)]
    pub fn g1s(&self) -> G1S_R {
        G1S_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Analog I/O group x status
    #[inline(always)]
    pub fn g2s(&self) -> G2S_R {
        G2S_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Analog I/O group x status
    #[inline(always)]
    pub fn g3s(&self) -> G3S_R {
        G3S_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Analog I/O group x status
    #[inline(always)]
    pub fn g4s(&self) -> G4S_R {
        G4S_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Analog I/O group x status
    #[inline(always)]
    pub fn g5s(&self) -> G5S_R {
        G5S_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Analog I/O group x status
    #[inline(always)]
    pub fn g6s(&self) -> G6S_R {
        G6S_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Analog I/O group x status
    #[inline(always)]
    pub fn g7s(&self) -> G7S_R {
        G7S_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Analog I/O group x status
    #[inline(always)]
    pub fn g8s(&self) -> G8S_R {
        G8S_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOGCSR")
            .field("g1s", &self.g1s())
            .field("g8s", &self.g8s())
            .field("g7s", &self.g7s())
            .field("g6s", &self.g6s())
            .field("g5s", &self.g5s())
            .field("g4s", &self.g4s())
            .field("g3s", &self.g3s())
            .field("g2s", &self.g2s())
            .field("g1e", &self.g1e())
            .field("g8e", &self.g8e())
            .field("g7e", &self.g7e())
            .field("g6e", &self.g6e())
            .field("g5e", &self.g5e())
            .field("g4e", &self.g4e())
            .field("g3e", &self.g3e())
            .field("g2e", &self.g2e())
            .finish()
    }
}
impl W {
    ///Bit 0 - Analog I/O group x enable
    #[inline(always)]
    pub fn g1e(&mut self) -> G1E_W<IOGCSRrs> {
        G1E_W::new(self, 0)
    }
    ///Bit 1 - Analog I/O group x enable
    #[inline(always)]
    pub fn g2e(&mut self) -> G2E_W<IOGCSRrs> {
        G2E_W::new(self, 1)
    }
    ///Bit 2 - Analog I/O group x enable
    #[inline(always)]
    pub fn g3e(&mut self) -> G3E_W<IOGCSRrs> {
        G3E_W::new(self, 2)
    }
    ///Bit 3 - Analog I/O group x enable
    #[inline(always)]
    pub fn g4e(&mut self) -> G4E_W<IOGCSRrs> {
        G4E_W::new(self, 3)
    }
    ///Bit 4 - Analog I/O group x enable
    #[inline(always)]
    pub fn g5e(&mut self) -> G5E_W<IOGCSRrs> {
        G5E_W::new(self, 4)
    }
    ///Bit 5 - Analog I/O group x enable
    #[inline(always)]
    pub fn g6e(&mut self) -> G6E_W<IOGCSRrs> {
        G6E_W::new(self, 5)
    }
    ///Bit 6 - Analog I/O group x enable
    #[inline(always)]
    pub fn g7e(&mut self) -> G7E_W<IOGCSRrs> {
        G7E_W::new(self, 6)
    }
    ///Bit 7 - Analog I/O group x enable
    #[inline(always)]
    pub fn g8e(&mut self) -> G8E_W<IOGCSRrs> {
        G8E_W::new(self, 7)
    }
}
/**I/O group control status register

You can [`read`](crate::Reg::read) this register and get [`iogcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iogcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#TSC:IOGCSR)*/
pub struct IOGCSRrs;
impl crate::RegisterSpec for IOGCSRrs {
    type Ux = u32;
}
///`read()` method returns [`iogcsr::R`](R) reader structure
impl crate::Readable for IOGCSRrs {}
///`write(|w| ..)` method takes [`iogcsr::W`](W) writer structure
impl crate::Writable for IOGCSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IOGCSR to value 0
impl crate::Resettable for IOGCSRrs {
    const RESET_VALUE: u32 = 0;
}
