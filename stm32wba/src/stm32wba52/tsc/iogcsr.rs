///Register `IOGCSR` reader
pub type R = crate::R<IOGCSRrs>;
///Register `IOGCSR` writer
pub type W = crate::W<IOGCSRrs>;
/**Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.

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
///Field `GE(1-6)` reader - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
pub type GE_R = crate::BitReader<G1E>;
impl GE_R {
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
///Field `GE(1-6)` writer - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
pub type GE_W<'a, REG> = crate::BitWriter<'a, REG, G1E>;
impl<'a, REG> GE_W<'a, REG>
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
/**Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.

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
///Field `GS(1-6)` reader - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
pub type GS_R = crate::BitReader<G1S>;
impl GS_R {
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
impl R {
    ///Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `G1E` field.</div>
    #[inline(always)]
    pub fn ge(&self, n: u8) -> GE_R {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        GE_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn ge_iter(&self) -> impl Iterator<Item = GE_R> + '_ {
        (0..6).map(move |n| GE_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g1e(&self) -> GE_R {
        GE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g2e(&self) -> GE_R {
        GE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g3e(&self) -> GE_R {
        GE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g4e(&self) -> GE_R {
        GE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g5e(&self) -> GE_R {
        GE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g6e(&self) -> GE_R {
        GE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `G1S` field.</div>
    #[inline(always)]
    pub fn gs(&self, n: u8) -> GS_R {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        GS_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
    #[inline(always)]
    pub fn gs_iter(&self) -> impl Iterator<Item = GS_R> + '_ {
        (0..6).map(move |n| GS_R::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    ///Bit 16 - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
    #[inline(always)]
    pub fn g1s(&self) -> GS_R {
        GS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
    #[inline(always)]
    pub fn g2s(&self) -> GS_R {
        GS_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
    #[inline(always)]
    pub fn g3s(&self) -> GS_R {
        GS_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
    #[inline(always)]
    pub fn g4s(&self) -> GS_R {
        GS_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
    #[inline(always)]
    pub fn g5s(&self) -> GS_R {
        GS_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
    #[inline(always)]
    pub fn g6s(&self) -> GS_R {
        GS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOGCSR")
            .field("g1e", &self.g1e())
            .field("g2e", &self.g2e())
            .field("g3e", &self.g3e())
            .field("g4e", &self.g4e())
            .field("g5e", &self.g5e())
            .field("g6e", &self.g6e())
            .field("g1s", &self.g1s())
            .field("g2s", &self.g2s())
            .field("g3s", &self.g3s())
            .field("g4s", &self.g4s())
            .field("g5s", &self.g5s())
            .field("g6s", &self.g6s())
            .finish()
    }
}
impl W {
    ///Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `G1E` field.</div>
    #[inline(always)]
    pub fn ge(&mut self, n: u8) -> GE_W<IOGCSRrs> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        GE_W::new(self, n)
    }
    ///Bit 0 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g1e(&mut self) -> GE_W<IOGCSRrs> {
        GE_W::new(self, 0)
    }
    ///Bit 1 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g2e(&mut self) -> GE_W<IOGCSRrs> {
        GE_W::new(self, 1)
    }
    ///Bit 2 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g3e(&mut self) -> GE_W<IOGCSRrs> {
        GE_W::new(self, 2)
    }
    ///Bit 3 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g4e(&mut self) -> GE_W<IOGCSRrs> {
        GE_W::new(self, 3)
    }
    ///Bit 4 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g5e(&mut self) -> GE_W<IOGCSRrs> {
        GE_W::new(self, 4)
    }
    ///Bit 5 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g6e(&mut self) -> GE_W<IOGCSRrs> {
        GE_W::new(self, 5)
    }
}
/**TSC I/O group control status register

You can [`read`](crate::Reg::read) this register and get [`iogcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iogcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#TSC:IOGCSR)*/
pub struct IOGCSRrs;
impl crate::RegisterSpec for IOGCSRrs {
    type Ux = u32;
}
///`read()` method returns [`iogcsr::R`](R) reader structure
impl crate::Readable for IOGCSRrs {}
///`write(|w| ..)` method takes [`iogcsr::W`](W) writer structure
impl crate::Writable for IOGCSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IOGCSR to value 0
impl crate::Resettable for IOGCSRrs {}
