///Register `RGSR` reader
pub type R = crate::R<RGSRrs>;
/**:0\]: Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OF0 {
    ///0: No new trigger event occured on DMA request generator channel x, before the request counter underrun
    NoTrigger = 0,
    ///1: New trigger event occured on DMA request generator channel x, before the request counter underrun
    Trigger = 1,
}
impl From<OF0> for bool {
    #[inline(always)]
    fn from(variant: OF0) -> Self {
        variant as u8 != 0
    }
}
///Field `OF(0-7)` reader - :0\]: Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register.
pub type OF_R = crate::BitReader<OF0>;
impl OF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OF0 {
        match self.bits {
            false => OF0::NoTrigger,
            true => OF0::Trigger,
        }
    }
    ///No new trigger event occured on DMA request generator channel x, before the request counter underrun
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == OF0::NoTrigger
    }
    ///New trigger event occured on DMA request generator channel x, before the request counter underrun
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == OF0::Trigger
    }
}
impl R {
    ///:0\]: Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register.
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OF0` field.</div>
    #[inline(always)]
    pub fn of(&self, n: u8) -> OF_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        OF_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///:0\]: Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register.
    #[inline(always)]
    pub fn of_iter(&self) -> impl Iterator<Item = OF_R> + '_ {
        (0..8).map(move |n| OF_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - :0\]: Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register.
    #[inline(always)]
    pub fn of0(&self) -> OF_R {
        OF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - :0\]: Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register.
    #[inline(always)]
    pub fn of1(&self) -> OF_R {
        OF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - :0\]: Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register.
    #[inline(always)]
    pub fn of2(&self) -> OF_R {
        OF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - :0\]: Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register.
    #[inline(always)]
    pub fn of3(&self) -> OF_R {
        OF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - :0\]: Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register.
    #[inline(always)]
    pub fn of4(&self) -> OF_R {
        OF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - :0\]: Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register.
    #[inline(always)]
    pub fn of5(&self) -> OF_R {
        OF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - :0\]: Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register.
    #[inline(always)]
    pub fn of6(&self) -> OF_R {
        OF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - :0\]: Trigger overrun event flag The flag is set when a new trigger event occurs on DMA request generator channel x, before the request counter underrun (the internal request counter programmed via the GNBREQ field of the DMAMUX_RGxCR register). The flag is cleared by writing 1 to the corresponding COFx bit in the DMAMUX_RGCFR register.
    #[inline(always)]
    pub fn of7(&self) -> OF_R {
        OF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGSR")
            .field("of0", &self.of0())
            .field("of1", &self.of1())
            .field("of2", &self.of2())
            .field("of3", &self.of3())
            .field("of4", &self.of4())
            .field("of5", &self.of5())
            .field("of6", &self.of6())
            .field("of7", &self.of7())
            .finish()
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`rgsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#DMAMUX1:RGSR)*/
pub struct RGSRrs;
impl crate::RegisterSpec for RGSRrs {
    type Ux = u32;
}
///`read()` method returns [`rgsr::R`](R) reader structure
impl crate::Readable for RGSRrs {}
///`reset()` method sets RGSR to value 0
impl crate::Resettable for RGSRrs {}
