///Register `C1ISR` reader
pub type R = crate::R<C1ISRrs>;
///Field `ISF(0-31)` reader - Interrupt(1) semaphore %s status bit
pub type ISF_R = crate::BitReader;
impl R {
    ///Interrupt(1) semaphore (0-31) status bit
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `ISF0` field.</div>
    #[inline(always)]
    pub fn isf(&self, n: u8) -> ISF_R {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        ISF_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Interrupt(1) semaphore (0-31) status bit
    #[inline(always)]
    pub fn isf_iter(&self) -> impl Iterator<Item = ISF_R> + '_ {
        (0..32).map(move |n| ISF_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Interrupt(1) semaphore 0 status bit
    #[inline(always)]
    pub fn isf0(&self) -> ISF_R {
        ISF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt(1) semaphore 1 status bit
    #[inline(always)]
    pub fn isf1(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt(1) semaphore 2 status bit
    #[inline(always)]
    pub fn isf2(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt(1) semaphore 3 status bit
    #[inline(always)]
    pub fn isf3(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Interrupt(1) semaphore 4 status bit
    #[inline(always)]
    pub fn isf4(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Interrupt(1) semaphore 5 status bit
    #[inline(always)]
    pub fn isf5(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Interrupt(1) semaphore 6 status bit
    #[inline(always)]
    pub fn isf6(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Interrupt(1) semaphore 7 status bit
    #[inline(always)]
    pub fn isf7(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Interrupt(1) semaphore 8 status bit
    #[inline(always)]
    pub fn isf8(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Interrupt(1) semaphore 9 status bit
    #[inline(always)]
    pub fn isf9(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Interrupt(1) semaphore 10 status bit
    #[inline(always)]
    pub fn isf10(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Interrupt(1) semaphore 11 status bit
    #[inline(always)]
    pub fn isf11(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Interrupt(1) semaphore 12 status bit
    #[inline(always)]
    pub fn isf12(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Interrupt(1) semaphore 13 status bit
    #[inline(always)]
    pub fn isf13(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Interrupt(1) semaphore 14 status bit
    #[inline(always)]
    pub fn isf14(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Interrupt(1) semaphore 15 status bit
    #[inline(always)]
    pub fn isf15(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Interrupt(1) semaphore 16 status bit
    #[inline(always)]
    pub fn isf16(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Interrupt(1) semaphore 17 status bit
    #[inline(always)]
    pub fn isf17(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Interrupt(1) semaphore 18 status bit
    #[inline(always)]
    pub fn isf18(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Interrupt(1) semaphore 19 status bit
    #[inline(always)]
    pub fn isf19(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Interrupt(1) semaphore 20 status bit
    #[inline(always)]
    pub fn isf20(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Interrupt(1) semaphore 21 status bit
    #[inline(always)]
    pub fn isf21(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Interrupt(1) semaphore 22 status bit
    #[inline(always)]
    pub fn isf22(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Interrupt(1) semaphore 23 status bit
    #[inline(always)]
    pub fn isf23(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Interrupt(1) semaphore 24 status bit
    #[inline(always)]
    pub fn isf24(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Interrupt(1) semaphore 25 status bit
    #[inline(always)]
    pub fn isf25(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Interrupt(1) semaphore 26 status bit
    #[inline(always)]
    pub fn isf26(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Interrupt(1) semaphore 27 status bit
    #[inline(always)]
    pub fn isf27(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Interrupt(1) semaphore 28 status bit
    #[inline(always)]
    pub fn isf28(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Interrupt(1) semaphore 29 status bit
    #[inline(always)]
    pub fn isf29(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Interrupt(1) semaphore 30 status bit
    #[inline(always)]
    pub fn isf30(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Interrupt(1) semaphore 31 status bit
    #[inline(always)]
    pub fn isf31(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 31) & 1) != 0)
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

You can [`read`](crate::Reg::read) this register and get [`c1isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#HSEM:C1ISR)*/
pub struct C1ISRrs;
impl crate::RegisterSpec for C1ISRrs {
    type Ux = u32;
}
///`read()` method returns [`c1isr::R`](R) reader structure
impl crate::Readable for C1ISRrs {}
///`reset()` method sets C1ISR to value 0
impl crate::Resettable for C1ISRrs {
    const RESET_VALUE: u32 = 0;
}