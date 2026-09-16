///Register `ISR` reader
pub type R = crate::R<ISRrs>;
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
///Field `ISF(0-31)` reader - Interrupt semaphore %s status bit before enable (mask)
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
    ///Interrupt semaphore (0-31) status bit before enable (mask)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ISF0` field.</div>
    #[inline(always)]
    pub fn isf(&self, n: u8) -> ISF_R {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        ISF_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Interrupt semaphore (0-31) status bit before enable (mask)
    #[inline(always)]
    pub fn isf_iter(&self) -> impl Iterator<Item = ISF_R> + '_ {
        (0..32).map(move |n| ISF_R::new(((self.bits >> n) & 1) != 0))
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
    ///Bit 16 - Interrupt semaphore 16 status bit before enable (mask)
    #[inline(always)]
    pub fn isf16(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Interrupt semaphore 17 status bit before enable (mask)
    #[inline(always)]
    pub fn isf17(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Interrupt semaphore 18 status bit before enable (mask)
    #[inline(always)]
    pub fn isf18(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Interrupt semaphore 19 status bit before enable (mask)
    #[inline(always)]
    pub fn isf19(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Interrupt semaphore 20 status bit before enable (mask)
    #[inline(always)]
    pub fn isf20(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Interrupt semaphore 21 status bit before enable (mask)
    #[inline(always)]
    pub fn isf21(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Interrupt semaphore 22 status bit before enable (mask)
    #[inline(always)]
    pub fn isf22(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Interrupt semaphore 23 status bit before enable (mask)
    #[inline(always)]
    pub fn isf23(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Interrupt semaphore 24 status bit before enable (mask)
    #[inline(always)]
    pub fn isf24(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Interrupt semaphore 25 status bit before enable (mask)
    #[inline(always)]
    pub fn isf25(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Interrupt semaphore 26 status bit before enable (mask)
    #[inline(always)]
    pub fn isf26(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Interrupt semaphore 27 status bit before enable (mask)
    #[inline(always)]
    pub fn isf27(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Interrupt semaphore 28 status bit before enable (mask)
    #[inline(always)]
    pub fn isf28(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Interrupt semaphore 29 status bit before enable (mask)
    #[inline(always)]
    pub fn isf29(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Interrupt semaphore 30 status bit before enable (mask)
    #[inline(always)]
    pub fn isf30(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Interrupt semaphore 31 status bit before enable (mask)
    #[inline(always)]
    pub fn isf31(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
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
            .field("isf16", &self.isf16())
            .field("isf17", &self.isf17())
            .field("isf18", &self.isf18())
            .field("isf19", &self.isf19())
            .field("isf20", &self.isf20())
            .field("isf21", &self.isf21())
            .field("isf22", &self.isf22())
            .field("isf23", &self.isf23())
            .field("isf24", &self.isf24())
            .field("isf25", &self.isf25())
            .field("isf26", &self.isf26())
            .field("isf27", &self.isf27())
            .field("isf28", &self.isf28())
            .field("isf29", &self.isf29())
            .field("isf30", &self.isf30())
            .field("isf31", &self.isf31())
            .finish()
    }
}
/**HSEM Interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HSEM:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
