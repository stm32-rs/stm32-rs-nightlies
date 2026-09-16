///Register `MISR` reader
pub type R = crate::R<MISRrs>;
/**Masked interrupt semaphore %s status bit after enable (mask)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MISF0 {
    ///0: No interrupt pending after masking
    NotPending = 0,
    ///1: Interrupt pending after masking
    Pending = 1,
}
impl From<MISF0> for bool {
    #[inline(always)]
    fn from(variant: MISF0) -> Self {
        variant as u8 != 0
    }
}
///Field `MISF(0-15)` reader - Masked interrupt semaphore %s status bit after enable (mask)
pub type MISF_R = crate::BitReader<MISF0>;
impl MISF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MISF0 {
        match self.bits {
            false => MISF0::NotPending,
            true => MISF0::Pending,
        }
    }
    ///No interrupt pending after masking
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == MISF0::NotPending
    }
    ///Interrupt pending after masking
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == MISF0::Pending
    }
}
impl R {
    ///Masked interrupt semaphore (0-15) status bit after enable (mask)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MISF0` field.</div>
    #[inline(always)]
    pub fn misf(&self, n: u8) -> MISF_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        MISF_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Masked interrupt semaphore (0-15) status bit after enable (mask)
    #[inline(always)]
    pub fn misf_iter(&self) -> impl Iterator<Item = MISF_R> + '_ {
        (0..16).map(move |n| MISF_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Masked interrupt semaphore 0 status bit after enable (mask)
    #[inline(always)]
    pub fn misf0(&self) -> MISF_R {
        MISF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Masked interrupt semaphore 1 status bit after enable (mask)
    #[inline(always)]
    pub fn misf1(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Masked interrupt semaphore 2 status bit after enable (mask)
    #[inline(always)]
    pub fn misf2(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Masked interrupt semaphore 3 status bit after enable (mask)
    #[inline(always)]
    pub fn misf3(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Masked interrupt semaphore 4 status bit after enable (mask)
    #[inline(always)]
    pub fn misf4(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Masked interrupt semaphore 5 status bit after enable (mask)
    #[inline(always)]
    pub fn misf5(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Masked interrupt semaphore 6 status bit after enable (mask)
    #[inline(always)]
    pub fn misf6(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Masked interrupt semaphore 7 status bit after enable (mask)
    #[inline(always)]
    pub fn misf7(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Masked interrupt semaphore 8 status bit after enable (mask)
    #[inline(always)]
    pub fn misf8(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Masked interrupt semaphore 9 status bit after enable (mask)
    #[inline(always)]
    pub fn misf9(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Masked interrupt semaphore 10 status bit after enable (mask)
    #[inline(always)]
    pub fn misf10(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Masked interrupt semaphore 11 status bit after enable (mask)
    #[inline(always)]
    pub fn misf11(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Masked interrupt semaphore 12 status bit after enable (mask)
    #[inline(always)]
    pub fn misf12(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Masked interrupt semaphore 13 status bit after enable (mask)
    #[inline(always)]
    pub fn misf13(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Masked interrupt semaphore 14 status bit after enable (mask)
    #[inline(always)]
    pub fn misf14(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Masked interrupt semaphore 15 status bit after enable (mask)
    #[inline(always)]
    pub fn misf15(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISR")
            .field("misf0", &self.misf0())
            .field("misf1", &self.misf1())
            .field("misf2", &self.misf2())
            .field("misf3", &self.misf3())
            .field("misf4", &self.misf4())
            .field("misf5", &self.misf5())
            .field("misf6", &self.misf6())
            .field("misf7", &self.misf7())
            .field("misf8", &self.misf8())
            .field("misf9", &self.misf9())
            .field("misf10", &self.misf10())
            .field("misf11", &self.misf11())
            .field("misf12", &self.misf12())
            .field("misf13", &self.misf13())
            .field("misf14", &self.misf14())
            .field("misf15", &self.misf15())
            .finish()
    }
}
/**HSEM Masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#HSEM:MISR)*/
pub struct MISRrs;
impl crate::RegisterSpec for MISRrs {
    type Ux = u32;
}
///`read()` method returns [`misr::R`](R) reader structure
impl crate::Readable for MISRrs {}
///`reset()` method sets MISR to value 0
impl crate::Resettable for MISRrs {}
