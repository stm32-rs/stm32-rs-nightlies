///Register `C1MISR` reader
pub type R = crate::R<C1MISRrs>;
///Field `MISF(0-31)` reader - masked interrupt(1) semaphore %s status bit
pub type MISF_R = crate::BitReader;
impl R {
    ///masked interrupt(1) semaphore (0-31) status bit
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MISF0` field.</div>
    #[inline(always)]
    pub fn misf(&self, n: u8) -> MISF_R {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        MISF_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///masked interrupt(1) semaphore (0-31) status bit
    #[inline(always)]
    pub fn misf_iter(&self) -> impl Iterator<Item = MISF_R> + '_ {
        (0..32).map(move |n| MISF_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - masked interrupt(1) semaphore 0 status bit
    #[inline(always)]
    pub fn misf0(&self) -> MISF_R {
        MISF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - masked interrupt(1) semaphore 1 status bit
    #[inline(always)]
    pub fn misf1(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - masked interrupt(1) semaphore 2 status bit
    #[inline(always)]
    pub fn misf2(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - masked interrupt(1) semaphore 3 status bit
    #[inline(always)]
    pub fn misf3(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - masked interrupt(1) semaphore 4 status bit
    #[inline(always)]
    pub fn misf4(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - masked interrupt(1) semaphore 5 status bit
    #[inline(always)]
    pub fn misf5(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - masked interrupt(1) semaphore 6 status bit
    #[inline(always)]
    pub fn misf6(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - masked interrupt(1) semaphore 7 status bit
    #[inline(always)]
    pub fn misf7(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - masked interrupt(1) semaphore 8 status bit
    #[inline(always)]
    pub fn misf8(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - masked interrupt(1) semaphore 9 status bit
    #[inline(always)]
    pub fn misf9(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - masked interrupt(1) semaphore 10 status bit
    #[inline(always)]
    pub fn misf10(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - masked interrupt(1) semaphore 11 status bit
    #[inline(always)]
    pub fn misf11(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - masked interrupt(1) semaphore 12 status bit
    #[inline(always)]
    pub fn misf12(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - masked interrupt(1) semaphore 13 status bit
    #[inline(always)]
    pub fn misf13(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - masked interrupt(1) semaphore 14 status bit
    #[inline(always)]
    pub fn misf14(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - masked interrupt(1) semaphore 15 status bit
    #[inline(always)]
    pub fn misf15(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - masked interrupt(1) semaphore 16 status bit
    #[inline(always)]
    pub fn misf16(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - masked interrupt(1) semaphore 17 status bit
    #[inline(always)]
    pub fn misf17(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - masked interrupt(1) semaphore 18 status bit
    #[inline(always)]
    pub fn misf18(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - masked interrupt(1) semaphore 19 status bit
    #[inline(always)]
    pub fn misf19(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - masked interrupt(1) semaphore 20 status bit
    #[inline(always)]
    pub fn misf20(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - masked interrupt(1) semaphore 21 status bit
    #[inline(always)]
    pub fn misf21(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - masked interrupt(1) semaphore 22 status bit
    #[inline(always)]
    pub fn misf22(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - masked interrupt(1) semaphore 23 status bit
    #[inline(always)]
    pub fn misf23(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - masked interrupt(1) semaphore 24 status bit
    #[inline(always)]
    pub fn misf24(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - masked interrupt(1) semaphore 25 status bit
    #[inline(always)]
    pub fn misf25(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - masked interrupt(1) semaphore 26 status bit
    #[inline(always)]
    pub fn misf26(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - masked interrupt(1) semaphore 27 status bit
    #[inline(always)]
    pub fn misf27(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - masked interrupt(1) semaphore 28 status bit
    #[inline(always)]
    pub fn misf28(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - masked interrupt(1) semaphore 29 status bit
    #[inline(always)]
    pub fn misf29(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - masked interrupt(1) semaphore 30 status bit
    #[inline(always)]
    pub fn misf30(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - masked interrupt(1) semaphore 31 status bit
    #[inline(always)]
    pub fn misf31(&self) -> MISF_R {
        MISF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1MISR")
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
            .field("misf16", &self.misf16())
            .field("misf17", &self.misf17())
            .field("misf18", &self.misf18())
            .field("misf19", &self.misf19())
            .field("misf20", &self.misf20())
            .field("misf21", &self.misf21())
            .field("misf22", &self.misf22())
            .field("misf23", &self.misf23())
            .field("misf24", &self.misf24())
            .field("misf25", &self.misf25())
            .field("misf26", &self.misf26())
            .field("misf27", &self.misf27())
            .field("misf28", &self.misf28())
            .field("misf29", &self.misf29())
            .field("misf30", &self.misf30())
            .field("misf31", &self.misf31())
            .finish()
    }
}
/**HSEM Masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`c1misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#HSEM:C1MISR)*/
pub struct C1MISRrs;
impl crate::RegisterSpec for C1MISRrs {
    type Ux = u32;
}
///`read()` method returns [`c1misr::R`](R) reader structure
impl crate::Readable for C1MISRrs {}
///`reset()` method sets C1MISR to value 0
impl crate::Resettable for C1MISRrs {
    const RESET_VALUE: u32 = 0;
}
