///Register `C1ISR` reader
pub type R = crate::R<C1ISRrs>;
/**Interrupt semaphore %s status bit before enable (mask)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF0 {
    ///0: No interrupt pending
    NotPending = 0,
    ///1: Interrupt pending
    Pending = 1,
}
impl From<ISF0> for bool {
    #[inline(always)]
    fn from(variant: ISF0) -> Self {
        variant as u8 != 0
    }
}
///Field `ISF(0-15)` reader - Interrupt semaphore %s status bit before enable (mask)
pub type ISF_R = crate::BitReader<ISF0>;
impl ISF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ISF0 {
        match self.bits {
            false => ISF0::NotPending,
            true => ISF0::Pending,
        }
    }
    ///No interrupt pending
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == ISF0::NotPending
    }
    ///Interrupt pending
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ISF0::Pending
    }
}
impl R {
    ///Interrupt semaphore (0-15) status bit before enable (mask)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ISF0` field.</div>
    #[inline(always)]
    pub fn isf(&self, n: u8) -> ISF_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        ISF_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Interrupt semaphore (0-15) status bit before enable (mask)
    #[inline(always)]
    pub fn isf_iter(&self) -> impl Iterator<Item = ISF_R> + '_ {
        (0..16).map(move |n| ISF_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Interrupt semaphore 0 status bit before enable (mask)
    #[inline(always)]
    pub fn isf0(&self) -> ISF_R {
        ISF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt semaphore 1 status bit before enable (mask)
    #[inline(always)]
    pub fn isf1(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt semaphore 2 status bit before enable (mask)
    #[inline(always)]
    pub fn isf2(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt semaphore 3 status bit before enable (mask)
    #[inline(always)]
    pub fn isf3(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Interrupt semaphore 4 status bit before enable (mask)
    #[inline(always)]
    pub fn isf4(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Interrupt semaphore 5 status bit before enable (mask)
    #[inline(always)]
    pub fn isf5(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Interrupt semaphore 6 status bit before enable (mask)
    #[inline(always)]
    pub fn isf6(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Interrupt semaphore 7 status bit before enable (mask)
    #[inline(always)]
    pub fn isf7(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Interrupt semaphore 8 status bit before enable (mask)
    #[inline(always)]
    pub fn isf8(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Interrupt semaphore 9 status bit before enable (mask)
    #[inline(always)]
    pub fn isf9(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Interrupt semaphore 10 status bit before enable (mask)
    #[inline(always)]
    pub fn isf10(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Interrupt semaphore 11 status bit before enable (mask)
    #[inline(always)]
    pub fn isf11(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Interrupt semaphore 12 status bit before enable (mask)
    #[inline(always)]
    pub fn isf12(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Interrupt semaphore 13 status bit before enable (mask)
    #[inline(always)]
    pub fn isf13(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Interrupt semaphore 14 status bit before enable (mask)
    #[inline(always)]
    pub fn isf14(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Interrupt semaphore 15 status bit before enable (mask)
    #[inline(always)]
    pub fn isf15(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1ISR")
            .field("isf0", &self.isf0())
            .field("isf1", &self.isf1())
            .field("isf2", &self.isf2())
            .field("isf3", &self.isf3())
            .field("isf4", &self.isf4())
            .field("isf5", &self.isf5())
            .field("isf6", &self.isf6())
            .field("isf7", &self.isf7())
            .field("isf8", &self.isf8())
            .field("isf9", &self.isf9())
            .field("isf10", &self.isf10())
            .field("isf11", &self.isf11())
            .field("isf12", &self.isf12())
            .field("isf13", &self.isf13())
            .field("isf14", &self.isf14())
            .field("isf15", &self.isf15())
            .finish()
    }
}
/**HSEM Interrupt status register

You can [`read`](crate::Reg::read) this register and get [`c1isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#HSEM:C1ISR)*/
pub struct C1ISRrs;
impl crate::RegisterSpec for C1ISRrs {
    type Ux = u32;
}
///`read()` method returns [`c1isr::R`](R) reader structure
impl crate::Readable for C1ISRrs {}
///`reset()` method sets C1ISR to value 0
impl crate::Resettable for C1ISRrs {}
