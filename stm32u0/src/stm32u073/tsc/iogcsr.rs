///Register `IOGCSR` reader
pub type R = crate::R<IOGCSRrs>;
///Register `IOGCSR` writer
pub type W = crate::W<IOGCSRrs>;
///Field `G1E` reader - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
pub type G1E_R = crate::BitReader;
///Field `G1E` writer - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
pub type G1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G2E` reader - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
pub type G2E_R = crate::BitReader;
///Field `G2E` writer - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
pub type G2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G3E` reader - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
pub type G3E_R = crate::BitReader;
///Field `G3E` writer - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
pub type G3E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G4E` reader - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
pub type G4E_R = crate::BitReader;
///Field `G4E` writer - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
pub type G4E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G5E` reader - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
pub type G5E_R = crate::BitReader;
///Field `G5E` writer - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
pub type G5E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G6E` reader - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
pub type G6E_R = crate::BitReader;
///Field `G6E` writer - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
pub type G6E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G7E` reader - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
pub type G7E_R = crate::BitReader;
///Field `G7E` writer - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
pub type G7E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `G1S` reader - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
pub type G1S_R = crate::BitReader;
///Field `G2S` reader - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
pub type G2S_R = crate::BitReader;
///Field `G3S` reader - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
pub type G3S_R = crate::BitReader;
///Field `G4S` reader - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
pub type G4S_R = crate::BitReader;
///Field `G5S` reader - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
pub type G5S_R = crate::BitReader;
///Field `G6S` reader - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
pub type G6S_R = crate::BitReader;
///Field `G7S` reader - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
pub type G7S_R = crate::BitReader;
impl R {
    ///Bit 0 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g1e(&self) -> G1E_R {
        G1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g2e(&self) -> G2E_R {
        G2E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g3e(&self) -> G3E_R {
        G3E_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g4e(&self) -> G4E_R {
        G4E_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g5e(&self) -> G5E_R {
        G5E_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g6e(&self) -> G6E_R {
        G6E_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g7e(&self) -> G7E_R {
        G7E_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 16 - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
    #[inline(always)]
    pub fn g1s(&self) -> G1S_R {
        G1S_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
    #[inline(always)]
    pub fn g2s(&self) -> G2S_R {
        G2S_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
    #[inline(always)]
    pub fn g3s(&self) -> G3S_R {
        G3S_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
    #[inline(always)]
    pub fn g4s(&self) -> G4S_R {
        G4S_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
    #[inline(always)]
    pub fn g5s(&self) -> G5S_R {
        G5S_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
    #[inline(always)]
    pub fn g6s(&self) -> G6S_R {
        G6S_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Analog I/O group x status These bits are set by hardware when the acquisition on the corresponding enabled analog I/O group x is complete. They are cleared by hardware when a new acquisition is started. Note: When a max count error is detected the remaining GxS bits of the enabled analog I/O groups are not set.
    #[inline(always)]
    pub fn g7s(&self) -> G7S_R {
        G7S_R::new(((self.bits >> 22) & 1) != 0)
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
            .field("g7e", &self.g7e())
            .field("g1s", &self.g1s())
            .field("g2s", &self.g2s())
            .field("g3s", &self.g3s())
            .field("g4s", &self.g4s())
            .field("g5s", &self.g5s())
            .field("g6s", &self.g6s())
            .field("g7s", &self.g7s())
            .finish()
    }
}
impl W {
    ///Bit 0 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g1e(&mut self) -> G1E_W<IOGCSRrs> {
        G1E_W::new(self, 0)
    }
    ///Bit 1 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g2e(&mut self) -> G2E_W<IOGCSRrs> {
        G2E_W::new(self, 1)
    }
    ///Bit 2 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g3e(&mut self) -> G3E_W<IOGCSRrs> {
        G3E_W::new(self, 2)
    }
    ///Bit 3 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g4e(&mut self) -> G4E_W<IOGCSRrs> {
        G4E_W::new(self, 3)
    }
    ///Bit 4 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g5e(&mut self) -> G5E_W<IOGCSRrs> {
        G5E_W::new(self, 4)
    }
    ///Bit 5 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g6e(&mut self) -> G6E_W<IOGCSRrs> {
        G6E_W::new(self, 5)
    }
    ///Bit 6 - Analog I/O group x enable These bits are set and cleared by software to enable/disable the acquisition (counter is counting) on the corresponding analog I/O group x.
    #[inline(always)]
    pub fn g7e(&mut self) -> G7E_W<IOGCSRrs> {
        G7E_W::new(self, 6)
    }
}
/**TSC I/O group control status register

You can [`read`](crate::Reg::read) this register and get [`iogcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iogcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TSC:IOGCSR)*/
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
