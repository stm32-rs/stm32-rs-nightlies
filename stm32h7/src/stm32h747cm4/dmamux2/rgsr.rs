#[doc = "Register `RGSR` reader"]
pub type R = crate::R<RGSRrs>;
#[doc = "Field `OF(0-7)` reader - Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register."]
pub type OF_R = crate::BitReader;
impl R {
    #[doc = "Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OF0` field"]
    #[inline(always)]
    pub fn of(&self, n: u8) -> OF_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        OF_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register."]
    #[inline(always)]
    pub fn of_iter(&self) -> impl Iterator<Item = OF_R> + '_ {
        (0..8).map(move |n| OF_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register."]
    #[inline(always)]
    pub fn of0(&self) -> OF_R {
        OF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register."]
    #[inline(always)]
    pub fn of1(&self) -> OF_R {
        OF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register."]
    #[inline(always)]
    pub fn of2(&self) -> OF_R {
        OF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register."]
    #[inline(always)]
    pub fn of3(&self) -> OF_R {
        OF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register."]
    #[inline(always)]
    pub fn of4(&self) -> OF_R {
        OF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register."]
    #[inline(always)]
    pub fn of5(&self) -> OF_R {
        OF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register."]
    #[inline(always)]
    pub fn of6(&self) -> OF_R {
        OF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register."]
    #[inline(always)]
    pub fn of7(&self) -> OF_R {
        OF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "DMAMux - DMA request generator status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RGSRrs;
impl crate::RegisterSpec for RGSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgsr::R`](R) reader structure"]
impl crate::Readable for RGSRrs {}
#[doc = "`reset()` method sets RGSR to value 0"]
impl crate::Resettable for RGSRrs {
    const RESET_VALUE: u32 = 0;
}
