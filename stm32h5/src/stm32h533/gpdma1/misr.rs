///Register `MISR` reader
pub type R = crate::R<MISRrs>;
/**masked interrupt status of channel x

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIS0R {
    ///0: No interrupt has occurred on channel
    NoTrigger = 0,
    ///1: An interrupt has occurred on channel
    Trigger = 1,
}
impl From<MIS0R> for bool {
    #[inline(always)]
    fn from(variant: MIS0R) -> Self {
        variant as u8 != 0
    }
}
///Field `MIS(0-7)` reader - masked interrupt status of channel x
pub type MIS_R = crate::BitReader<MIS0R>;
impl MIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MIS0R {
        match self.bits {
            false => MIS0R::NoTrigger,
            true => MIS0R::Trigger,
        }
    }
    ///No interrupt has occurred on channel
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == MIS0R::NoTrigger
    }
    ///An interrupt has occurred on channel
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == MIS0R::Trigger
    }
}
impl R {
    ///masked interrupt status of channel x
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MIS0` field.</div>
    #[inline(always)]
    pub fn mis(&self, n: u8) -> MIS_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        MIS_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///masked interrupt status of channel x
    #[inline(always)]
    pub fn mis_iter(&self) -> impl Iterator<Item = MIS_R> + '_ {
        (0..8).map(move |n| MIS_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis0(&self) -> MIS_R {
        MIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis1(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis2(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis3(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis4(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis5(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis6(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - masked interrupt status of channel x
    #[inline(always)]
    pub fn mis7(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISR")
            .field("mis0", &self.mis0())
            .field("mis1", &self.mis1())
            .field("mis2", &self.mis2())
            .field("mis3", &self.mis3())
            .field("mis4", &self.mis4())
            .field("mis5", &self.mis5())
            .field("mis6", &self.mis6())
            .field("mis7", &self.mis7())
            .finish()
    }
}
/**GPDMA nonsecure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#GPDMA1:MISR)*/
pub struct MISRrs;
impl crate::RegisterSpec for MISRrs {
    type Ux = u32;
}
///`read()` method returns [`misr::R`](R) reader structure
impl crate::Readable for MISRrs {}
///`reset()` method sets MISR to value 0
impl crate::Resettable for MISRrs {}
